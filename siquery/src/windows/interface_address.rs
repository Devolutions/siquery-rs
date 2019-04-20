use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{InterfaceAddress,InterfaceAddressIface};
use crate::utils;

pub struct Reader {}
impl InterfaceAddressIface for Reader {
    fn get_wmi_nicconfig(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get",
                "IPEnabled,InterfaceIndex,Description,DefaultIPGateway,IPAddress,IPSubnet,DHCPEnabled",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

fn split_wmi_array(s: String) -> Vec<String> {
    s.replace("{", "")
     .replace("}", "")
     .replace("\"", "")
     .split(",").map(|v| String::from(v)).collect()
}

impl InterfaceAddress {
    pub fn new() -> InterfaceAddress {
        InterfaceAddress {
            interface: String::new(),
            address: String::new(),
            mask: String::new(),
            interface_type: String::from("manual"),
            friendly_name: String::new(),
            broadcast: String::new(),
            point_to_point: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &InterfaceAddressIface) -> Vec<InterfaceAddress> {
        let mut interfaces: Vec<InterfaceAddress> = Vec::new();

        if let Some(interface_info) = reader.get_wmi_nicconfig() {
            let mut interface = InterfaceAddress::new();
            let lines = interface_info.split('\n');
            let mut ip_enabled = String::new();

            for line in lines {
                if line.len() <= 2 {
                    if ip_enabled == "TRUE" {
                        interfaces.push(interface);
                    }
                    interface = InterfaceAddress::new();
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
                        interface.interface = v;
                    },
                    "IPAddress" => {
                        let values = split_wmi_array(v);
                        if values.len() >= 1 {
                            interface.address = values[0].clone();
                        }
                    },
                    "IPSubnet" => {
                        let values = split_wmi_array(v);
                        if values.len() >= 1 {
                            interface.mask = values[0].clone();
                        }
                    },
                    "DHCPEnabled" => {
                        if v == "TRUE" {
                            interface.interface_type = String::from("dhcp");
                        }
                    },
                    "Description" => {
                        interface.friendly_name = v;
                    },
                    "IPEnabled" => {
                        ip_enabled = v;
                    }
                    _ => ()
                }
            }
        }

        interfaces
    }

    pub fn get_specific() -> Vec<InterfaceAddress> {
        let reader: Box<InterfaceAddressIface> = Box::new(Reader{});
        let out = InterfaceAddress::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl InterfaceAddressIface for Test {
        fn get_wmi_nicconfig(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-nicconfig.txt")))
        }
    }
    #[test]
    fn test_interface_addresses () {
        let reader: Box<InterfaceAddressIface> = Box::new(Test {});
        assert_eq!(InterfaceAddress::get_specific_ex(reader.borrow()).len(), 1);
        let interface = &InterfaceAddress::get_specific_ex(reader.borrow())[0];
        assert_eq!(interface.interface, "1");
        assert_eq!(interface.friendly_name, "Realtek USB GbE Family Controller");
        assert_eq!(interface.address, "192.168.1.172");
        assert_eq!(interface.mask, "255.255.248.0");
        assert_eq!(interface.interface_type, "dhcp");
    }
}