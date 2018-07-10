use tables::WmiNetworkAdapters;
use utils;
use windows::SystemReaderInterface;

impl WmiNetworkAdapters {
    pub(crate) fn new() -> WmiNetworkAdapters {
        WmiNetworkAdapters {
            description: String::new(),
            database_path: String::new(),
            dhcp_enabled: String::new(),
            ip_address: Vec::new(),
            ip_subnet: Vec::new(),
            mac_address: String::new(),
        }
    }

    pub(crate) fn get_netwok_adapters_info(system_reader: &SystemReaderInterface) -> Vec<WmiNetworkAdapters> {

        let mut network_adapters: Vec<WmiNetworkAdapters> = Vec::new();

        if let Some(network_adapter_info) = system_reader.get_wmi_network_adapters_info() {
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
                        network_adapter.dhcp_enabled = v;
                    },
                    "IPAddress" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_address);
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