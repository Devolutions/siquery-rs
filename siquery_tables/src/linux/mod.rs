#[allow(unused_imports)]
// Required in test
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::process::Command;
#[allow(unused_imports)]
// TODO implement to_json
use serde_json;
use sysconf::raw::{sysconf, SysconfVariable};
use uname;

mod logical_drive;
mod interface_address;
mod interface_details;
mod os_version;
mod system_info;
mod uptime;
mod process_open_sockets;
mod processes;

#[allow(unused_imports)]
// Required for test
use tables::*;
use utils;

pub trait SystemReaderInterface {
    fn hostname(&self) -> Option<String>;
    fn cpuinfo(&self) -> Option<String>;
    fn cpu_count(&self) -> u32;
    fn os_release(&self) -> Option<String>;
    fn os_platform(&self) -> Option<String>;
    fn meminfo(&self) -> Option<String>;
}

pub struct SystemReader {
    uname_info: Option<uname::Info>,
}

impl SystemReader {
    pub fn new() -> SystemReader {
        SystemReader {
            uname_info: uname::uname().ok()
        }
    }
}

impl SystemReaderInterface for SystemReader {


    fn cpuinfo(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/proc/cpuinfo").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }

    fn os_release(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/os-release").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }

    fn cpu_count(&self) -> u32 {
        let mut cpu_count = sysconf(SysconfVariable::ScNprocessorsConf).unwrap_or(0);
        if cpu_count < 0 {
            cpu_count = 0;
        }
        cpu_count as u32
    }

    fn os_platform(&self) -> Option<String> {
        match self.uname_info {
            Some(ref info) => Some(info.sysname.clone()),
            None => None
        }
    }

    fn meminfo(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/proc/meminfo").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }


}

pub struct EtcHostsReader {}
impl EtcHostsIface for EtcHostsReader {
    fn get_hosts_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/hosts").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcProtocolsReader {}
impl EtcProtocolsIface for EtcProtocolsReader {
    fn get_protocols_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/protocols").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcServicesReader {}
impl EtcServicesIface for EtcServicesReader {
    fn get_services_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/services").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
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
            Some(String::from(include_str!("../../test_data/cpuinfo.txt")))
        }

        fn cpu_count(&self) -> u32 {
            4
        }

        fn os_release(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/os-release.txt")))
        }

        fn os_platform(&self) -> Option<String> {
            Some(String::from("Linux"))
        }

        fn meminfo(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/meminfo.txt")))
        }

    }

    #[test]
    fn test_system_info() {
        let system_reader: Box<SystemReaderInterface> = Box::new(MockSystemReader{});


        // system_info
        let system_info = &SystemInfoData::get_specific(system_reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_brand, "Intel(R) Core(TM) i7-4790 CPU @ 3.60GHz");
        assert_eq!(system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.physical_memory, 16769040384);

        //os_version
        let os_version = &OsVersion::get_specific(system_reader.borrow())[0];
        assert_eq!(os_version.platform, "Linux");
        assert_eq!(os_version.name, "Ubuntu");
        assert_eq!(os_version.version, "17.10");
        assert_eq!(os_version.major, 17);
        assert_eq!(os_version.minor, 10);
    }
}