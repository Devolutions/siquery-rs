use tables::InterfaceAddress;
use nix::ifaddrs;
use libc::*;

#[cfg(not(fuzzing))]    // Since not yet implemented for linux, disable this implementation when fuzzing to allow fuzzing interface_address for windows
impl InterfaceAddress {
    pub(crate) fn new() -> InterfaceAddress {
        InterfaceAddress {
            interface: String::new(),
            address: String::new(),
            mask: String::new(),
            interface_type: String::new(),
            friendly_name: String::new(),
            broadcast: String::new(),
            point_to_point: String::new(),
        }
    }

    pub(crate) fn get_specific() -> Vec<InterfaceAddress> {
        let mut output: Vec<InterfaceAddress> = Vec::new();
        let mut interface_address = InterfaceAddress::new();
        let addrs = ifaddrs::getifaddrs().unwrap();

        for ifaddr in addrs {
            match ifaddr.address {
                Some(address) => {
                    interface_address.interface = ifaddr.interface_name;
                    interface_address.address = address.to_str();

                    if let Some(broadcast) = ifaddr.broadcast {
                        interface_address.broadcast = broadcast.to_str();
                    }

                    if let Some(point_to_point) = ifaddr.destination {
                        if ifaddr.flags.bits() & IFF_BROADCAST == IFF_BROADCAST {
                            interface_address.broadcast = point_to_point.to_str();
                        } else {
                            interface_address.point_to_point = point_to_point.to_str();
                        }

                    }

                    if let Some(netmask) = ifaddr.netmask {
                        interface_address.mask = netmask.to_str();
                        output.push(interface_address);

                    }
                },
                None => {
                    println!("interface {} with unsupported address family",
                             ifaddr.interface_name);
                }
            }
            interface_address = InterfaceAddress::new();
        }
        output
    }
}