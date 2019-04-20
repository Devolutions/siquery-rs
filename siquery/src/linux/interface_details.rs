use crate::tables::InterfaceDetails;
use nix::sys::socket::SockAddr;
use libc::*;
use std::{mem, ptr, ffi::CStr, str, fs::{read_dir, File}, io::prelude::*};

#[repr(C)]
pub struct rtnl_link_stats {
    rx_packets: u32,
    tx_packets: u32,
    rx_bytes: u32,
    tx_bytes: u32,
    rx_errors: u32,
    tx_errors: u32,
    rx_dropped: u32,
    tx_dropped: u32,
    multicast: u32,
    collisions: u32,
    rx_length_errors: u32,
    rx_over_errors: u32,
    rx_crc_errors: u32,
    rx_frame_errors: u32,
    rx_fifo_errors: u32,
    rx_missed_errors: u32,
    tx_aborted_errors: u32,
    tx_carrier_errors: u32,
    tx_fifo_errors: u32,
    tx_heartbeat_errors: u32,
    tx_window_errors: u32,
    rx_compressed: u32,
    tx_compressed: u32,
}

#[cfg(not(fuzzing))]
impl InterfaceDetails {
    pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            type_: 0,
            mtu: 1500,
            metric: 0,
            enabled: 1,
            flags: 0,
            ipackets: 0,
            opackets: 0,
            ibytes: 0,
            obytes: 0,
            ierrors: 0,
            oerrors: 0,
            idrops: 0,
            odrops: 0,
            collisions: 0,
            last_change: -1,
            link_speed: 0,
            pci_slot: String::new(),
        }
    }

    pub fn get_specific() -> Vec<InterfaceDetails> {
        let mut output: Vec<InterfaceDetails> = Vec::new();
        let mut interface_detail;

        let mut addrs: *mut ifaddrs = unsafe { mem::uninitialized() };

        if unsafe { getifaddrs(&mut addrs) } != 0 || addrs == ptr::null_mut() {
            return output
        }

        while addrs != ptr::null_mut() {
            interface_detail = gen_details_from_addr(addrs);
            if interface_detail.ibytes > 0 {
                output.push(interface_detail);
            }
            addrs = unsafe { (*addrs).ifa_next };
        }

        output
    }
}

fn gen_details_from_addr(addrs: *mut ifaddrs) -> InterfaceDetails {
    let mut interface_detail = InterfaceDetails::new();

    unsafe {
        // interface name
        let c_buf: *const c_char = (*addrs).ifa_name;
        let c_str: &CStr = CStr::from_ptr(c_buf);
        let str_slice: &str = c_str.to_str().unwrap();
        interface_detail.interface = str_slice.to_string();

        // mac address
        let address = SockAddr::from_libc_sockaddr((*addrs).ifa_addr);
        interface_detail.mac = address.unwrap().to_string();

        let interface_address_data = (*addrs).ifa_data;
        let ifd = interface_address_data as *const rtnl_link_stats;

        if ifd != ptr::null_mut() {
            interface_detail.ipackets = (*ifd).rx_packets;
            interface_detail.opackets = (*ifd).tx_packets;
            interface_detail.ibytes = (*ifd).rx_bytes;
            interface_detail.obytes = (*ifd).tx_bytes;
            interface_detail.ierrors = (*ifd).rx_errors;
            interface_detail.oerrors = (*ifd).tx_errors;
            interface_detail.idrops = (*ifd).rx_dropped;
            interface_detail.odrops = (*ifd).tx_dropped;
            interface_detail.collisions = (*ifd).collisions;
        }

        let fd = socket(AF_INET, SOCK_DGRAM, 0);

        if fd >= 0 {
            if let Some(ifreq) = IfReq::from_name(&interface_detail.interface.as_str()) {
                if ioctl(fd, SIOCGIFMTU, &ifreq) >= 0 {
                    interface_detail.mtu = ifreq.ifr_mtu() as u32;
                }

                if ioctl(fd, SIOCGIFMETRIC, &ifreq) >= 0 {
                    interface_detail.metric = ifreq.ifr_metric() as u32;
                }

                if ioctl(fd, SIOCGIFHWADDR, &ifreq) >= 0 {
                    interface_detail.type_ = ifreq.ifr_hwaddr().sa_family as u32;
                }

                let net_path = format!("/sys/class/net/{}/", interface_detail.interface.as_str());
                get_link_speed(&net_path, &mut interface_detail);

                let pci_path = "/sys/bus/pci/devices/";
                get_pci_slot(&pci_path, &mut interface_detail);
            }
        }
    }

    interface_detail
}

fn get_link_speed (path : &str, interface_detail: &mut InterfaceDetails) -> Option<bool> {
    let dir_entries = read_dir(path).ok()?;
    let mut rsp = false;

    for interface_info_file_dir in dir_entries {
        let file = interface_info_file_dir.ok()?;
        let info_file_name = file.file_name().into_string().ok()?;

        match info_file_name.as_str() {
            "speed" => {
                let speed_file_path = file.path();
                let f = File::open(speed_file_path);
                let mut buffer = String::new();
                f.ok()?.read_to_string(&mut buffer).ok()?;

                let speed: Vec<_> = buffer.split('\n').collect();
                interface_detail.link_speed = speed[0].parse::<i64>().unwrap_or(0);

                rsp = true;
                break;
            }
            _ => {/*do nothing*/}
        }
    }

    Some(rsp)
}


fn get_pci_slot (path : &str, interface_detail: &mut InterfaceDetails) -> Option<bool> {
    let dir_entries = read_dir(path).ok()?;
    let mut rsp = false;


    for interface_info_file_dir in dir_entries {
        let file = interface_info_file_dir.ok()?;

        let pci_file_path = file.path();
        let pci_entries = read_dir(pci_file_path.clone()).ok()?;

        for pci_entry in pci_entries {
            let f = pci_entry.ok()?;
            let info_file = f.file_name().into_string().ok()?;
            match info_file.as_str() {
                "net" => {
                    let net_file_path = format!("{}/net/{}",
                                                pci_file_path.clone().into_os_string().to_str().unwrap(),
                                                interface_detail.interface);

                    let _ifa = read_dir(net_file_path).ok()?;
                    interface_detail.pci_slot = file.file_name().clone().into_string().ok()?;

                    rsp = true;
                    break;
                }
                _ => {/*do nothing*/}
            }
        }
    }

    Some(rsp)
}



//https://hermanradtke.com/2016/03/17/unions-rust-ffi.html for more info about C unions in Rust FFI
#[repr(C)]
struct IfReq {
    ifr_name: [c_char; IF_NAMESIZE],
    union: IfReqUnion,
}

impl IfReq {

    fn new() -> Self {
        let name : [c_char ; IF_NAMESIZE] = [0 as c_char ; IF_NAMESIZE];
        IfReq {
            ifr_name : name,
            union : IfReqUnion {data: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},
        }
    }

    fn from_name(name : &str) -> Option<IfReq> {
        if name.len() >= IF_NAMESIZE {
            None
        }
            else {
                let mut ifreq : IfReq = IfReq::new();
                for (i,c) in name.as_bytes().iter().enumerate() {
                    ifreq.ifr_name[i] = *c as c_char;
                }
                Some(ifreq)
            }
    }

    pub fn ifr_mtu(&self) -> c_int {
        self.union.as_int()
    }

    pub fn ifr_metric(&self) -> c_int {
        self.union.as_int()
    }

    pub fn ifr_hwaddr(&self) -> sockaddr {
        self.union.as_sockaddr()
    }
}

#[repr(C)]
struct IfReqUnion {
    data: [u8; 24],
}

impl IfReqUnion {
    fn as_sockaddr(&self) -> sockaddr {
        let mut s = sockaddr {
            sa_family: u16::from_be((self.data[0] as u16) << 8 | (self.data[1] as u16)),
            sa_data: [0; 14],
        };

        // basically a memcpy
        for (i, b) in self.data[2..16].iter().enumerate() {
            s.sa_data[i] = *b as i8;
        }

        s
    }

    fn as_int(&self) -> c_int {
        c_int::from_be((self.data[0] as c_int) << 24 |
            (self.data[1] as c_int) << 16 |
            (self.data[2] as c_int) <<  8 |
            (self.data[3] as c_int))
    }

}