#[macro_use]
extern crate clap;

extern crate siquery;

use clap::App;

use siquery::SystemInfo;
use siquery::sys::SystemReader;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();

    let system_reader = Box::new(SystemReader::new());
    let system_info = SystemInfo::new(system_reader);

    let table = matches.value_of("table").unwrap_or("").to_string();

    match table.as_str() {
        "os_version" => {
            println!("Operating System");
            println!("  Platform: {} ", system_info.os_version.platform);
            println!("  Name: {}", system_info.os_version.name);
            println!(
                "  Version: {} Major: {} Minor: {}",
                system_info.os_version.version, system_info.os_version.major, system_info.os_version.minor
            );
        },
        "system_info" => {
            println!("Cpu brand: {}", system_info.system_info.cpu_brand);
            println!("Cpu cores: {}", system_info.system_info.cpu_logical_cores);
            println!(
                "Physical memory: {} bytes",
                system_info.system_info.physical_memory
            );
        },
        "logical_drives" => {
            println!("Logical Drives");
            for drive in &system_info.logical_drives {
                println!("  Name: {}", drive.device_id);
                println!("  File system: {}", drive.file_system);
                println!("  Size: {}", drive.size);
                println!("  Free space: {}", drive.free_space);
                println!("  Drive Type: {}", drive.drive_type);
            }
        },
        "etc_hosts" => {
            for hosts in &system_info.etc_hosts {
                println!("{:?}", &hosts);
            }
        },
        "etc_protocols" => {
            for protocol_entries in &system_info.etc_protocols {
                println!("{:?}", protocol_entries);
            }
        },
        "etc_services" => {
            for services_entries in &system_info.etc_services {
                println!("{:?}", services_entries);
            }
        },
        "uptime" => {
            println!("{:?}", &system_info.uptime.unwrap());
        }

        "wmi_printers" => {
            #[cfg(target_os = "windows")]
            for printer in &system_info.wmi_printers {
                println!("Printer name : {:?}", &printer.name);
                println!("Attributes : {:?}", &printer.attributes);
                println!("Caption : {:?}", &printer.caption);
                println!("Creation class name : {:?}", &printer.creation_class_name);
                println!("DeviceID : {:?}", &printer.device_id);
                println!("Do complete first : {:?}", &printer.do_complete_first);
                println!("Driver name :{:?}", &printer.driver_name);
                println!("Extended printer status : {:?}", &printer.extended_printer_status);
                println!("Horizontal resolution : {:?}", &printer.horizontal_resolution);
                println!("local : {:?}", &printer.local);
                println!("Port name : {:?}", &printer.port_name);
                println!("Printer status : {:?}", &printer.printer_status);
                println!("Print job data type : {:?}", &printer.print_job_data_type);
                println!("Print processor : {:?}", &printer.print_processor);
                println!("Priority : {:?}", &printer.priority);
                println!("Status : {:?}", &printer.status);
                println!("System creation class name : {:?}", &printer.system_creation_class_name);
                println!("System name : {:?}", &printer.system_name);
                println!("Vertical resolution : {:?}", &printer.vertical_resolution);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }

        "wmi_services" => {
            #[cfg(target_os = "windows")]
            for service in &system_info.wmi_services {
                println!("AcceptPause: {:?}", &service.accept_pause);
                println!("AcceptStop: {:?}", &service.accept_stop);
                println!("Caption: {:?}", &service.caption);
                println!("CreationClassName: {:?}", &service.creation_class_name);
                println!("Description: {:?}", &service.description);
                println!("DesktopInteract: {:?}", &service.desktop_interact);
                println!("DisplayName: {:?}", &service.display_name);
                println!("ErrorControl: {:?}", &service.error_control);
                println!("ExitCode: {:?}", &service.exit_code);
                println!("Name: {:?}", &service.name);
                println!("PathName: {:?}", &service.path_name);
                println!("ServiceType:  {:?}", &service.service_type);
                println!("Started: {:?}", &service.started);
                println!("StartMode: {:?}", &service.start_mode);
                println!("StartName: {:?}", &service.start_name);
                println!("State: {:?}", &service.state);
                println!("Status: {:?}", &service.status);
                println!("SystemCreationClassName: {:?}", &service.system_creation_class_name);
                println!("SystemName: {:?}", &service.system_name);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
                println!("Not implemented!");
        }
        "wmi_hotfixes" => {
            #[cfg(target_os = "windows")]
            for hotfix in &system_info.wmi_hotfixes {
                println!("Caption: {:?}", &hotfix.caption);
                println!("CSName: {:?}", &hotfix.csname);
                println!("Description: {:?}", &hotfix.description);
                println!("HotFixID: {:?}", &hotfix.hotfix_id);
                println!("InstalledBy: {:?}", &hotfix.installed_by);
                println!("InstalledOn: {:?}", &hotfix.installed_on);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
                println!("Not implemented!");
        }
        "products" => {
            #[cfg(target_os = "windows")]
            for product in &system_info.products{
                println!("Name: {}",	product.name);
                println!("HelpLink: {}", product.help_link);
                println!("InstallDate: {}", product.install_date);
                println!("InstallLocation: {}", product.install_location);
                println!("Vendor: {}", product.vendor);
                println!("Version: {}", product.version);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
                println!("Not implemented!");
        }
        "wmi_shares" => {
            #[cfg(target_os = "windows")]
                for share in &system_info.wmi_shares{
                println!("Name: {}",	share.name);
                println!("caption: {}", share.caption);
                println!("description: {}", share.description);
                println!("Path: {}", share.path);
                println!("Status: {}", share.status);
                println!("Type: {}", share._type);
                println!("AllowMaximum: {}", share.allow_maximum);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
                println!("Not implemented!");
        }
        "wmi_network_adapters" => {
            #[cfg(target_os = "windows")]
            for network_adapter in &system_info.wmi_network_adapters {
                println!("Description: {}", network_adapter.description);
                println!("DatabasePath: {}", network_adapter.database_path);
                println!("DHCPEnabled: {}", network_adapter.dhcp_enabled);
                println!("IPAddress{:?}", network_adapter.ip_address);
                println!("IPEnabled: {}", network_adapter.ip_enabled);
                println!("IPSubnet: {:?}", network_adapter.ip_subnet);
                println!("MACAddress: {}", network_adapter.mac_address);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        _ => {}
    }
}