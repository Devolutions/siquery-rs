use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiNetworkAdapters,WmiNetworkAdaptersIface};
use utils;

pub struct Reader {}
impl WmiNetworkAdaptersIface for Reader {
    fn get_wmi_network_adapters_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiNetworkAdapters {
    pub(crate) fn new() -> WmiNetworkAdapters {
        WmiNetworkAdapters {
            description: String::new(),
            database_path: String::new(),
            dhcp_enabled: String::new(),
            ip_address: Vec::new(),
            ip_enabled: String::new(),
            ip_subnet: Vec::new(),
            mac_address: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiNetworkAdaptersIface) -> Vec<WmiNetworkAdapters> {

        let mut network_adapters: Vec<WmiNetworkAdapters> = Vec::new();
        if let Some(network_adapter_info) = reader.get_wmi_network_adapters_info() {
            let mut network_adapter = WmiNetworkAdapters::new();
            let lines = network_adapter_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if network_adapter.mac_address != "" {
                        network_adapters.push(network_adapter);
                    }
                    network_adapter = WmiNetworkAdapters::new();
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
                    "Description" => {
                        network_adapter.description = v;
                    },
                    "DatabasePath" => {
                        network_adapter.database_path = v;
                    },
                    "DHCPEnabled" => {
                        network_adapter.dhcp_enabled = v.to_lowercase();
                    },
                    "IPAddress" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_address);
                    },
                    "IPEnabled" => {
                        network_adapter.ip_enabled = v.to_lowercase();
                    },
                    "IPSubnet" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_subnet);
                    },
                    "MACAddress" => {
                        network_adapter.mac_address = v;
                    },
                    _ => ()
                }
            }
        }

        network_adapters
    }

    pub(crate) fn get_specific() -> Vec<WmiNetworkAdapters> {
        let reader: Box<WmiNetworkAdaptersIface> = Box::new(Reader{});
        let out = WmiNetworkAdapters::get_specific_ex(reader.borrow());
        out
    }
}

fn add_formatted_entry(s: &mut String, v: &mut Vec<String>){
    s.retain(|c| c != '"');
    s.retain(|c| c != '{');
    s.retain(|c| c != '}');
    s.retain(|c| c != ']');
    s.retain(|c| c != '[');
    s.retain(|c| c != ' ');

    let p: Vec<_> = s.split(',').collect();

    for x in 0..p.len() {
        v.push(String::from(p[x]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiNetworkAdaptersIface for Test {
        fn get_wmi_network_adapters_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-network-adapters.txt")))
        }
    }
    #[test]
    fn test_wmi_network_adapters () {
        let reader: Box<WmiNetworkAdaptersIface> = Box::new(Test{});
        let wmi_network_adapter = &WmiNetworkAdapters::get_specific_ex(reader.borrow())[0];
        assert_eq!(wmi_network_adapter.description, "VMware Virtual Ethernet Adapter for VMnet8");
        assert_eq!(wmi_network_adapter.database_path, "%SystemRoot%\\System32\\drivers\\etc");
        assert_eq!(wmi_network_adapter.dhcp_enabled, "TRUE");
        assert_eq!(wmi_network_adapter.ip_address, vec!["192.168.197.1", "ff80::9999:ffff:9999:f9f9"]);
        assert_eq!(wmi_network_adapter.ip_enabled, "TRUE");
        assert_eq!(wmi_network_adapter.ip_subnet, vec!["255.255.255.0", "64"]);
        assert_eq!(wmi_network_adapter.mac_address, "FF:FF:FF:FF:FF:FF");
    }
}