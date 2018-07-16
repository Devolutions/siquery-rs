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
    WmiKeyboard,
    WmiPointingDevice,
    ProcessOpenSocketsRow,
    ProcessesRow,
    ProcessMemoryMapRow,
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
mod wmi_keyboard;
mod wmi_pointing_device;
mod process_open_sockets;
mod processes;
mod process_memory_map;

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
    fn get_wmi_keyboard_info(&self)-> Option<String>;
    fn get_wmi_pointing_device(&self)-> Option<String>;
    fn get_wmi_process_info(&self) -> Option<String>;
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

    fn get_wmi_printers_info(&self) -> Option<String> {
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

    fn get_wmi_services_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["service",
                "get",
                "AcceptPause,AcceptStop,Caption,CreationClassName,Description,DesktopInteract,\
                DisplayName,ErrorControl,ExitCode,Name,PathName,ServiceType,Started,StartMode,\
                StartName,State,Status,SystemCreationClassName,SystemName",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_hotfixes_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["qfe",
                "get",
                "Caption,CSName,Description,HotFixID,InstalledBy,InstalledOn",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_shares_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["share",
                "get",
                "Caption,Description,Name,Path,Status,Type,AllowMaximum",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_network_adapters_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["nicconfig", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_local_accounts_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["useraccount", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_bios_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["bios", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_motherboard_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["baseboard", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_processor_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["cpu", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_physical_memory(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["memorychip", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_sound_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["sounddev", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_video_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_monitor_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["desktopmonitor", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_keyboard_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "Win32_Keyboard", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_pointing_device(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "Win32_PointingDevice", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_wmi_process_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "Win32_Process", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
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

        fn get_wmi_keyboard_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-keyboard.txt")))
        }

        fn get_wmi_pointing_device(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-pointing-device.txt")))
        }

        fn get_wmi_process_info(&self) -> Option<String> {
            //TODO test file
            Some(String::from(include_str!("../../test_data/wmi-pointing-device.txt")))
        }
    }

    #[test]
    fn test_system_info() {
        let system_reader: Box<SystemReaderInterface> = Box::new(MockSystemReader{});

        // checking possible cases for services file
        let etc_services = EtcServices::get_specific(system_reader.borrow());
        assert_eq!(etc_services.get(0).unwrap().name, "echo");
        assert_eq!(etc_services.get(0).unwrap().port, 7);
        assert_eq!(etc_services.get(0).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(0).unwrap().aliases, "");
        assert_eq!(etc_services.get(0).unwrap().comment, "");
        assert_eq!(etc_services.get(2).unwrap().name, "discard");
        assert_eq!(etc_services.get(2).unwrap().port, 9);
        assert_eq!(etc_services.get(2).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(2).unwrap().aliases, "sink null");
        assert_eq!(etc_services.get(2).unwrap().comment, "");
        assert_eq!(etc_services.get(12).unwrap().name, "ftp-data");
        assert_eq!(etc_services.get(12).unwrap().port, 20);
        assert_eq!(etc_services.get(12).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(12).unwrap().aliases, "");
        assert_eq!(etc_services.get(12).unwrap().comment, "FTP, data");
        assert_eq!(etc_services.len(), 15);

        //protocols
        let etc_protocols = EtcProtocols::get_specific(system_reader.borrow());
        assert_eq!(etc_protocols.get(0).unwrap().name, "ip");
        assert_eq!(etc_protocols.get(0).unwrap().number, 0);
        assert_eq!(etc_protocols.get(0).unwrap().alias, "IP");
        assert_eq!(etc_protocols.get(0).unwrap().comment, "internet protocol, pseudo protocol number");
        assert_eq!(etc_protocols.get(1).unwrap().name, "icmp");
        assert_eq!(etc_protocols.get(1).unwrap().number, 1);
        assert_eq!(etc_protocols.get(1).unwrap().alias, "ICMP");
        assert_eq!(etc_protocols.get(1).unwrap().comment, "internet control message protocol");
        assert_eq!(etc_protocols.len(), 3);

        //hosts
        let etc_hosts = EtcHosts::get_specific(system_reader.borrow());
        assert_eq!(etc_hosts.get(0).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(0).unwrap().hostnames, "localhost");
        assert_eq!(etc_hosts.get(1).unwrap().address, "255.255.255.255");
        assert_eq!(etc_hosts.get(1).unwrap().hostnames, "broadcasthost");
        assert_eq!(etc_hosts.get(2).unwrap().address, "::1");
        assert_eq!(etc_hosts.get(2).unwrap().hostnames, "localhost");
        assert_eq!(etc_hosts.get(3).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(3).unwrap().hostnames, "example.com,example");
        assert_eq!(etc_hosts.get(4).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(4).unwrap().hostnames, "example.net");
        assert_eq!(etc_hosts.len(), 5);

        // wmi_computer_info
        let wmi_computer_info = &WmiComputerInfo::get_specific(system_reader.borrow())[0];
        assert_eq!(wmi_computer_info.computer_name, "Lucerne Publishing");
        assert_eq!(wmi_computer_info.domain, "STANDALONE");
        assert_eq!(wmi_computer_info.manufacturer, "Lucerne Publishing");
        assert_eq!(wmi_computer_info.model, "TailSpin Toys");
        assert_eq!(wmi_computer_info.number_of_processors, "18");
        assert_eq!(wmi_computer_info.system_type, "x128-based PC");

        // system_info
        let system_info = &SystemInfoData::get_specific(system_reader.borrow())[0];
        assert_eq!(system_info.computer_name, "galaxy500");
        assert_eq!(system_info.cpu_logical_cores, 4);
        assert_eq!(system_info.cpu_brand, "Intel(R) Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(system_info.physical_memory, 17043189760);

        // wmi_os_version
        let wmi_os_version = &WmiOsVersion::get_specific(system_reader.borrow())[0];
        assert_eq!(wmi_os_version.platform, "Windows");
        assert_eq!(wmi_os_version.csname, "Olympia");
        assert_eq!(wmi_os_version.version, "10.10.16299");
        assert_eq!(wmi_os_version.major, "10");
        assert_eq!(wmi_os_version.minor, "10");
        assert_eq!(wmi_os_version.build_number, "9999");
        assert_eq!(wmi_os_version.caption, "describe something here");
        assert_eq!(wmi_os_version.free_physical_mem, "10138896");
        assert_eq!(wmi_os_version.free_virtual_mem, "10900164");
        assert_eq!(wmi_os_version.manufacturer, "Wide World Importers");
        assert_eq!(wmi_os_version.name, "Wide World Importers 10 Home");
        assert_eq!(wmi_os_version.service_pack_major, "0");
        assert_eq!(wmi_os_version.service_pack_minor, "0");
        assert_eq!(wmi_os_version.size_stored_in_paging_file, "2490368");
        assert_eq!(wmi_os_version.total_virtual_mem_size, "19134092");
        assert_eq!(wmi_os_version.total_visible_mem_size, "16643724");
        assert_eq!(wmi_os_version.win_directory, "C:\\WINDOWS");

        // os_version
        let os_version = &OsVersion::get_specific(system_reader.borrow())[0];
        assert_eq!(os_version.platform, "Windows");
        assert_eq!(os_version.name, "Microsoft Windows 10 Pro");
        assert_eq!(os_version.version, "10.0.16299");
        assert_eq!(os_version.major, 10);
        assert_eq!(os_version.minor, 0);

        // logical_drives
        let logical_drives = LogicalDrive::get_specific(system_reader.borrow());
        assert_eq!(logical_drives.len(), 2);

        let drive = LogicalDrive::get_specific(system_reader.borrow());
        assert_eq!(drive[0].device_id, "C:");
        assert_eq!(drive[0].file_system, "NTFS");
        assert_eq!(drive[0].size, 496869830656);
        assert_eq!(drive[0].free_space, 55674548224);

        assert_eq!(drive[1].device_id, "E:");
        assert_eq!(drive[1].file_system, "NTFS");
        assert_eq!(drive[1].size, 501215232);
        assert_eq!(drive[1].free_space, 469622784);

        // interface_addresses
        assert_eq!(InterfaceAddress::get_specific(system_reader.borrow()).len(), 1);

        let interface = &InterfaceAddress::get_specific(system_reader.borrow())[0];
        assert_eq!(interface.interface, "1");
        assert_eq!(interface.friendly_name, "Realtek USB GbE Family Controller");
        assert_eq!(interface.address, "192.168.1.172");
        assert_eq!(interface.mask, "255.255.248.0");
        assert_eq!(interface.interface_type, "dhcp");

        // interface_details
        let interface_details = &InterfaceDetails::get_specific(system_reader.borrow())[0];

        assert_eq!(InterfaceDetails::get_specific(system_reader.borrow()).len(), 1);
        assert_eq!(interface_details.interface, "1");
        assert_eq!(interface_details.mac, "A0:CE:C8:05:0D:32");
        assert_eq!(interface_details.enabled, 1);
        assert_eq!(interface_details.mtu, 1400);

        //wmi_printers
        let _test_printer = &WmiPrinters::get_specific(system_reader.borrow())[0];
        assert_eq!(_test_printer.caption, "Snagit 2018");
        assert_eq!(_test_printer.creation_class_name, "Win32_Printer");
        assert_eq!(_test_printer.device_id, "Snagit 2018");
        assert_eq!(_test_printer.do_complete_first, "FALSE");
        assert_eq!(_test_printer.driver_name, "Snagit 18 Printer");
        assert_eq!(_test_printer.extended_printer_status, "2");
        assert_eq!(_test_printer.horizontal_resolution, "200");
        assert_eq!(_test_printer.local, "TRUE");
        assert_eq!(_test_printer.name, "Snagit 2018");
        assert_eq!(_test_printer.port_name, "C:\\ProgramData\\TechSmith\\Snagit18\\PrinterPortFile");
        assert_eq!(_test_printer.printer_status, "3");
        assert_eq!(_test_printer.print_job_data_type, "RAW");
        assert_eq!(_test_printer.print_processor, "winprint");
        assert_eq!(_test_printer.priority, "1");
        assert_eq!(_test_printer.status, "Unknown");
        assert_eq!(_test_printer.system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(_test_printer.system_name, "ekyaw");
        assert_eq!(_test_printer.vertical_resolution, "200");

        //wmi-services
        let _test_service = &WmiServices::get_specific(system_reader.borrow())[0];
        assert_eq!(_test_service.accept_pause,"FALSE");
        assert_eq!(_test_service.accept_stop,"TRUE");
        assert_eq!(_test_service.caption,"Windows Push Notifications User Service_10b2b340");
        assert_eq!(_test_service.creation_class_name,"Win32_Service");
        assert_eq!(_test_service.description,"do something");
        assert_eq!(_test_service.desktop_interact,"FALSE");
        assert_eq!(_test_service.display_name,"Windows Push Notifications User Service_10b2b340");
        assert_eq!(_test_service.error_control,"Ignore");
        assert_eq!(_test_service.exit_code, 0);
        assert_eq!(_test_service.name,"WpnUserService_10b2b340");
        assert_eq!(_test_service.path_name,"C:\\WINDOWS\\system32\\svchost.exe -k UnistackSvcGroup");
        assert_eq!(_test_service.service_type,"Unknown");
        assert_eq!(_test_service.started,"TRUE");
        assert_eq!(_test_service.start_mode,"Auto");
        assert_eq!(_test_service.start_name,"");
        assert_eq!(_test_service.state,"Running");
        assert_eq!(_test_service.status,"OK");
        assert_eq!(_test_service.system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(_test_service.system_name, "waka-waka");

        //wmi-hotfixes
        let _test_hotfix = &WmiHotfixes::get_specific(system_reader.borrow())[0];
        assert_eq!(_test_hotfix.caption,"http://support.microsoft.com/?kbid=4103");
        assert_eq!(_test_hotfix.csname,"wakwaka");
        assert_eq!(_test_hotfix.description,"Update");
        assert_eq!(_test_hotfix.hotfix_id,"KB4103");
        assert_eq!(_test_hotfix.installed_by,"wakwaka\\johnCena");
        assert_eq!(_test_hotfix.installed_on,"5/10/2018");

        //wmi-shares
        let _test_share = &WmiShares::get_specific(system_reader.borrow())[0];
        assert_eq!(_test_share.name,"print$");
        assert_eq!(_test_share.caption,"Printer Drivers");
        assert_eq!(_test_share.description,"Printer Drivers");
        assert_eq!(_test_share.path,"C:\\WINDOWS\\system32\\spool\\drivers");
        assert_eq!(_test_share.status,"OK");
        assert_eq!(_test_share._type,"Device Admin");
        assert_eq!(_test_share.allow_maximum,"TRUE");

        //wmi-network-adapter
        let _wmi_network_adapter = &WmiNetworkAdapters::get_specific(system_reader.borrow())[0];
        assert_eq!(_wmi_network_adapter.description,"VMware Virtual Ethernet Adapter for VMnet8");
        assert_eq!(_wmi_network_adapter.database_path,"%SystemRoot%\\System32\\drivers\\etc");
        assert_eq!(_wmi_network_adapter.dhcp_enabled,"TRUE");
        assert_eq!(_wmi_network_adapter.ip_address,vec!["192.168.197.1", "ff80::9999:ffff:9999:f9f9"]);
        assert_eq!(_wmi_network_adapter.ip_enabled,"TRUE");
        assert_eq!(_wmi_network_adapter.ip_subnet,vec!["255.255.255.0", "64"]);
        assert_eq!(_wmi_network_adapter.mac_address,"FF:FF:FF:FF:FF:FF");

        //wmi-local-accounts
        let _wmi_local_account = &WmiLocalAccounts::get_specific(system_reader.borrow())[0];
        assert_eq!(_wmi_local_account.account_type,"Server trust account");
        assert_eq!(_wmi_local_account.caption,"bipbip\\Acc");
        assert_eq!(_wmi_local_account.description,"A server account");
        assert_eq!(_wmi_local_account._domain,"bipbip1010");
        assert_eq!(_wmi_local_account.local_account,"TRUE");
        assert_eq!(_wmi_local_account.name,"UtilityAccount");
        assert_eq!(_wmi_local_account.sid,"S-0-0-11-1111111111-111111111-111111111-111");
        assert_eq!(_wmi_local_account.sid_type,"1");
        assert_eq!(_wmi_local_account.status,"Degraded");
        assert_eq!(WmiLocalAccounts::get_specific(system_reader.borrow()).len(),2);

        //wmi-bios
        let bios_info = WmiBios::get_specific(system_reader.borrow());
        assert_eq!(bios_info.caption,"1.23.3");
        assert_eq!(bios_info.manufacturer,"Lucerne Publishing");
        assert_eq!(bios_info.release_date,"20180126");
        assert_eq!(bios_info.serial_number,"AAAAAAAA");
        assert_eq!(bios_info.smbios_version,"1.23.3");

        //wmi_motherboard
        let motherboard_info = &WmiMotherboard::get_specific(system_reader.borrow())[0];
        assert_eq!(motherboard_info.name,"Base Board");
        assert_eq!(motherboard_info.manufacturer," The Phone Company");
        assert_eq!(motherboard_info.product," 958B84C99");
        assert_eq!(motherboard_info.serial_number," /D8D8DH2/ETFSC0070C000T/");
        assert_eq!(motherboard_info.version," A11");

        //wmi_processor
        let processor_info = &WmiProcessor::get_specific(system_reader.borrow())[0];
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
        let physical_memory = &WmiMemory::get_specific(system_reader.borrow())[0];
        assert_eq!(physical_memory.name,"Physical Memory");
        assert_eq!(physical_memory.bank_label,"BANK 0");
        assert_eq!(physical_memory.capacity,"17179869184 bytes");
        assert_eq!(physical_memory.description,"Physical Memory");
        assert_eq!(physical_memory.device_locator,"DIMM A");
        assert_eq!(physical_memory.form_factor,"12");
        assert_eq!(physical_memory.interleave_data_depth,"0");
        assert_eq!(physical_memory.interleave_position,"0");
        assert_eq!(physical_memory.manufacturer,"Fabrikam, Inc.");
        assert_eq!(physical_memory.memory_type,"0");
        assert_eq!(physical_memory.serial_number,"91A92B93C");
        assert_eq!(physical_memory.speed,"2400");

        //wmi_sound
        let sound_info = &WmiSound::get_specific(system_reader.borrow())[0];
        assert_eq!(sound_info.name,"Fabrikam Audio");
        assert_eq!(sound_info.manufacturer,"Fabrikam, Inc.");
        assert_eq!(sound_info.status,"OK");
        assert_eq!(sound_info.dma_buffer_size,"256");

        //wmi_video
        let video_info = &WmiVideo::get_specific(system_reader.borrow())[0];
        assert_eq!(WmiVideo::get_specific(system_reader.borrow()).len(), 3);
        assert_eq!(video_info.name,"Graphic Design Institute 940MX");
        assert_eq!(video_info.adapter_compatibility,"Graphic Design Institute");
        assert_eq!(video_info.adapter_dac_type,"Integrated RAMDAC");
        assert_eq!(video_info.adapter_ram, 2.0);
        assert_eq!(video_info.availability,"Power Cycle");
        assert_eq!(video_info.driver_version,"23.21.13.9065");
        assert_eq!(video_info.installed_display_driver.len(), 2);
        assert_eq!(video_info.refresh_rate,"60");
        assert_eq!(video_info.screen_info,"1920 x 1080 x 4294967296 colors");
        assert_eq!(video_info.status,"OK");
        assert_eq!(video_info.video_architecture,"MDA");
        assert_eq!(video_info.video_memory_type,"WRAM");

        //wmi_monitors
        assert_eq!(WmiMonitors::get_specific(system_reader.borrow()).len(), 3);

        let monitor_info = &WmiMonitors::get_specific(system_reader.borrow())[0];
        assert_eq!(monitor_info.name,"Default Monitor");
        assert_eq!(monitor_info.availability,"In Test");
        assert_eq!(monitor_info.bandwidth, 0);
        assert_eq!(monitor_info.screen_height, 1080);
        assert_eq!(monitor_info.screen_width, 1920);
        assert_eq!(monitor_info.manufacturer,"");

        //wmi_monitors
        assert_eq!(WmiKeyboard::get_specific(system_reader.borrow()).len(), 2);

        let keyboard_info = &WmiKeyboard::get_specific(system_reader.borrow())[0];
        assert_eq!(keyboard_info.name, "Enhanced (101- or 102-key)");
        assert_eq!(keyboard_info.description, "USB Input Device");
        assert_eq!(keyboard_info.device_id, "USB\\VID_046D&amp;0&amp;0000");
        assert_eq!(keyboard_info.status, "OK");

        //wmi_pointing_device
        assert_eq!(WmiPointingDevice::get_specific(system_reader.borrow()).len(), 3);

        let pointing_device_info = &WmiPointingDevice::get_specific(system_reader.borrow())[0];
        assert_eq!(pointing_device_info.name,"PS/2 Compatible Mouse");
        assert_eq!(pointing_device_info.manufacturer,"Fabrikam, Inc.");
        assert_eq!(pointing_device_info.description, "PS/2 Compatible Mouse");
        assert_eq!(pointing_device_info.pointing_type, "Touch Screen");
        assert_eq!(pointing_device_info.status, "OK");
    }
}