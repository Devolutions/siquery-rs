use utils;
use tables::SystemInfoData;
use windows::SystemReaderInterface;


impl SystemInfoData {
    pub fn new() -> SystemInfoData {
        SystemInfoData {
            computer_name: String::new(),
            cpu_brand: String::new(),
            cpu_logical_cores: 0,
            physical_memory: 0
        }
    }

    pub(crate) fn update(&mut self, system_reader: &SystemReaderInterface) {
        if let Some(os_info) = system_reader.get_wmi_cpu_info() {
            let lines = os_info.split('\n');

            for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }
                if v[0].starts_with("Name") {
                    self.cpu_brand = String::from(v[1]);
                    utils::trim_string(&mut self.cpu_brand);
                } else if v[0].starts_with("NumberOfLogicalProcessors") {
                    let mut n = String::from(v[1]);
                    utils::trim_string(&mut n);
                    self.cpu_logical_cores = n.parse::<u32>().unwrap_or(1);
                }
            }
        }

        if let Some(os_info) = system_reader.get_wmi_system_info() {
        let lines = os_info.split('\n');

        for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }
                if v[0].starts_with("Caption") {
                    self.computer_name = String::from(v[1]);
                    utils::trim_string(&mut self.computer_name);
                } else if v[0].starts_with("TotalPhysicalMemory") {
                    let mut n = String::from(v[1]);
                    utils::trim_string(&mut n);
                    self.physical_memory = n.parse::<u64>().unwrap_or(0);
                }
            }
        }
    }
}
