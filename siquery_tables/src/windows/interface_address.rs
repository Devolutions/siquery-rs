use tables::InterfaceAddress;
use utils;
use windows::SystemReaderInterface;


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
        }
    }

    pub fn get_specific(system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
        let mut interfaces: Vec<InterfaceAddress> = Vec::new();

        if let Some(interface_info) = system_reader.get_wmi_nicconfig() {
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
}