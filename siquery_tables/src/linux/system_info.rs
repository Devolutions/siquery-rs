use std::process::Command;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use sysconf::raw::{sysconf, SysconfVariable};

use tables::{SystemInfoData,SystemInfoDataIface};
use utils;

struct CpuInfo {
    cpu_brand: String,
    cpu_logical_cores: u32
}

pub struct Reader {}
impl SystemInfoDataIface for Reader {
    fn get_wmi_cpu_info(&self) -> Option<String>  {
        Some(String::from("For windows only"))
    }
    fn get_wmi_system_info(&self) -> Option<String> {
        Some(String::from("For windows only"))
    }
    fn hostname(&self) -> Option<String> {
        let output = Command::new("hostname").output().ok()?;
        let mut hostname = String::from_utf8(output.stdout).ok()?;
        utils::trim_string(&mut hostname);
        Some(hostname)
    }
    fn meminfo(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/proc/meminfo").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
    fn cpuinfo(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/proc/cpuinfo").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
    fn cpu_count(&self) -> u32 {
        let mut cpu_count = sysconf(SysconfVariable::ScNprocessorsConf).unwrap_or(0);
        if cpu_count < 0 {
            cpu_count = 0;
        }
        cpu_count as u32
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

    pub(crate) fn get_specific_ex(reader: &SystemInfoDataIface) -> Vec<SystemInfoData> {
        let mut output : Vec<SystemInfoData> = Vec::new();
        let mut system_info = SystemInfoData::new();
        system_info.computer_name = reader.hostname().unwrap_or_else(|| String::from(""));

        system_info.physical_memory = match reader.meminfo() {
            Some(s) => {
                let n = s.split('\n').find(|line| line.starts_with("MemTotal"))
                         .and_then(|line| line.split(':').last())
                         .and_then(|v| v.trim_left_matches(' ').split(' ').next())
                         .and_then(|v| v.parse::<u64>().ok());
                n.unwrap_or(0) * 1024
            }

            None => 0
        };

        if let Some(cpu_info) = system_info.get_cpu_info(reader) {
            system_info.cpu_brand = cpu_info.cpu_brand;
            system_info.cpu_logical_cores = cpu_info.cpu_logical_cores;
        };
        output.push(system_info);
        output
    }

    fn get_cpu_info(&mut self, reader: &SystemInfoDataIface) -> Option<CpuInfo> {
        let s = reader.cpuinfo()?;
        let model_name = s.split('\n').find(|line| line.starts_with("model name"))
                          .and_then(|line| line.split(':').last())
                          .and_then(|val| Some(val.trim()));
        let model_name = String::from(model_name.unwrap_or(""));

        Some(CpuInfo {cpu_brand: model_name, cpu_logical_cores: reader.cpu_count()})
    }

    pub(crate) fn get_specific() -> Vec<SystemInfoData> {
        let reader: Box<SystemInfoDataIface> = Box::new(Reader{});
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
            Some(String::from("For windows only"))
        }
        fn get_wmi_system_info(&self)-> Option<String> {
            Some(String::from("For windows only"))
        }
        fn hostname(&self) -> Option<String> {
            Some(String::from("galaxy500"))
        }
        fn meminfo(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/meminfo.txt")))
        }
        fn cpuinfo(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/cpuinfo.txt")))
        }
        fn cpu_count(&self) -> u32 {
            4
        }
    }
    #[test]
    fn test_system_info() {
        let reader: Box<SystemInfoDataIface> = Box::new(Test{});
        let system_info = &SystemInfoData::get_specific_ex(reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_brand, "Intel(R) Core(TM) i7-4790 CPU @ 3.60GHz");
        assert_eq!(system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.physical_memory, 16769040384);
    }
}

