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
        "wmi_os_version" => {
            #[cfg(target_os = "windows")]
            {
                println!("Operating System");
                println!("  Platform: {} ", system_info.wmi_os_version.platform);
                println!("  BuildNumber: {} ", system_info.wmi_os_version.build_number);
                println!("  CSName: {}", system_info.wmi_os_version.csname);
                println!("  Caption: {} ", system_info.wmi_os_version.caption);
                println!("  FreePhysicalMemory: {} ", system_info.wmi_os_version.free_physical_mem);
                println!("  FreeVirtualMemory: {} ", system_info.wmi_os_version.free_virtual_mem);
                println!(
                    "  Version: {} Major: {} Minor: {}",
                    system_info.wmi_os_version.version, system_info.wmi_os_version.major, system_info.wmi_os_version.minor
                );
                println!("  Manufacturer: {} ", system_info.wmi_os_version.manufacturer);
                println!("  Name: {} ", system_info.wmi_os_version.name);
                println!("  ServicePackMajorVersion: {} ", system_info.wmi_os_version.service_pack_major);
                println!("  ServicePackMinorVersion: {} ", system_info.wmi_os_version.service_pack_minor);
                println!("  SizeStoredInPagingFiles: {} ", system_info.wmi_os_version.size_stored_in_paging_file);
                println!("  TotalVirtualMemorySize: {} ", system_info.wmi_os_version.total_virtual_mem_size);
                println!("  TotalVisibleMemorySize: {} ", system_info.wmi_os_version.total_visible_mem_size);
                println!("  WindowsDirectory: {} ", system_info.wmi_os_version.win_directory);
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        },
        "os_version" => {
            println!("Operating System");
            println!("  Platform: {} ", system_info.os_version.platform);
            println!("  Name: {}", system_info.os_version.name);
            println!(
                "  Version: {} Major: {} Minor: {}",
                system_info.os_version.version, system_info.os_version.major, system_info.os_version.minor
            );
        },
        "wmi_computer_info" => {
            #[cfg(target_os = "windows")]
            {
                println!("Name: {}", system_info.wmi_computer_info.computer_name);
                println!("Domain: {}", system_info.wmi_computer_info.domain);
                println!("Manufacturer: {}", system_info.wmi_computer_info.manufacturer);
                println!("Model: {}", system_info.wmi_computer_info.model);
                println!("NumberOfProcessors: {}", system_info.wmi_computer_info.number_of_processors);
                println!("SystemType: {}", system_info.wmi_computer_info.system_type);
            }

            #[cfg(not(windows))]
            println!("Not implemented!");
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
            for product in &system_info.products {
                println!("Name: {}", product.name);
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
            for share in &system_info.wmi_shares {
                println!("Name: {}", share.name);
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
        "wmi_local_accounts" => {
            #[cfg(target_os = "windows")]
            for local_account in &system_info.wmi_local_accounts {
                println!("AccountType: {}", local_account.account_type);
                println!("Caption: {}", local_account.caption);
                println!("Description: {}", local_account.description);
                println!("Domain: {}", local_account._domain);
                println!("Local Account: {:?}", local_account.local_account);
                println!("Name: {}", local_account.name);
                println!("SID: {}", local_account.sid);
                println!("SID Type: {}", local_account.sid_type);
                println!("Status: {}", local_account.status);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_bios" => {
            #[cfg(target_os = "windows")]
            let bios_info = &system_info.wmi_bios;
            println!("Caption: {}", bios_info.caption);
            println!("Manufacturer: {}", bios_info.manufacturer);
            println!("Release Date: {}", bios_info.release_date);
            println!("Serial Number: {}", bios_info.serial_number);
            println!("SMBIOS BIOS Version: {}", bios_info.smbios_version);

            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_motherboard" => {
            #[cfg(target_os = "windows")]
            let motherboard_info = &system_info.wmi_motherboard;
            println!("Name: {}", motherboard_info.name);
            println!("Manufacturer: {}", motherboard_info.manufacturer);
            println!("Product: {}", motherboard_info.product);
            println!("Serial Number: {}", motherboard_info.serial_number);
            println!("Version: {}", motherboard_info.version);

            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_processor" => {
            #[cfg(target_os = "windows")]
            let processor_info = &system_info.wmi_processor;
            println!("Name: {}", processor_info.name);
            println!("Address width: {}", processor_info.address_width);
            println!("Cpu status: {}", processor_info.cpu_satus);
            println!("Current clock speed: {} Mhz", processor_info.current_clock_speed);
            println!("Current voltage: {}", processor_info.current_voltage);
            println!("Description: {}", processor_info.description);
            println!("External clock: {} Mhz", processor_info.external_clock);
            println!("Hyper threading enabled: {}", processor_info.hyper_threading_enabled);
            println!("L2 cache size: {}", processor_info.l2_cache_size);
            println!("L2 cache speed: {}", processor_info.l2_cache_speed);
            println!("L3 cache size: {}", processor_info.l3_cache_size);
            println!("L3 cache speed: {}", processor_info.l3_cache_speed);
            println!("Manufacturer: {}", processor_info.manufacturer);
            println!("Max clock speed: {} Mhz", processor_info.max_clock_speed);
            println!("Number of cores: {}", processor_info.number_of_cores);
            println!("Number of logical processors: {}", processor_info.number_of_logical_processors);
            println!("Socket designation: {}", processor_info.socket_designation);

            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_physical_memory" => {
            #[cfg(target_os = "windows")]
            for physical_memory_info in &system_info.wmi_physical_memory {
                println!("Name: {}", physical_memory_info.name);
                println!("Bank label: {}", physical_memory_info.bank_label);
                println!("Capacity: {} bytes", physical_memory_info.capacity);
                println!("Description: {}", physical_memory_info.description);
                println!("Device locator: {}", physical_memory_info.device_locator);
                println!("Form factor: {}", physical_memory_info.form_factor);
                println!("Interleave data depth: {}", physical_memory_info.interleave_data_depth);
                println!("Interleave position: {}", physical_memory_info.interleave_position);
                println!("Manufacturer: {}", physical_memory_info.manufacturer);
                println!("Memory type: {}", physical_memory_info.memory_type);
                println!("Serial number: {}", physical_memory_info.serial_number);
                println!("Speed: {}", physical_memory_info.speed);
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_sound" => {
            #[cfg(target_os = "windows")]
            for sound_info in &system_info.wmi_sound {
                println!("Name: {}", sound_info.name);
                println!("Manufacturer: {}", sound_info.manufacturer);
                println!("Status: {}", sound_info.status);
                println!("DMABufferSize: {}", sound_info.dma_buffer_size);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_video" => {
            #[cfg(target_os = "windows")]
            for video_info in &system_info.wmi_video {
                println!("Name: {}", video_info.name);
                println!("Adapter compatibility: {}", video_info.adapter_compatibility);
                println!("Adapter DAC Type: {}", video_info.adapter_dac_type);
                println!("Adapter RAM: {} GB", video_info.adapter_ram);
                println!("Availability: {}", video_info.availability);
                println!("Driver Version: {}", video_info.driver_version);
                println!("Installed Display Drivers: {:?}", video_info.installed_display_driver);
                println!("Refresh Rate: {} mhz", video_info.refresh_rate);
                println!("Video Mode Description: {}", video_info.screen_info);
                println!("Status: {}", video_info.status);
                println!("Video Architecture: {}", video_info.video_architecture);
                println!("Video Memory Type: {}", video_info.video_memory_type);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_monitors" => {
            #[cfg(target_os = "windows")]
            for monitor_info in &system_info.wmi_monitors {
                println!("Name: {}", monitor_info.name);
                println!("Availability: {}", monitor_info.availability);
                println!("Bandwidth: {}", monitor_info.bandwidth);
                println!("Screen Height: {}", monitor_info.screen_height);
                println!("Screen Width: {}", monitor_info.screen_width);
                println!("Manufacturer: {}", monitor_info.manufacturer);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_keyboard" => {
            for keyboard_info in &system_info.wmi_keyboard {
                println!("Name: {}", keyboard_info.name);
                println!("Description: {}", keyboard_info.description);
                println!("Device id: {}", keyboard_info.device_id);
                println!("Status: {}", keyboard_info.status);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "wmi_pointing_device" => {
            #[cfg(target_os = "windows")]
             for pointing_device_info in &system_info.wmi_pointing_device{
                println!("Name: {}", pointing_device_info.name);
                println!("Manufacturer: {}", pointing_device_info.manufacturer);
                println!("Description: {}", pointing_device_info.description);
                println!("Pointing type: {}", pointing_device_info.pointing_type);
                println!("Status: {}", pointing_device_info.status);
                println!("----------------------------------------------------------------");
            }
            #[cfg(not(windows))]
            println!("Not implemented!");
        }
        "process_open_sockets" => {
            #[cfg(target_os = "linux")]
                for entry in &system_info.process_open_sockets {
                println!("{:?}", entry);
            }
            #[cfg(target_os = "windows")] {
                for entry in &system_info.process_open_sockets {
                    println!("{:?}", entry);
                }
            }
            #[cfg(any(not(linux), not(windows)))]
                println!("Not implemented!");
        }
        "processes" => {
            #[cfg(target_os = "linux")]
                for process in &system_info.processes {
                println!("{:?}", &process);
                println!("");
            }
            #[cfg(target_os = "windows")]
                for process in &system_info.processes {
                println!("{:?}", process);
                println!("");
            }
            #[cfg(not(linux))]
                println!("Not implemented!");
        }
        "process_memory_map" => {
            #[cfg(target_os = "linux")]
                for pid in &system_info.process_memory_map {
                for entry in pid {
                    println!("{:?}", entry);
                }
            }
            #[cfg(target_os = "windows")]
                for pid in &system_info.process_memory_map {
                for entry in pid {
                    println!("{:?}", entry);
                }
            }
            #[cfg(any(not(linux), not(windows)))]
                println!("Not implemented!");
        }
        _ => {}
    }
}