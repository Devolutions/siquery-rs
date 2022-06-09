use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{SystemInfoData,SystemInfoDataIface};
use crate::utils;

pub struct Reader {}
impl SystemInfoDataIface for Reader {
    fn hostname(&self) -> Option<String> {
        let output = Command::new("hostname").output().ok()?;
        let mut hostname = String::from_utf8(output.stdout).ok()?;
        utils::trim_string(&mut hostname);
        Some(hostname)
    }
    fn cpuinfo(&self) -> Option<String> {
        // TODO
        Some(String::new())
    }
    fn cpu_count(&self) -> u32 {
        // TODO
        0
    }
    // NA for linux
    fn get_wmi_cpu_info(&self) -> Option<String> {
        Some(String::new())
    }
    fn get_wmi_system_info(&self) -> Option<String> {
        Some(String::new())
    }
    fn meminfo(&self) -> Option<String> {
        Some(String::new())
    }
}

impl SystemInfoData {
    pub(crate) fn new() -> SystemInfoData {
        SystemInfoData {
            computer_name: String::new(),
            cpu_brand: String::new(),
            cpu_logical_cores: 0,
            physical_memory: 0
        }
    }

    pub(crate) fn get_specific_ex(reader: &dyn SystemInfoDataIface) -> Vec<SystemInfoData> {
        let mut output : Vec<SystemInfoData> = Vec::new();
        let mut system_info = SystemInfoData::new();
        system_info.computer_name = reader.hostname().unwrap_or(String::from(""));
        output.push(system_info);
        output
    }

    pub(crate) fn get_specific() -> Vec<SystemInfoData> {
        let reader: Box<dyn SystemInfoDataIface> = Box::new(Reader{});
        let out = SystemInfoData::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Test{}
    impl SystemInfoDataIface for Test {
        fn get_wmi_cpu_info(&self) -> Option<String> {
            Some(String::new())
        }
        fn get_wmi_system_info(&self) -> Option<String> {
            Some(String::new())
        }
        fn hostname(&self) -> Option<String> {
            Some(String::from("galaxy500"))
        }
        fn cpuinfo(&self) -> Option<String> {
            Some(String::new())
        }
        fn cpu_count(&self) -> u32 {
            4
        }
        fn meminfo(&self) -> Option<String> {
            Some(String::new())
        }
    }
    #[test]
    fn test_system_info () {
        let system_reader: Box<dyn SystemInfoDataIface> = Box::new(Test{});
        let system_info = &SystemInfoData::get_specific_ex(system_reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_logical_cores, 0);
    }

}