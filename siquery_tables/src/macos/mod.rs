use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::process::Command;

use serde_json;

mod os_version;
mod system_info;
mod uptime;
mod processes;

use tables::{
    LogicalDrive,
    OsVersion,
    SystemInfoData,
    EtcHosts,
    EtcProtocols,
    EtcServices,
    Uptime,
    ProcessesRow,
    ProcessEnvsRow,
};
use utils;

pub trait SystemReaderInterface {
    fn hostname(&self) -> Option<String>;
    fn cpuinfo(&self) -> Option<String>;
    fn cpu_count(&self) -> u32;
    fn meminfo(&self) -> Option<String>;
    fn system_version(&self) -> Option<String>;
    fn get_hosts_file(&self) -> Option<String>;
    fn get_protocols_file(&self) -> Option<String>;
    fn get_services_file(&self) -> Option<String>;
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

    fn get_protocols_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/protocols").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }

    fn get_services_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/services").ok()?.read_to_string(&mut s).ok()?;
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
    pub etc_hosts: Vec<EtcHosts>,
    pub etc_protocols: Vec<EtcProtocols>,
    pub etc_services: Vec<EtcServices>,
    pub uptime: Result<Uptime, String>,
    pub processes: Vec<ProcessesRow>,
    pub process_envs: Vec<ProcessEnvsRow>,
}

impl SystemInfo {
    pub fn new(system_reader: Box<SystemReaderInterface>) -> SystemInfo {
        let mut system_info_data = SystemInfoData::new();
        system_info_data.update(system_reader.borrow());

        SystemInfo {
            system_info: system_info_data,
            os_version: OsVersion::new(system_reader.borrow()),
            logical_drives: LogicalDrive::new(),
            etc_hosts: EtcHosts::get_hosts(system_reader.borrow()),
            etc_protocols: EtcProtocols::get_protocols(system_reader.borrow()),
            etc_services: EtcServices::get_services(system_reader.borrow()),
            uptime: Uptime::get_uptime(),
            processes: ProcessesRow::gen_processes_table(system_reader.borrow()),
            process_envs: ProcessEnvsRow::gen_process_envs_table(),
            system_reader,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&json!({
            "system_info": self.system_info,
            "os_version" : self.os_version,
            "logical_drives" : self.logical_drives,
            "etc_hosts" : self.etc_hosts,
            "etc_protocols" : self.etc_protocols,
            "etc_services" : self.etc_services,
            "uptime" : self.uptime,
            "processes" : self.processes,
            "process_envs" : self.process_envs,
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

        fn get_protocols_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/protocols.txt")))
        }

        fn get_services_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/services.txt")))
        }
    }

    #[test]
    fn test_system_info() {
        let system_info = SystemInfo::new(Box::new(MockSystemReader{}));

        // checking possible cases for services file
        assert_eq!(system_info.etc_services.get(0).unwrap().name, "echo");
        assert_eq!(system_info.etc_services.get(0).unwrap().port, 7);
        assert_eq!(system_info.etc_services.get(0).unwrap().protocol, "tcp");
        assert_eq!(system_info.etc_services.get(0).unwrap().aliases, "");
        assert_eq!(system_info.etc_services.get(0).unwrap().comment, "");
        assert_eq!(system_info.etc_services.get(2).unwrap().name, "discard");
        assert_eq!(system_info.etc_services.get(2).unwrap().port, 9);
        assert_eq!(system_info.etc_services.get(2).unwrap().protocol, "tcp");
        assert_eq!(system_info.etc_services.get(2).unwrap().aliases, "sink null");
        assert_eq!(system_info.etc_services.get(2).unwrap().comment, "");
        assert_eq!(system_info.etc_services.get(12).unwrap().name, "ftp-data");
        assert_eq!(system_info.etc_services.get(12).unwrap().port, 20);
        assert_eq!(system_info.etc_services.get(12).unwrap().protocol, "tcp");
        assert_eq!(system_info.etc_services.get(12).unwrap().aliases, "");
        assert_eq!(system_info.etc_services.get(12).unwrap().comment, "FTP, data");
        assert_eq!(system_info.etc_services.len(), 15);

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
        assert_eq!(system_info.etc_hosts.get(1).unwrap().address, "255.255.255.255");
        assert_eq!(system_info.etc_hosts.get(1).unwrap().hostnames, "broadcasthost");
        assert_eq!(system_info.etc_hosts.get(2).unwrap().address, "::1");
        assert_eq!(system_info.etc_hosts.get(2).unwrap().hostnames, "localhost");
        assert_eq!(system_info.etc_hosts.get(3).unwrap().address, "127.0.0.1");
        assert_eq!(system_info.etc_hosts.get(3).unwrap().hostnames, "example.com,example");
        assert_eq!(system_info.etc_hosts.get(4).unwrap().address, "127.0.0.1");
        assert_eq!(system_info.etc_hosts.get(4).unwrap().hostnames, "example.net");
        assert_eq!(system_info.etc_hosts.len(), 5);
        //protocols
        assert_eq!(system_info.etc_protocols.get(0).unwrap().name, "ip");
        assert_eq!(system_info.etc_protocols.get(0).unwrap().number, 0);
        assert_eq!(system_info.etc_protocols.get(0).unwrap().alias, "IP");
        assert_eq!(system_info.etc_protocols.get(0).unwrap().comment, "internet protocol, pseudo protocol number");
        assert_eq!(system_info.etc_protocols.get(1).unwrap().name, "icmp");
        assert_eq!(system_info.etc_protocols.get(1).unwrap().number, 1);
        assert_eq!(system_info.etc_protocols.get(1).unwrap().alias, "ICMP");
        assert_eq!(system_info.etc_protocols.get(1).unwrap().comment, "internet control message protocol");
        assert_eq!(system_info.etc_protocols.len(), 3);
        //uptime
        assert_eq!(system_info.uptime.is_ok(), true);
    }
}