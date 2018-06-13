use std::borrow::Borrow;
use std::process::Command;
use serde_json;
use std::fs::File;
use std::io::Read;
use tables::{
    EtcServices,
    EtcProtocols,
    EtcHosts,
    InterfaceAddress,
    InterfaceDetails,
    LogicalDrive,
    OsVersion,
    WmiOsVersion,
    SystemInfoData,
    WmiComputerInfo,
    Uptime,
    WmiPrinters,
    WmiServices,
    WmiHotfixes,
    Products,
    WmiShares,
    WmiNetworkAdapters,
    WmiLocalAccounts,
    WmiBios,
    WmiMotherboard,
    WmiProcessor,
    WmiMemory,
    WmiSound,
    WmiVideo,
    WmiMonitors,
};
use std::env;

mod interface_address;
mod interface_details;
mod logical_drive;
mod os_version;
mod system_info;
mod wmi_os_version;
mod wmi_computer_info;
mod uptime;
mod wmi_printers;
mod wmi_services;
mod wmi_hotfixes;
mod products;
mod wmi_shares;
mod wmi_network_adapters;
mod wmi_local_accounts;
mod wmi_bios;
mod wmi_motherboard;
mod wmi_processor;
mod wmi_physical_memory;
mod wmi_sound;
mod wmi_video;
mod wmi_monitors;

pub trait SystemReaderInterface {
    fn get_os_info(&self) -> Option<String>;
    fn get_wmi_system_info(&self) -> Option<String>;
    fn get_wmi_os_info(&self) -> Option<String>;
    fn get_wmi_cpu_info(&self) -> Option<String>;
    fn get_wmi_computer_info(&self) -> Option<String>;
    fn get_wmi_drives_info(&self) -> Option<String>;
    fn get_wmi_nicconfig(&self) -> Option<String>;
    fn get_wmi_nicconfig_details(&self) -> Option<String>;
    fn get_hosts_file(&self) -> Option<String>;
    fn get_protocols_file(&self) -> Option<String>;
    fn get_services_file(&self) -> Option<String>;
    fn get_wmi_printers_info(&self)-> Option<String>;
    fn get_wmi_services_info(&self)-> Option<String>;
    fn get_wmi_hotfixes_info(&self)-> Option<String>;
    fn get_wmi_shares_info(&self)-> Option<String>;
    fn get_wmi_network_adapters_info(&self)-> Option<String>;
    fn get_wmi_local_accounts_info(&self)-> Option<String>;
    fn get_wmi_bios_info(&self)-> Option<String>;
    fn get_wmi_motherboard_info(&self)-> Option<String>;
    fn get_wmi_processor_info(&self)-> Option<String>;
    fn get_wmi_physical_memory(&self)-> Option<String>;
    fn get_wmi_sound_info(&self)-> Option<String>;
    fn get_wmi_video_info(&self)-> Option<String>;
    fn get_wmi_monitor_info(&self)-> Option<String>;
}

pub struct SystemReader {}

impl SystemReader {
    pub fn new() -> SystemReader {
        SystemReader {}
    }
}

impl SystemReaderInterface for SystemReader {

    fn get_os_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["os", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_os_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["os", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

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

    fn get_wmi_computer_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["computersystem", "get", "/format:list"]).output().ok()?;
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
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\hosts").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }

    fn get_protocols_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\protocol").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }

    fn get_services_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\services").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }

    fn get_wmi_printers_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["printer",
                "get",
                "Attributes,Caption,CreationClassName,DeviceID,DoCompleteFirst,DriverName,\
                ExtendedPrinterStatus,HorizontalResolution,Local,Name,PortName,PrinterStatus,\
                PrintJobDataType,PrintProcessor,Priority,Status,SystemCreationClassName,\
                SystemName,VerticalResolution",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_services_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["service",
                "get",
                "AcceptPause,AcceptStop,Caption,CreationClassName,Description,DesktopInteract,\
                DisplayName,ErrorControl,ExitCode,Name,PathName,ServiceType,Started,StartMode,\
                StartName,State,Status,SystemCreationClassName,SystemName",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_hotfixes_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["qfe",
                "get",
                "Caption,CSName,Description,HotFixID,InstalledBy,InstalledOn",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_shares_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["share",
                "get",
                "Caption,Description,Name,Path,Status,Type,AllowMaximum",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_network_adapters_info(&self)-> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_local_accounts_info(&self)-> Option<String> {
        let output = Command::new("wmic")
            .args(&["useraccount", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_bios_info(&self)-> Option<String> {
        let output = Command::new("wmic")
            .args(&["bios", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_motherboard_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["baseboard", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_processor_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["cpu", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_physical_memory(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["memorychip", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_sound_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["sounddev", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_video_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["path","win32_VideoController", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_monitor_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["desktopmonitor", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

pub struct SystemInfo {
    system_reader: Box<SystemReaderInterface>,
    pub system_info: SystemInfoData,
    pub wmi_computer_info: WmiComputerInfo,
    pub wmi_os_version: WmiOsVersion,
    pub os_version: OsVersion,
    pub logical_drives: Vec<LogicalDrive>,
    pub interface_addresses: Vec<InterfaceAddress>,
    pub interface_details: Vec<InterfaceDetails>,
    pub etc_hosts: Vec<EtcHosts>,
    pub etc_protocols: Vec<EtcProtocols>,
    pub etc_services: Vec<EtcServices>,
    pub uptime: Result<Uptime, String>,
    pub wmi_printers: Vec<WmiPrinters>,
    pub wmi_services: Vec<WmiServices>,
    pub wmi_hotfixes: Vec<WmiHotfixes>,
    pub products: Vec<Products>,
    pub wmi_shares: Vec<WmiShares>,
    pub wmi_network_adapters: Vec<WmiNetworkAdapters>,
    pub wmi_local_accounts : Vec<WmiLocalAccounts>,
    pub wmi_bios: WmiBios,
    pub wmi_motherboard: WmiMotherboard,
    pub wmi_processor: WmiProcessor,
    pub wmi_physical_memory: Vec<WmiMemory>,
    pub wmi_sound: Vec<WmiSound>,
    pub wmi_video: Vec<WmiVideo>,
    pub wmi_monitors: Vec<WmiMonitors>,
}

impl SystemInfo {
    pub fn new(system_reader: Box<SystemReaderInterface>) -> SystemInfo {
        let mut system_info_data = SystemInfoData::new();
        system_info_data.update(system_reader.borrow());

        SystemInfo {
            system_info: system_info_data,
            wmi_computer_info: WmiComputerInfo::get_system_info(system_reader.borrow()),
            wmi_os_version: WmiOsVersion::new(system_reader.borrow()),
            os_version: OsVersion::new(system_reader.borrow()),
            logical_drives: LogicalDrive::get_drives(system_reader.borrow()),
            interface_addresses: InterfaceAddress::get_interfaces(system_reader.borrow()),
            interface_details: InterfaceDetails::get_interface_details(system_reader.borrow()),
            etc_hosts: EtcHosts::get_hosts(system_reader.borrow()),
            etc_protocols: EtcProtocols::get_protocols(system_reader.borrow()),
            etc_services: EtcServices::get_services(system_reader.borrow()),
            uptime: Uptime::get_uptime(),
            wmi_printers: WmiPrinters::get_printers_info(system_reader.borrow()),
            wmi_services: WmiServices::get_services_info(system_reader.borrow()),
            wmi_hotfixes: WmiHotfixes::get_hotfixes_info(system_reader.borrow()),
            products: Products::get_products_info(),
            wmi_shares: WmiShares::get_shares_info(system_reader.borrow()),
            wmi_network_adapters: WmiNetworkAdapters::get_netwok_adapters_info(system_reader.borrow()),
            wmi_local_accounts : WmiLocalAccounts::get_local_accounts_info(system_reader.borrow()),
            wmi_bios: WmiBios::get_bios_info(system_reader.borrow()),
            wmi_motherboard: WmiMotherboard::get_motherboard_info(system_reader.borrow()),
            wmi_processor: WmiProcessor::get_processor_info(system_reader.borrow()),
            wmi_physical_memory: WmiMemory::get_physical_memory_info(system_reader.borrow()),
            wmi_sound: WmiSound::get_sound_info(system_reader.borrow()),
            wmi_video: WmiVideo::get_video_info(system_reader.borrow()),
            wmi_monitors: WmiMonitors::get_monitors_info(system_reader.borrow()),
            system_reader,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&json!({
            "system_info": self.system_info,
            "wmi_computer_info": self.system_info,
            "wmi_os_version" : self.wmi_os_version,
            "os_version" : self.os_version,
            "logical_drives" : self.logical_drives,
            "interface_addresses" : self.interface_addresses,
            "interface_details" : self.interface_details,
            "etc_hosts" : self.etc_hosts,
            "etc_protocols" : self.etc_protocols,
            "etc_services" : self.etc_services,
            "uptime" : self.uptime,
            "wmi_printers" : self.wmi_printers,
            "wmi_services" : self.wmi_services,
            "wmi_hotfixes" : self.wmi_hotfixes,
            "products": self.products,
            "wmi_shares" : self.wmi_shares,
            "wmi_network_adapters" : self.wmi_network_adapters,
            "wmi_local_accounts" : self.wmi_local_accounts,
            "wmi_bios" : self.wmi_bios,
            "wmi_motherboard" : self.wmi_motherboard,
            "wmi_processor" : self.wmi_processor,
            "wmi_physical_memory" : self.wmi_physical_memory,
            "wmi_sound" : self.wmi_sound,
            "wmi_video" : self.wmi_video,
            "wmi_monitors" : self.wmi_monitors,
        })).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSystemReader{}

    impl SystemReaderInterface for MockSystemReader {
        fn get_os_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-osinfo.txt")))
        }

        fn get_wmi_os_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-os-version.txt")))
        }

        fn get_wmi_cpu_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-cpuinfo.txt")))
        }

        fn get_wmi_system_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-system-info.txt")))
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

        fn get_protocols_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/protocols.txt")))
        }

        fn get_services_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/services.txt")))
        }

        fn get_wmi_printers_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-printers.txt")))
        }

        fn get_wmi_services_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-services.txt")))
        }

        fn get_wmi_hotfixes_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-hotfixes.txt")))
        }
        fn get_wmi_shares_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-shares.txt")))
        }

        fn get_wmi_network_adapters_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-network-adapters.txt")))
        }

        fn get_wmi_local_accounts_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-local-accounts.txt")))
        }

        fn get_wmi_bios_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-bios.txt")))
        }

        fn get_wmi_motherboard_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-motherboard-info.txt")))
        }

        fn get_wmi_processor_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-processor.txt")))
        }

        fn get_wmi_physical_memory(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-physical-memory.txt")))
        }

        fn get_wmi_sound_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-sound.txt")))
        }

        fn get_wmi_video_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-video.txt")))
        }

        fn get_wmi_monitor_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-monitors.txt")))
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

        // wmi_computer_info
        assert_eq!(system_info.wmi_computer_info.computer_name, "Lucerne Publishing");
        assert_eq!(system_info.wmi_computer_info.domain, "STANDALONE");
        assert_eq!(system_info.wmi_computer_info.manufacturer, "Lucerne Publishing");
        assert_eq!(system_info.wmi_computer_info.model, "TailSpin Toys");
        assert_eq!(system_info.wmi_computer_info.number_of_processors, "18");
        assert_eq!(system_info.wmi_computer_info.system_type, "x128-based PC");

        // system_info
        assert_eq!(system_info.system_info.computer_name, "galaxy500");
        assert_eq!(system_info.system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.system_info.cpu_brand, "Intel(R) Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(system_info.system_info.physical_memory, 17043189760);

        // wmi_os_version
        assert_eq!(system_info.wmi_os_version.platform, "Windows");
        assert_eq!(system_info.wmi_os_version.csname, "Olympia");
        assert_eq!(system_info.wmi_os_version.version, "10.10.16299");
        assert_eq!(system_info.wmi_os_version.major, "10");
        assert_eq!(system_info.wmi_os_version.minor, "10");
        assert_eq!(system_info.wmi_os_version.build_number, "9999");
        assert_eq!(system_info.wmi_os_version.caption, "describe something here");
        assert_eq!(system_info.wmi_os_version.free_physical_mem, "10138896");
        assert_eq!(system_info.wmi_os_version.free_virtual_mem, "10900164");
        assert_eq!(system_info.wmi_os_version.manufacturer, "Wide World Importers");
        assert_eq!(system_info.wmi_os_version.name, "Wide World Importers 10 Home");
        assert_eq!(system_info.wmi_os_version.service_pack_major, "0");
        assert_eq!(system_info.wmi_os_version.service_pack_minor, "0");
        assert_eq!(system_info.wmi_os_version.size_stored_in_paging_file, "2490368");
        assert_eq!(system_info.wmi_os_version.total_virtual_mem_size, "19134092");
        assert_eq!(system_info.wmi_os_version.total_visible_mem_size, "16643724");
        assert_eq!(system_info.wmi_os_version.win_directory, "C:\\WINDOWS");

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

        //uptime
        assert_eq!(system_info.uptime.is_ok(), true);

        //wmi_printers
        let _test_printer = &system_info.wmi_printers.get(0);
        assert_eq!(_test_printer.unwrap().caption, "Snagit 2018");
        assert_eq!(_test_printer.unwrap().creation_class_name, "Win32_Printer");
        assert_eq!(_test_printer.unwrap().device_id, "Snagit 2018");
        assert_eq!(_test_printer.unwrap().do_complete_first, "FALSE");
        assert_eq!(_test_printer.unwrap().driver_name, "Snagit 18 Printer");
        assert_eq!(_test_printer.unwrap().extended_printer_status, "2");
        assert_eq!(_test_printer.unwrap().horizontal_resolution, "200");
        assert_eq!(_test_printer.unwrap().local, "TRUE");
        assert_eq!(_test_printer.unwrap().name, "Snagit 2018");
        assert_eq!(_test_printer.unwrap().port_name, "C:\\ProgramData\\TechSmith\\Snagit18\\PrinterPortFile");
        assert_eq!(_test_printer.unwrap().printer_status, "3");
        assert_eq!(_test_printer.unwrap().print_job_data_type, "RAW");
        assert_eq!(_test_printer.unwrap().print_processor, "winprint");
        assert_eq!(_test_printer.unwrap().priority, "1");
        assert_eq!(_test_printer.unwrap().status, "Unknown");
        assert_eq!(_test_printer.unwrap().system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(_test_printer.unwrap().system_name, "ekyaw");
        assert_eq!(_test_printer.unwrap().vertical_resolution, "200");

        //wmi-services
        let _test_service = &system_info.wmi_services.get(0);
        assert_eq!(_test_service.unwrap().accept_pause,"FALSE");
        assert_eq!(_test_service.unwrap().accept_stop,"TRUE");
        assert_eq!(_test_service.unwrap().caption,"Windows Push Notifications User Service_10b2b340");
        assert_eq!(_test_service.unwrap().creation_class_name,"Win32_Service");
        assert_eq!(_test_service.unwrap().description,"do something");
        assert_eq!(_test_service.unwrap().desktop_interact,"FALSE");
        assert_eq!(_test_service.unwrap().display_name,"Windows Push Notifications User Service_10b2b340");
        assert_eq!(_test_service.unwrap().error_control,"Ignore");
        assert_eq!(_test_service.unwrap().exit_code, 0);
        assert_eq!(_test_service.unwrap().name,"WpnUserService_10b2b340");
        assert_eq!(_test_service.unwrap().path_name,"C:\\WINDOWS\\system32\\svchost.exe -k UnistackSvcGroup");
        assert_eq!(_test_service.unwrap().service_type,"Unknown");
        assert_eq!(_test_service.unwrap().started,"TRUE");
        assert_eq!(_test_service.unwrap().start_mode,"Auto");
        assert_eq!(_test_service.unwrap().start_name,"");
        assert_eq!(_test_service.unwrap().state,"Running");
        assert_eq!(_test_service.unwrap().status,"OK");
        assert_eq!(_test_service.unwrap().system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(_test_service.unwrap().system_name, "waka-waka");

        //wmi-hotfixes
        let _test_hotfix = &system_info.wmi_hotfixes.get(0);
        assert_eq!(_test_hotfix.unwrap().caption,"http://support.microsoft.com/?kbid=4103");
        assert_eq!(_test_hotfix.unwrap().csname,"wakwaka");
        assert_eq!(_test_hotfix.unwrap().description,"Update");
        assert_eq!(_test_hotfix.unwrap().hotfix_id,"KB4103");
        assert_eq!(_test_hotfix.unwrap().installed_by,"wakwaka\\johnCena");
        assert_eq!(_test_hotfix.unwrap().installed_on,"5/10/2018");

        //wmi-shares
        let _test_share = &system_info.wmi_shares.get(0);
        assert_eq!(_test_share.unwrap().name,"print$");
        assert_eq!(_test_share.unwrap().caption,"Printer Drivers");
        assert_eq!(_test_share.unwrap().description,"Printer Drivers");
        assert_eq!(_test_share.unwrap().path,"C:\\WINDOWS\\system32\\spool\\drivers");
        assert_eq!(_test_share.unwrap().status,"OK");
        assert_eq!(_test_share.unwrap()._type,"Device Admin");
        assert_eq!(_test_share.unwrap().allow_maximum,"TRUE");

        //wmi-network-adapter
        let _wmi_network_adapter = &system_info.wmi_network_adapters.get(0);
        assert_eq!(_wmi_network_adapter.unwrap().description,"VMware Virtual Ethernet Adapter for VMnet8");
        assert_eq!(_wmi_network_adapter.unwrap().database_path,"%SystemRoot%\\System32\\drivers\\etc");
        assert_eq!(_wmi_network_adapter.unwrap().dhcp_enabled,"TRUE");
        assert_eq!(_wmi_network_adapter.unwrap().ip_address,vec!["192.168.197.1", "ff80::9999:ffff:9999:f9f9"]);
        assert_eq!(_wmi_network_adapter.unwrap().ip_enabled,"TRUE");
        assert_eq!(_wmi_network_adapter.unwrap().ip_subnet,vec!["255.255.255.0", "64"]);
        assert_eq!(_wmi_network_adapter.unwrap().mac_address,"FF:FF:FF:FF:FF:FF");

        //wmi-local-accounts
        let _wmi_local_account = &system_info.wmi_local_accounts.get(0);
        assert_eq!(_wmi_local_account.unwrap().account_type,"Server trust account");
        assert_eq!(_wmi_local_account.unwrap().caption,"bipbip\\Acc");
        assert_eq!(_wmi_local_account.unwrap().description,"A server account");
        assert_eq!(_wmi_local_account.unwrap()._domain,"bipbip1010");
        assert_eq!(_wmi_local_account.unwrap().local_account,"TRUE");
        assert_eq!(_wmi_local_account.unwrap().name,"UtilityAccount");
        assert_eq!(_wmi_local_account.unwrap().sid,"S-0-0-11-1111111111-111111111-111111111-111");
        assert_eq!(_wmi_local_account.unwrap().sid_type,"1");
        assert_eq!(_wmi_local_account.unwrap().status,"Degraded");
        assert_eq!(system_info.wmi_local_accounts.len(),2);

        //wmi-bios
        let bios_info = &system_info.wmi_bios;
        assert_eq!(bios_info.caption,"1.23.3");
        assert_eq!(bios_info.manufacturer,"Lucerne Publishing");
        assert_eq!(bios_info.release_date,"20180126");
        assert_eq!(bios_info.serial_number,"AAAAAAAA");
        assert_eq!(bios_info.smbios_version,"1.23.3");

        //wmi_motherboard
        let motherboard_info = &system_info.wmi_motherboard;
        assert_eq!(motherboard_info.name,"Base Board");
        assert_eq!(motherboard_info.manufacturer," The Phone Company");
        assert_eq!(motherboard_info.product," 958B84C99");
        assert_eq!(motherboard_info.serial_number," /D8D8DH2/ETFSC0070C000T/");
        assert_eq!(motherboard_info.version," A11");

        //wmi_processor
        let processor_info = &system_info.wmi_processor;
        assert_eq!(processor_info.name,"Fabrikam Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(processor_info.address_width,"64");
        assert_eq!(processor_info.cpu_satus,"CPU Enabled");
        assert_eq!(processor_info.current_clock_speed,"1600 Mhz");
        assert_eq!(processor_info.current_voltage,"11");
        assert_eq!(processor_info.description,"Fabrikam Family 6 Model 142 Stepping 9");
        assert_eq!(processor_info.external_clock,"100");
        assert_eq!(processor_info.hyper_threading_enabled,"FALSE");
        assert_eq!(processor_info.l2_cache_size,"512");
        assert_eq!(processor_info.l2_cache_speed,"0");
        assert_eq!(processor_info.l3_cache_size,"4096");
        assert_eq!(processor_info.l3_cache_speed,"0");
        assert_eq!(processor_info.manufacturer,"Fabrikam, Inc.");
        assert_eq!(processor_info.max_clock_speed,"2901 Mhz");
        assert_eq!(processor_info.number_of_cores,"2");
        assert_eq!(processor_info.number_of_logical_processors,"2");
        assert_eq!(processor_info.socket_designation,"U4E2");

        //wmi_physical_memory
        let physical_memory = &system_info.wmi_physical_memory.get(0);
        assert_eq!(physical_memory.unwrap().name,"Physical Memory");
        assert_eq!(physical_memory.unwrap().bank_label,"BANK 0");
        assert_eq!(physical_memory.unwrap().capacity,"17179869184 bytes");
        assert_eq!(physical_memory.unwrap().description,"Physical Memory");
        assert_eq!(physical_memory.unwrap().device_locator,"DIMM A");
        assert_eq!(physical_memory.unwrap().form_factor,"12");
        assert_eq!(physical_memory.unwrap().interleave_data_depth,"0");
        assert_eq!(physical_memory.unwrap().interleave_position,"0");
        assert_eq!(physical_memory.unwrap().manufacturer,"Fabrikam, Inc.");
        assert_eq!(physical_memory.unwrap().memory_type,"0");
        assert_eq!(physical_memory.unwrap().serial_number,"91A92B93C");
        assert_eq!(physical_memory.unwrap().speed,"2400");

        //wmi_sound
        let sound_info = &system_info.wmi_sound.get(0);
        assert_eq!(sound_info.unwrap().name,"Fabrikam Audio");
        assert_eq!(sound_info.unwrap().manufacturer,"Fabrikam, Inc.");
        assert_eq!(sound_info.unwrap().status,"OK");
        assert_eq!(sound_info.unwrap().dma_buffer_size,"256");

        //wmi_video
        let video_info = &system_info.wmi_video.get(0);
        assert_eq!(system_info.wmi_video.len(), 3);
        assert_eq!(video_info.unwrap().name,"Graphic Design Institute 940MX");
        assert_eq!(video_info.unwrap().adapter_compatibility,"Graphic Design Institute");
        assert_eq!(video_info.unwrap().adapter_dac_type,"Integrated RAMDAC");
        assert_eq!(video_info.unwrap().adapter_ram, 2.0);
        assert_eq!(video_info.unwrap().availability,"Power Cycle");
        assert_eq!(video_info.unwrap().driver_version,"23.21.13.9065");
        assert_eq!(video_info.unwrap().installed_display_driver.len(), 2);
        assert_eq!(video_info.unwrap().refresh_rate,"60");
        assert_eq!(video_info.unwrap().screen_info,"1920 x 1080 x 4294967296 colors");
        assert_eq!(video_info.unwrap().status,"OK");
        assert_eq!(video_info.unwrap().video_architecture,"MDA");
        assert_eq!(video_info.unwrap().video_memory_type,"WRAM");

        //wmi_monitors
        assert_eq!(system_info.wmi_monitors.len(), 3);

        let monitor_info = &system_info.wmi_monitors.get(0);
        assert_eq!(monitor_info.unwrap().name,"Default Monitor");
        assert_eq!(monitor_info.unwrap().availability,"In Test");
        assert_eq!(monitor_info.unwrap().bandwidth, 0);
        assert_eq!(monitor_info.unwrap().screen_height, 1080);
        assert_eq!(monitor_info.unwrap().screen_width, 1920);
        assert_eq!(monitor_info.unwrap().manufacturer,"");
    }
}