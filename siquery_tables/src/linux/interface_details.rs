use tables::InterfaceDetails;
//use nix::ifaddrs;
use libc::*;
use libc::ifaddrs;
use std::mem;
use std::{ptr,net::Ipv4Addr,net::Ipv6Addr};

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

#[cfg(not(fuzzing))]    // Since not yet implemented for linux, disable this implementation when fuzzing to allow fuzzing interface_address for windows
impl InterfaceDetails {
    pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            mtu: 1500,
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
            last_change: 0,
            link_speed : 0,
            pci_slot: String::new(),
        }
    }

    pub fn get_specific() -> Vec<InterfaceDetails> {
        // TODO: implement interface_details table
        let mut output: Vec<InterfaceDetails> = Vec::new();
        let mut interface_detail = InterfaceDetails::new();
        //let addrs = ifaddrs::getifaddrs().unwrap();

        let mut addrs: *mut ifaddrs = unsafe { mem::uninitialized() };
        unsafe{getifaddrs(&mut addrs)};

        while addrs != ptr::null_mut() {
            //make sur the interface name is not null


            let s = format!("{}",  unsafe{String::from_raw_parts(unsafe{(*addrs).ifa_name} as *mut _, 20, 150).trim_matches('\0')});
                //unsafe{String::from_raw_parts(unsafe{(*addrs).ifa_name} as *mut _, 10, 10)};
            println!("string {:?}", s );

            let mut interface_address_data = unsafe{(*addrs).ifa_data};
            let mut ifd= interface_address_data as *const rtnl_link_stats;

            if ifd != ptr::null_mut(){
                println!("ipackets {:?}", unsafe{(*ifd).rx_packets});
                println!("opackets {:?}", unsafe{(*ifd).tx_packets});
                println!("ibytes {:?}", unsafe{(*ifd).rx_bytes});
                println!("obytes {:?}", unsafe{(*ifd).tx_bytes});
                println!("ierrors {:?}", unsafe{(*ifd).rx_errors});
                println!("oerrors {:?}", unsafe{(*ifd).tx_errors});
                println!("idrops {:?}", unsafe{(*ifd).rx_dropped});
                println!("odrops {:?}", unsafe{(*ifd).tx_dropped});
                println!("collisions {:?}", unsafe{(*ifd).collisions});
            }
            output.push(interface_detail);
            interface_detail = InterfaceDetails::new();
            addrs = unsafe{(*addrs).ifa_next};
        }

        //if let Some(addrs) = ifaddrs::getifaddrs().ok() {
        /*for ifaddr in addrs {
            match ifaddr.address {
                Some(address) => {
                    interface_detail.interface = ifaddr.interface_name;
                    //interface_detail.address = address.to_str();
                },
                None => {
                    println!("interface {} with unsupported address family",
                             ifaddr.interface_name);
                }
            }
            output.push(interface_detail);
            interface_detail = InterfaceDetails::new();
        }*/
        output
    }
}