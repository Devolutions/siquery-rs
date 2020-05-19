use std::process::Command;
use std::borrow::Borrow;

use crate::utils;
use crate::tables::{SystemInfoData,SystemInfoDataIface};

pub struct Reader {}
impl SystemInfoDataIface for Reader {
    fn get_wmi_cpu_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["cpu", "get", "Name,NumberOfLogicalProcessors", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
    fn get_wmi_system_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["computersystem", "get", "Caption,TotalPhysicalMemory", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
    // NA for windows
    fn hostname(&self) -> Option<String> {Some(String::new())}
    fn meminfo(&self) -> Option<String> {Some(String::new())}
    fn cpuinfo(&self) -> Option<String> {Some(String::new())}
    fn cpu_count(&self) -> u32 {0}
}


impl SystemInfoData {
    #[cfg(not(fuzzing))]
    pub fn new() -> SystemInfoData {
        SystemInfoData {
            computer_name: String::new(),
            cpu_brand: String::new(),
            cpu_logical_cores: 0,
            physical_memory: 0
        }
    }

    pub(crate) fn get_specific_ex(reader: &dyn SystemInfoDataIface) -> Vec<SystemInfoData>{
        let mut output : Vec<SystemInfoData> = Vec::new();
        let mut system_info = SystemInfoData::new();
        if let Some(os_info) = reader.get_wmi_cpu_info() {
            let lines = os_info.split('\n');

            for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }
                if v[0].starts_with("Name") {
                    system_info.cpu_brand = String::from(v[1]);
                    utils::trim_string(&mut system_info.cpu_brand);
                } else if v[0].starts_with("NumberOfLogicalProcessors") {
                    let mut n = String::from(v[1]);
                    utils::trim_string(&mut n);
                    system_info.cpu_logical_cores = n.parse::<u32>().unwrap_or(1);
                }
            }
        }

        if let Some(os_info) = reader.get_wmi_system_info() {
        let lines = os_info.split('\n');

        for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }
                if v[0].starts_with("Caption") {
                    system_info.computer_name = String::from(v[1]);
                    utils::trim_string(&mut system_info.computer_name);
                } else if v[0].starts_with("TotalPhysicalMemory") {
                    let mut n = String::from(v[1]);
                    utils::trim_string(&mut n);
                    system_info.physical_memory = n.parse::<i64>().unwrap_or(0);
                }
            }
        }
        output.push(system_info);
        output
    }

    #[cfg(not(fuzzing))]
    pub(crate) fn get_specific() -> Vec<SystemInfoData> {
        let reader: Box<dyn SystemInfoDataIface> = Box::new(Reader{});
        let out = SystemInfoData::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl SystemInfoDataIface for Test {
        fn get_wmi_cpu_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-cpuinfo.txt")))
        }
        fn get_wmi_system_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-system-info.txt")))
        }
        // NA for windows
        fn hostname(&self) -> Option<String> {Some(String::new())}
        fn meminfo(&self) -> Option<String> {Some(String::new())}
        fn cpuinfo(&self) -> Option<String> {Some(String::new())}
        fn cpu_count(&self) -> u32 {0}
    }
    #[test]
    fn test_system_info () {
        let reader: Box<dyn SystemInfoDataIface> = Box::new(Test{});
        let system_info = &SystemInfoData::get_specific_ex(reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.cpu_brand, "Intel(R) Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(system_info.physical_memory, 17043189760);
    }
}


