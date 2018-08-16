use tables::InterfaceDetails;
use nix::ifaddrs;
use libc::*;

#[cfg(not(fuzzing))]    // Since not yet implemented for linux, disable this implementation when fuzzing to allow fuzzing interface_address for windows
impl InterfaceDetails {
    pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            mtu: 1500,
            enabled: 1,
        }
    }

    pub fn get_specific() -> Vec<InterfaceDetails> {
        // TODO: implement interface_details table
        let mut interfaces: Vec<InterfaceDetails> = Vec::new();

        if let Some(addrs) = ifaddrs::getifaddrs().ok(){
            for ifaddr in addrs {
                println!("{:?}", ifaddr);
            }
            interfaces.push(InterfaceDetails::new());
        }

        interfaces
    }
}