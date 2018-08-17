use std::process::Command;
use std::borrow::Borrow;

use tables::{InterfaceDetails,InterfaceDetailsIface};
use utils;


pub(crate) struct Reader {}
impl InterfaceDetailsIface for Reader {
    fn get_wmi_nicconfig_details(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get", "IPEnabled,InterfaceIndex,MACAddress,MTU", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

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

    pub(crate) fn get_specific_ex(system_reader: &InterfaceDetailsIface) -> Vec<InterfaceDetails> {
        let mut interfaces: Vec<InterfaceDetails> = Vec::new();

        if let Some(interface_info) = system_reader.get_wmi_nicconfig_details() {
            let mut interface_details = InterfaceDetails::new();
            let lines = interface_info.split('\n');
            let mut ip_enabled = String::from("");

            for line in lines {
                if line.len() <= 2 {
                    if ip_enabled == "TRUE" {
                        interfaces.push(interface_details);
                    }
                    interface_details = InterfaceDetails::new();
                    ip_enabled = String::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "InterfaceIndex" => {
                        interface_details.interface = v;
                    },
                    "MACAddress" => {
                        interface_details.mac = v;
                    },
                    "MTU" => {
                        if let Ok(n) = v.parse::<u32>() {
                            interface_details.mtu = n;
                        }
                    },
                    "IPEnabled" => {
                        ip_enabled = v;
                    }
                    _ => () //todo continue implementation of fields
                }
            }
        }

        interfaces
    }

    pub(crate) fn get_specific() -> Vec<InterfaceDetails> {
        let reader: Box<InterfaceDetailsIface> = Box::new(Reader{});
        let out = InterfaceDetails::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl InterfaceDetailsIface for Test {
        fn get_wmi_nicconfig_details(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-nicconfig-details.txt")))
        }
    }
    #[test]
    fn test_interface_details () {
        let reader: Box<InterfaceDetailsIface> = Box::new(Test{});
        let interface_details = &InterfaceDetails::get_specific_ex(reader.borrow())[0];
        assert_eq!(InterfaceDetails::get_specific_ex(reader.borrow()).len(), 1);
        assert_eq!(interface_details.interface, "1");
        assert_eq!(interface_details.mac, "A0:CE:C8:05:0D:32");
        assert_eq!(interface_details.enabled, 1);
        assert_eq!(interface_details.mtu, 1400);
    }
}