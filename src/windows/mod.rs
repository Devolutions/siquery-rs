use std::borrow::Borrow;
use std::process::Command;
use serde_json;
use std::fs::File;
use std::io::Read;
use tables::{
    EtcHosts,
    InterfaceAddress,
    InterfaceDetails,
    LogicalDrive,
    OsVersion,
    SystemInfoData,
};
mod interface_address;
mod interface_details;
mod logical_drive;
mod os_version;
mod system_info;

pub trait SystemReaderInterface {
    fn get_wmi_os_info(&self) -> Option<String>;
    fn get_wmi_cpu_info(&self) -> Option<String>;
    fn get_wmi_computer_info(&self) -> Option<String>;
    fn get_wmi_drives_info(&self) -> Option<String>;
    fn get_wmi_nicconfig(&self) -> Option<String>;
    fn get_wmi_nicconfig_details(&self) -> Option<String>;
    fn get_hosts_file(&self) -> Option<String>;
}

pub struct SystemReader {}

impl SystemReader {
    pub fn new() -> SystemReader {
        SystemReader {}
    }
}

impl SystemReaderInterface for SystemReader {

    fn get_wmi_os_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["os", "get", "Caption,Version,CSName,OSArchitecture", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_cpu_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["cpu", "get", "Name,NumberOfLogicalProcessors", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_computer_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["computersystem", "get", "Caption,TotalPhysicalMemory", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_drives_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "get", "DeviceID,FileSystem,Size,FreeSpace,DriveType",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_nicconfig(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get",
                "IPEnabled,InterfaceIndex,Description,DefaultIPGateway,IPAddress,IPSubnet,DHCPEnabled",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_nicconfig_details(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get", "IPEnabled,InterfaceIndex,MACAddress,MTU", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_hosts_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("c:\\windows\\system32\\drivers\\etc\\hosts").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct SystemInfo {
    system_reader: Box<SystemReaderInterface>,
    pub system_info: SystemInfoData,
    pub os_version: OsVersion,
    pub logical_drives: Vec<LogicalDrive>,
    pub interface_addresses: Vec<InterfaceAddress>,
    pub interface_details: Vec<InterfaceDetails>,
    pub etc_hosts: Vec<EtcHosts>,
}

impl SystemInfo {
    pub fn new(system_reader: Box<SystemReaderInterface>) -> SystemInfo {
        let mut system_info_data = SystemInfoData::new();
        system_info_data.update(system_reader.borrow());

        SystemInfo {
            system_info: system_info_data,
            os_version: OsVersion::new(system_reader.borrow()),
            logical_drives: LogicalDrive::get_drives(system_reader.borrow()),
            interface_addresses: InterfaceAddress::get_interfaces(system_reader.borrow()),
            interface_details: InterfaceDetails::get_interface_details(system_reader.borrow()),
            etc_hosts: EtcHosts::get_hosts(system_reader.borrow()),
            system_reader,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&json!({
            "system_info": self.system_info,
            "os_version" : self.os_version,
            "logical_drives" : self.logical_drives,
            "interface_addresses" : self.interface_addresses,
            "interface_details" : self.interface_details,
            "etc_hosts" : self.etc_hosts
        })).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSystemReader{}

    impl SystemReaderInterface for MockSystemReader {
        fn get_wmi_os_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-osinfo.txt")))
        }

        fn get_wmi_cpu_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-cpuinfo.txt")))
        }

        fn get_wmi_computer_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-computerinfo.txt")))
        }

        fn get_wmi_drives_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-driveinfo.txt")))
        }

        fn get_wmi_nicconfig(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-nicconfig.txt")))
        }

        fn get_wmi_nicconfig_details(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-nicconfig-details.txt")))
        }

        fn get_hosts_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/hosts.txt")))
        }
    }

    #[test]
    fn test_system_info() {
        let system_info = SystemInfo::new(Box::new(MockSystemReader{}));
        //hosts
        assert_eq!(system_info.etc_hosts.get(0).unwrap().address, "127.0.0.1");
        assert_eq!(system_info.etc_hosts.get(0).unwrap().hostnames, "localhost");

        // system_info
        assert_eq!(system_info.system_info.computer_name, "galaxy500");
        assert_eq!(system_info.system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.system_info.cpu_brand, "Intel(R) Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(system_info.system_info.physical_memory, 17043189760);

        // os_version
        assert_eq!(system_info.os_version.platform, "Windows");
        assert_eq!(system_info.os_version.name, "Microsoft Windows 10 Pro");
        assert_eq!(system_info.os_version.version, "10.0.16299");
        assert_eq!(system_info.os_version.major, 10);
        assert_eq!(system_info.os_version.minor, 0);

        // logical_drives
        assert_eq!(system_info.logical_drives.len(), 2);

        let drive = &system_info.logical_drives[0];
        assert_eq!(drive.device_id, "C:");
        assert_eq!(drive.file_system, "NTFS");
        assert_eq!(drive.size, 496869830656);
        assert_eq!(drive.free_space, 55674548224);

        let drive = &system_info.logical_drives[1];
        assert_eq!(drive.device_id, "E:");
        assert_eq!(drive.file_system, "NTFS");
        assert_eq!(drive.size, 501215232);
        assert_eq!(drive.free_space, 469622784);

        // interface_addresses
        assert_eq!(system_info.interface_addresses.len(), 1);

        let interface = &system_info.interface_addresses[0];
        assert_eq!(interface.interface, "1");
        assert_eq!(interface.friendly_name, "Realtek USB GbE Family Controller");
        assert_eq!(interface.address, "192.168.1.172");
        assert_eq!(interface.mask, "255.255.248.0");
        assert_eq!(interface.interface_type, "dhcp");

        // interface_details
        assert_eq!(system_info.interface_details.len(), 1);

        let interface_details = &system_info.interface_details[0];
        assert_eq!(interface_details.interface, "1");
        assert_eq!(interface_details.mac, "A0:CE:C8:05:0D:32");
        assert_eq!(interface_details.enabled, 1);
        assert_eq!(interface_details.mtu, 1400);


    }
}