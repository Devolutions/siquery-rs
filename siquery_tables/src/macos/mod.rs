#[allow(unused_imports)]
// Required for test
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::process::Command;
#[allow(unused_imports)]
// TODO implement to_json
use serde_json;

mod logical_drive;
mod os_version;
mod system_info;
mod uptime;
mod processes;
mod process_envs;

#[allow(unused_imports)]
// Required for test
use tables::*;
use utils;

pub trait SystemReaderInterface {
    fn hostname(&self) -> Option<String>;
    fn cpuinfo(&self) -> Option<String>;
    fn cpu_count(&self) -> u32;
    fn meminfo(&self) -> Option<String>;
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



    fn meminfo(&self) -> Option<String> {
        Some(String::new())
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
            Some(String::new())
        }

        fn cpu_count(&self) -> u32 {
            4
        }



        fn meminfo(&self) -> Option<String> {
            Some(String::new())
        }
    }

    fn test_system_info () {
        let system_reader: Box<SystemReaderInterface> = Box::new(MockSystemReader {});

        //system_info
        let system_info = &SystemInfoData::get_specific(system_reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_logical_cores, 0);
    }

}