use tables::InterfaceDetails;
use utils;
use windows::SystemReaderInterface;

impl InterfaceDetails {
     pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            mtu: 1500,
            enabled: 1,
        }
    }

    pub(crate) fn get_interface_details(system_reader: &SystemReaderInterface) -> Vec<InterfaceDetails> {
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
                    _ => ()
                }
            }
        }

        interfaces
    } 
}