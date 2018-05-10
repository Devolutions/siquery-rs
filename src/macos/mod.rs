use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::process::Command;

use serde_json;

mod os_version;
mod system_info;

use tables::{LogicalDrive, OsVersion, SystemInfoData};
use utils;

pub trait SystemReaderInterface {
    fn hostname(&self) -> Option<String>;
    fn cpuinfo(&self) -> Option<String>;
    fn cpu_count(&self) -> u32;
    fn meminfo(&self) -> Option<String>;
    fn system_version(&self) -> Option<String>;
    fn get_hosts_file(&self) -> Option<String>;
}

pub struct SystemReader {}

impl SystemReader {
    pub fn new() -> SystemReader {
        SystemReader {}
    }
}

impl SystemReaderInterface for SystemReader {
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

    fn system_version(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/System/Library/CoreServices/SystemVersion.plist").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }

    fn meminfo(&self) -> Option<String> {
        Some(String::new())
    }

    fn get_hosts_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/hosts").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

struct CpuInfo {
    cpu_brand: String,
    cpu_logical_cores: u32
}

pub struct SystemInfo {
    system_reader: Box<SystemReaderInterface>,
    pub system_info: SystemInfoData,
    pub os_version: OsVersion,
    pub logical_drives: Vec<LogicalDrive>,
    pub etc_hosts: Vec<EtcHosts>
}

impl SystemInfo {
    pub fn new(system_reader: Box<SystemReaderInterface>) -> SystemInfo {
        let mut system_info_data = SystemInfoData::new();
        system_info_data.update(system_reader.borrow());

        SystemInfo {
            system_info: system_info_data,
            os_version: OsVersion::new(system_reader.borrow()),
            logical_drives: Vec::new(),
            etc_hosts: EtcHosts::get_hosts(system_reader.borrow()),
            system_reader,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&json!({
            "system_info": self.system_info,
            "os_version" : self.os_version,
            "logical_drives" : self.logical_drives,
            "etc_hosts" : self.etc_hosts
        })).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSystemReader{}

    impl SystemReaderInterface for MockSystemReader {
        fn hostname(&self) -> Option<String> {
            Some(String::from("galaxy500"))
        }

        fn cpuinfo(&self) -> Option<String> {
            Some(String::new())
        }

        fn cpu_count(&self) -> u32 {
            4
        }

        fn system_version(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/SystemVersion.plist")))
        }

        fn meminfo(&self) -> Option<String> {
            Some(String::new())
        }

        fn get_hosts_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/hosts.txt")))
        }
    }

    #[test]
    fn test_system_info() {
        let system_info = SystemInfo::new(Box::new(MockSystemReader{}));
        assert_eq!(system_info.system_info.computer_name, "galaxy500");
        assert_eq!(system_info.system_info.cpu_logical_cores, 0);
        assert_eq!(system_info.os_version.platform, "MacOS");
        assert_eq!(system_info.os_version.name, "Mac OS X");
        assert_eq!(system_info.os_version.version, "10.13.3");
        assert_eq!(system_info.os_version.major, 10);
        assert_eq!(system_info.os_version.minor, 13);
        //hosts
        assert_eq!(system_info.etc_hosts.get(0).unwrap().address, "127.0.0.1");
        assert_eq!(system_info.etc_hosts.get(0).unwrap().hostnames, "localhost");
    }
}