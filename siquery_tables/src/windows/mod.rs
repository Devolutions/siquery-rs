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
    SystemInfoData,
    Uptime,
    WmiPrinters,
    WmiServices,
    WmiHotfixes,
    Products,
    WmiShares,
};
use std::env;

mod interface_address;
mod interface_details;
mod logical_drive;
mod os_version;
mod system_info;
mod uptime;
mod wmi_printers;
mod wmi_services;
mod wmi_hotfixes;
mod products;
mod wmi_shares;


pub trait SystemReaderInterface {
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
                "Attributes,Caption,CreationClassName,DeviceID,DoCompleteFirst,DriverName,ExtendedPrinterStatus,HorizontalResolution,Local,Name,PortName,PrinterStatus,PrintJobDataType,PrintProcessor,Priority,Status,SystemCreationClassName,SystemName,VerticalResolution",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_services_info(&self)-> Option<String>{
        let output = Command::new("wmic")
            .args(&["service",
                "get",
                "AcceptPause,AcceptStop,Caption,CreationClassName,Description,DesktopInteract,DisplayName,ErrorControl,ExitCode,Name,PathName,ServiceType,Started,StartMode,StartName,State,Status,SystemCreationClassName,SystemName",
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
}

pub struct SystemInfo {
    system_reader: Box<SystemReaderInterface>,
    pub system_info: SystemInfoData,
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
            etc_protocols: EtcProtocols::get_protocols(system_reader.borrow()),
            etc_services: EtcServices::get_services(system_reader.borrow()),
            uptime: Uptime::get_uptime(),
            wmi_printers: WmiPrinters::get_printers_info(system_reader.borrow()),
            wmi_services: WmiServices::get_services_info(system_reader.borrow()),
            wmi_hotfixes: WmiHotfixes::get_hotfixes_info(system_reader.borrow()),
            products: Products::get_products_info(),
            wmi_shares: WmiShares::get_shares_info(system_reader.borrow()),
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
            "etc_hosts" : self.etc_hosts,
            "etc_protocols" : self.etc_protocols,
            "etc_services" : self.etc_services,
            "uptime" : self.uptime,
            "wmi_printers" : self.wmi_printers,
            "wmi_services" : self.wmi_services,
            "wmi_hotfixes" : self.wmi_hotfixes,
            "products": self.products,
            "wmi_shares" : self.wmi_shares,
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

        fn get_protocols_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/protocols.txt")))
        }

        fn get_services_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/services.txt")))
        }

        fn get_wmi_printers_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-printers.txt")))
        }

        fn get_wmi_services_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-services.txt")))
        }

        fn get_wmi_hotfixes_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-hotfixes.txt")))
        }

        fn get_wmi_shares_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-shares.txt")))
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
    }
}