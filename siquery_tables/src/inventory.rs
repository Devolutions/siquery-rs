#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use chrono::{NaiveDate,NaiveDateTime};

use tables::*;

fn ip_address() -> Element {
    let mut ip_address = Element::new("IPAddress");

    ip_address
}

pub fn get_local_accounts_inv(ref mut root: &mut Element) {
    let wmi_local_accounts = WmiLocalAccounts::get_specific();
    let mut local_accounts = Element::new("LocalAccounts");
    for local_account in wmi_local_accounts {
        let mut remote_account = Element::new("RemoteAccount");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Description");
        let mut child_3 = Element::new("Domain");
        let mut child_4 = Element::new("LocalAccount");
        let mut child_5 = Element::new("Name");
        let mut child_6 = Element::new("SID");
        let mut child_7 = Element::new("Status");

        child_1.text = Some(local_account.caption);
        child_2.text = Some(local_account.description);
        child_3.text = Some(local_account._domain);
        child_4.text = Some(local_account.local_account.to_lowercase());
        child_5.text = Some(local_account.name);
        child_6.text = Some(local_account.sid);
        child_7.text = Some(local_account.status);

        remote_account.children.push(child_1);
        remote_account.children.push(child_2);
        remote_account.children.push(child_3);
        remote_account.children.push(child_4);
        remote_account.children.push(child_5);
        remote_account.children.push(child_6);
        remote_account.children.push(child_7);

        local_accounts.children.push(remote_account);
    }
    root.children.push(local_accounts);
}

pub fn get_logical_drives_inv(ref mut root: &mut Element) {

    let wmi_logical_drives = LogicalDrive::get_specific();
    let mut logical_drives = Element::new("LogicalDrives");
    for logical_drive in wmi_logical_drives {
        let mut remote_logical_disk = Element::new("RemoteLogicalDisk");

        let mut child_1 = Element::new("Description");
        let mut child_2 = Element::new("DriveType");
        let mut child_3 = Element::new("FileSystem");
        let mut child_4 = Element::new("FreeSpace");
        let mut child_5 = Element::new("MaximumComponentLength");
        let mut child_6 = Element::new("Name");
        let mut child_7 = Element::new("Size");
        let mut child_8 = Element::new("SupportsFileBasedCompression");
        let mut child_9 = Element::new("VolumeSerialNumber");

        child_1.text = Some(logical_drive.description);
        child_2.text = Some(logical_drive.drive_type);
        child_3.text = Some(logical_drive.file_system);
        child_4.text = Some(logical_drive.free_space.to_string());
        child_5.text = Some(logical_drive.maximum_component_length.to_string());
        child_6.text = Some(logical_drive.name);
        child_7.text = Some(logical_drive.size.to_string());
        child_8.text = Some(logical_drive.supports_file_based_compression.to_string());
        child_9.text = Some(logical_drive.volume_serial_number);

        remote_logical_disk.children.push(child_1);
        remote_logical_disk.children.push(child_2);
        remote_logical_disk.children.push(child_3);
        remote_logical_disk.children.push(child_4);
        remote_logical_disk.children.push(child_5);
        remote_logical_disk.children.push(child_6);
        remote_logical_disk.children.push(child_7);
        remote_logical_disk.children.push(child_8);
        remote_logical_disk.children.push(child_9);

        logical_drives.children.push(remote_logical_disk);
    }
    root.children.push(logical_drives);
}

pub fn get_network_adapters_inv(ref mut root: &mut Element) {
    let wmi_network_adapters = WmiNetworkAdapters::get_specific();
    let mut network_adapters = Element::new("NetworkAdapters");
    for network_adapter in wmi_network_adapters {
        if (network_adapter.ip_enabled == "true") {
            let mut remote_network_adapters = Element::new("RemoteNetworkAdapter");
            let mut child_1 = Element::new("Ports");
            let mut child_2 = Element::new("DatabasePath");
            let mut child_3 = Element::new("Description");
            let mut child_4 = Element::new("DHCPEnabled");
            let mut child_5 = Element::new("IPAddress");
            let mut child_6 = Element::new("IPAddressMac");
            let mut child_7 = Element::new("IPEnabled");
            let mut child_8 = Element::new("IPSubnet");
            let mut child_9 = Element::new("MACAddress");

            child_1.text = Some("".to_string()); // no ports in wmi call this field is always empty
            child_2.text = Some(network_adapter.database_path);
            child_3.text = Some(network_adapter.description);
            child_4.text = Some(network_adapter.dhcp_enabled);

            for ip_addr in network_adapter.ip_address.iter() {
                let mut sub_child = Element::new("string");
                sub_child.text = Some((*ip_addr).to_owned());
                child_5.children.push(sub_child);
            }

            child_6.text = Some("".to_string()); // no ports in wmi call this field is always empty
            child_7.text = Some(network_adapter.ip_enabled);

            for ip_subnet in network_adapter.ip_subnet.iter() {
                let mut sub_child = Element::new("string");
                sub_child.text = Some((*ip_subnet).to_owned());
                child_8.children.push(sub_child);
            }

            child_9.text = Some(network_adapter.mac_address);

            remote_network_adapters.children.push(child_1);
            remote_network_adapters.children.push(child_2);
            remote_network_adapters.children.push(child_3);
            remote_network_adapters.children.push(child_4);
            remote_network_adapters.children.push(child_5);
            remote_network_adapters.children.push(child_6);
            remote_network_adapters.children.push(child_7);
            remote_network_adapters.children.push(child_8);
            remote_network_adapters.children.push(child_9);

            network_adapters.children.push(remote_network_adapters);
        }
    }
    root.children.push(network_adapters);
}

pub fn get_printers_inv(ref mut root: &mut Element) {
    let wmi_printers = WmiPrinters::get_specific();
    let mut printers = Element::new("Printers");
    for printer in wmi_printers {
        let mut remote_printer = Element::new("RemotePrinter");

        let mut child_1 = Element::new("Attributes");
        let mut child_2 = Element::new("Caption");
        let mut child_3 = Element::new("CreationClassName");
        let mut child_4 = Element::new("DeviceID");
        let mut child_5 = Element::new("DoCompleteFirst");
        let mut child_6 = Element::new("DriverName");
        let mut child_7 = Element::new("ExtendedPrinterStatus");
        let mut child_8 = Element::new("HorizontalResolution");
        let mut child_9 = Element::new("Local");
        let mut child_10 = Element::new("Name");
        let mut child_11 = Element::new("PortName");
        let mut child_12 = Element::new("PrinterStatus");
        let mut child_13 = Element::new("PrintJobDataType");
        let mut child_14 = Element::new("PrintProcessor");
        let mut child_15 = Element::new("Priority");
        let mut child_16 = Element::new("Status");
        let mut child_17 = Element::new("SystemCreationClassName");
        let mut child_18 = Element::new("SystemName");
        let mut child_19 = Element::new("VerticalResolution");

        child_1.text = Some(printer.attributes.to_string());
        child_2.text = Some(printer.caption);
        child_3.text = Some(printer.creation_class_name);
        child_4.text = Some(printer.device_id);
        child_5.text = Some(printer.do_complete_first);
        child_6.text = Some(printer.driver_name);
        child_7.text = Some(printer.extended_printer_status.to_string());
        child_8.text = Some(printer.horizontal_resolution.to_string());
        child_9.text = Some(printer.local);
        child_10.text = Some(printer.name);
        child_11.text = Some(printer.port_name);
        child_12.text = Some(printer.printer_status.to_string());
        child_13.text = Some(printer.print_job_data_type);
        child_14.text = Some(printer.print_processor);
        child_15.text = Some(printer.priority.to_string());
        child_16.text = Some(printer.status);
        child_17.text = Some(printer.system_creation_class_name);
        child_18.text = Some(printer.system_name);
        child_19.text = Some(printer.vertical_resolution.to_string());

        remote_printer.children.push(child_1);
        remote_printer.children.push(child_2);
        remote_printer.children.push(child_3);
        remote_printer.children.push(child_4);
        remote_printer.children.push(child_5);
        remote_printer.children.push(child_6);
        remote_printer.children.push(child_7);
        remote_printer.children.push(child_8);
        remote_printer.children.push(child_9);
        remote_printer.children.push(child_10);
        remote_printer.children.push(child_11);
        remote_printer.children.push(child_12);
        remote_printer.children.push(child_13);
        remote_printer.children.push(child_14);
        remote_printer.children.push(child_15);
        remote_printer.children.push(child_16);
        remote_printer.children.push(child_17);
        remote_printer.children.push(child_18);
        remote_printer.children.push(child_19);

        printers.children.push(remote_printer);
    }
    root.children.push(printers);
}

pub fn get_products_inv(ref mut root: &mut Element) {

    let wmi_products = WmiProducts::get_specific();
    let mut products = Element::new("Products");
    for product in wmi_products {
        let mut remote_product = Element::new("RemoteProgram");

        let mut child_1 = Element::new("HelpLink");
        let mut child_2 = Element::new("InstallDate");
        let mut child_3 = Element::new("InstallLocation");
        let mut child_4 = Element::new("Name");
        let mut child_5 = Element::new("Vendor");
        let mut child_6 = Element::new("Version");

        if product.help_link != "" {
            child_1.text = Some(product.help_link);
            remote_product.children.push(child_1);
        }
        if product.install_date != "" {
            let mut install_date = product.install_date.clone();
            if install_date.len() >= 14 {
                install_date.truncate(14);
                if let Ok(date) = NaiveDateTime::parse_from_str(
                    &install_date, "%Y%m%d%H%M%S") {
                    child_2.text  = Some(date.format("%Y-%m-%dT%H:%M:%S").to_string());
                }
                remote_product.children.push(child_2);
            }
        }
        if product.install_location != "" {
            child_3.text = Some(product.install_location);
            remote_product.children.push(child_3);
        }
        if product.name != "" {
            child_4.text = Some(product.name);
            remote_product.children.push(child_4);
        }
        if product.vendor != "" {
            child_5.text = Some(product.vendor);
            remote_product.children.push(child_5);
        }
        if product.version != "" {
            child_6.text = Some(product.version);
            remote_product.children.push(child_6);
        }

        products.children.push(remote_product);
    }
    root.children.push(products);
}

pub fn get_services_inv(ref mut root: &mut Element) {
    let wmi_services = WmiServices::get_specific();
    let mut services = Element::new("Services");
    for service in wmi_services {
        let mut remote_service = Element::new("RemoteService");

        let mut child_1 = Element::new("AcceptPause");
        let mut child_2 = Element::new("AcceptStop");
        let mut child_3 = Element::new("Caption");
        let mut child_4 = Element::new("CreationClassName");
        let mut child_5 = Element::new("Description");
        let mut child_6 = Element::new("DesktopInteract");
        let mut child_7 = Element::new("DisplayName");
        let mut child_8 = Element::new("ErrorControl");
        let mut child_9 = Element::new("ExitCode");
        let mut child_10 = Element::new("Name");
        let mut child_11 = Element::new("PathName");
        let mut child_12 = Element::new("ServiceType");
        let mut child_13 = Element::new("Started");
        let mut child_14 = Element::new("StartMode");
        let mut child_15 = Element::new("StartName");
        let mut child_16 = Element::new("State");
        let mut child_17 = Element::new("Status");
        let mut child_18 = Element::new("SystemCreationClassName");
        let mut child_19 = Element::new("SystemName");

        child_1.text = Some(service.accept_pause);
        child_2.text = Some(service.accept_stop);
        child_3.text = Some(service.caption);
        child_4.text = Some(service.creation_class_name);
        child_5.text = Some(service.description);
        child_6.text = Some(service.desktop_interact);
        child_7.text = Some(service.display_name);
        child_8.text = Some(service.error_control);
        child_9.text = Some(service.exit_code.to_string());
        child_10.text = Some(service.name);
        child_11.text = Some(service.path_name);
        child_12.text = Some(service.service_type);
        child_13.text = Some(service.started);
        child_14.text = Some(service.start_mode);
        child_15.text = Some(service.start_name);
        child_16.text = Some(service.state);
        child_17.text = Some(service.status);
        child_18.text = Some(service.system_creation_class_name);
        child_19.text = Some(service.system_name);

        remote_service.children.push(child_1);
        remote_service.children.push(child_2);
        remote_service.children.push(child_3);
        remote_service.children.push(child_4);
        remote_service.children.push(child_5);
        remote_service.children.push(child_6);
        remote_service.children.push(child_7);
        remote_service.children.push(child_8);
        if service.exit_code != 0 {
            remote_service.children.push(child_9);
        }
        remote_service.children.push(child_10);
        remote_service.children.push(child_11);
        remote_service.children.push(child_12);
        remote_service.children.push(child_13);
        remote_service.children.push(child_14);
        remote_service.children.push(child_15);
        remote_service.children.push(child_16);
        remote_service.children.push(child_17);
        remote_service.children.push(child_18);
        remote_service.children.push(child_19);

        services.children.push(remote_service);
    }
    root.children.push(services);
}

pub fn get_shares_inv(ref mut root: &mut Element) {
    let wmi_shares = WmiShares::get_specific();
    let mut shares = Element::new("Shares");
    for share in wmi_shares {
        let mut remote_share = Element::new("RemoteShare");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Description");
        let mut child_3 = Element::new("Name");
        let mut child_4 = Element::new("Path");
        let mut child_5 = Element::new("Status");
        let mut child_6 = Element::new("Type");

        child_1.text = Some(share.caption);
        child_2.text = Some(share.description);
        child_3.text = Some(share.name);
        child_5.text = Some(share.status);
        child_6.text = Some(share._type);

        remote_share.children.push(child_1);
        remote_share.children.push(child_2);
        remote_share.children.push(child_3);
        if share.path != "" {
            child_4.text = Some(share.path);
            remote_share.children.push(child_4);
        }
        remote_share.children.push(child_5);
        remote_share.children.push(child_6);

        shares.children.push(remote_share);
    }
    root.children.push(shares);
}

pub fn get_start_up_inv(ref mut root: &mut Element) {
    let wmi_start_ups = WmiStartUp::get_specific();
    let mut start_ups = Element::new("StartUps");
    for start_up in wmi_start_ups {
        let mut remote_start_up = Element::new("RemoteStartUp");

        let mut child_1 = Element::new("Command");
        let mut child_2 = Element::new("Location");
        let mut child_3 = Element::new("Name");
        let mut child_4 = Element::new("User");

        child_1.text = Some(start_up.command);
        child_2.text = Some(start_up.location);
        child_3.text = Some(start_up.name);
        child_4.text = Some(start_up.user);

        remote_start_up.children.push(child_1);
        remote_start_up.children.push(child_2);
        remote_start_up.children.push(child_3);
        remote_start_up.children.push(child_4);

        start_ups.children.push(remote_start_up);
    }
    root.children.push(start_ups);
}

pub fn get_hotfixes_inv(ref mut root: &mut Element) {
    let wmi_hotfixes = WmiHotfixes::get_specific();
    let mut hotfixes = Element::new("QuickFixEngineerings");
    for hotfix in wmi_hotfixes {
        let mut remote_hotfix = Element::new("RemoteQuickFixEngineering");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("CSName");
        let mut child_3 = Element::new("Description");
        let mut child_4 = Element::new("HotFixID");
        let mut child_5 = Element::new("InstalledBy");
        let mut child_6 = Element::new("InstalledOn");

        child_1.text = Some(hotfix.caption);
        child_2.text = Some(hotfix.csname);
        child_3.text = Some(hotfix.description);
        child_4.text = Some(hotfix.hotfix_id);
        child_5.text = Some(hotfix.installed_by);
        child_6.text = Some(hotfix.installed_on);

        remote_hotfix.children.push(child_1);
        remote_hotfix.children.push(child_2);
        remote_hotfix.children.push(child_3);
        remote_hotfix.children.push(child_4);
        remote_hotfix.children.push(child_5);
        remote_hotfix.children.push(child_6);

        hotfixes.children.push(remote_hotfix);
    }
    root.children.push(hotfixes);
}

pub fn get_system_info_inv(ref mut root: &mut Element) {
    for entry in WmiComputerInfo::get_specific(){
        let mut parent = Element::new("System");

        let mut child_1 = Element::new("Domain");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("Model");
        let mut child_4 = Element::new("NumberOfProcessors");
        let mut child_5 = Element::new("SystemType");

        child_1.text = Some(entry.domain);
        child_2.text = Some(entry.manufacturer);
        child_3.text = Some(entry.model);
        child_4.text = Some(entry.number_of_processors.to_string());
        child_5.text = Some(entry.system_type);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        root.children.push(parent);
    }

    for entry in WmiBios::get_specific() {
        let mut parent = Element::new("Bios");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("ReleaseDate");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("SMBIOSBIOSVersion");

        child_1.text = Some(entry.caption);
        child_2.text = Some(entry.manufacturer);
        if let Ok(date) = NaiveDate::parse_from_str(
            &entry.release_date, "%Y%m%d") {
            child_3.text = Some(date.to_string());
        }
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.smbios_version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        root.children.push(parent);
    }

    for entry in WmiOsVersion::get_specific(){
        let mut parent = Element::new("OperatingSystem");

        let mut child_1  = Element::new("BuildNumber");
        let mut child_2  = Element::new("CSName");
        let mut child_3  = Element::new("FreePhysicalMemory");
        let mut child_4  = Element::new("FreeVirtualMemory");
        let mut child_5  = Element::new("InstallDate");
        let mut child_6  = Element::new("LastBootUpTime");
        let mut child_7  = Element::new("Locale");
        let mut child_8  = Element::new("Manufacturer");
        let mut child_9  = Element::new("Name");
        let mut child_10 = Element::new("OSType");
        let mut child_11 = Element::new("SizeStoredInPagingFiles");
        let mut child_12 = Element::new("TotalVirtualMemorySize");
        let mut child_13 = Element::new("TotalVisibleMemorySize");
        let mut child_14 = Element::new("Version");
        let mut child_15 = Element::new("WindowsDirectory");

        child_1.text  = Some(entry.build_number);
        child_2.text  = Some(entry.csname);
        child_3.text  = Some(entry.free_physical_mem);
        child_4.text  = Some(entry.free_virtual_mem);

        let mut install_date = entry.install_date.clone();
        if install_date.len() >= 14 {
            install_date.truncate(14);
            if let Ok(date) = NaiveDateTime::parse_from_str(
                &install_date, "%Y%m%d%H%M%S") {
                child_5.text = Some(date.format("%Y-%m-%dT%H:%M:%S").to_string());
            }
        }
        let mut last_boot_up_time = entry.last_boot_up_time.clone();
        if last_boot_up_time.len() >= 14 {
            last_boot_up_time.truncate(14);
            if let Ok(date) = NaiveDateTime::parse_from_str(
                &last_boot_up_time, "%Y%m%d%H%M%S") {
                child_6.text = Some(date.format("%Y-%m-%dT%H:%M:%S").to_string());
            }
        }

        child_7.text  = Some(entry.locale);
        child_8.text  = Some(entry.manufacturer);
        child_9.text  = Some(entry.name);
        child_10.text = Some(entry.os_type);
        child_11.text = Some(entry.size_stored_in_paging_file);
        child_12.text = Some(entry.total_virtual_mem_size);
        child_13.text = Some(entry.total_visible_mem_size);
        child_14.text = Some(entry.version);
        child_15.text = Some(entry.win_directory);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);
        parent.children.push(child_13);
        parent.children.push(child_14);
        parent.children.push(child_15);

        root.children.push(parent);
    }

    let mut parent_motherboards = Element::new("Motherboards");
    for entry in WmiMotherboard::get_specific() {
        let mut parent = Element::new("RemoteMotherboard");

        let mut child_1 = Element::new("Manufacturer");
        let mut child_2 = Element::new("Name");
        let mut child_3 = Element::new("Product");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("Version");

        child_1.text = Some(entry.manufacturer);
        child_2.text = Some(entry.name);
        child_3.text = Some(entry.product);
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        parent_motherboards.children.push(parent);
    }
    root.children.push(parent_motherboards);

    let mut parent_processors = Element::new("Processors");
    for entry in WmiProcessor::get_specific() {
        let mut parent = Element::new("RemoteProcessor");

        let mut child_1  = Element::new("AddressWidth");
        let mut child_2  = Element::new("Architecture");
        let mut child_3  = Element::new("CpuStatus");
        let mut child_4  = Element::new("CurrentClockSpeed");
        let mut child_5  = Element::new("CurrentVoltage");
        let mut child_6  = Element::new("Description");
        let mut child_7  = Element::new("ExtClock");
        let mut child_8  = Element::new("L2CacheSize");
        let mut child_9  = Element::new("L3CacheSize");
        let mut child_10 = Element::new("Manufacturer");
        let mut child_11 = Element::new("MaxClockSpeed");
        let mut child_12 = Element::new("Name");
        let mut child_13 = Element::new("NumberOfCores");
        let mut child_14 = Element::new("NumberOfLogicalProcessors");
        let mut child_15 = Element::new("SocketDesignation");

        child_1.text  = Some(entry.address_width.to_string());
        child_2.text  = Some(entry.architecture);
        child_3.text  = Some(entry.cpu_satus);
        child_4.text  = Some(entry.current_clock_speed.to_string());
        child_5.text  = Some(entry.current_voltage.to_string());
        child_6.text  = Some(entry.description);
        child_7.text  = Some(entry.external_clock.to_string());
        child_8.text  = Some(entry.l2_cache_size.to_string());
        child_9.text  = Some(entry.l3_cache_size.to_string());
        child_10.text = Some(entry.manufacturer);
        child_11.text = Some(entry.max_clock_speed.to_string());
        child_12.text = Some(entry.name);
        child_13.text = Some(entry.number_of_cores.to_string());
        child_14.text = Some(entry.number_of_logical_processors.to_string());
        child_15.text = Some(entry.socket_designation);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);
        parent.children.push(child_13);
        parent.children.push(child_14);
        parent.children.push(child_15);

        parent_processors.children.push(parent);
    }
    root.children.push(parent_processors);

    let mut parent_memory = Element::new("Memory");
    for entry in WmiMemory::get_specific() {
        let mut parent = Element::new("RemoteMemory");

        let mut child_1 = Element::new("Capacity");
        let mut child_2 = Element::new("Description");
        let mut child_3 = Element::new("DeviceLocator");
        let mut child_4 = Element::new("FormFactor");
        let mut child_5 = Element::new("InterleaveDataDepth");
        let mut child_6 = Element::new("InterleavePosition");
        let mut child_7 = Element::new("Manufacturer");
        let mut child_8 = Element::new("MemoryType");
        let mut child_9 = Element::new("Name");
        let mut child_10 = Element::new("SerialNumber");
        let mut child_11 = Element::new("Speed");

        child_1.text = Some(entry.capacity);
        child_2.text = Some(entry.description);
        child_3.text = Some(entry.device_locator);
        child_4.text = Some(entry.form_factor.to_string());
        child_5.text = Some(entry.interleave_data_depth.to_string());
        child_6.text = Some(entry.interleave_position.to_string());
        child_7.text = Some(entry.manufacturer);
        child_8.text = Some(entry.memory_type.to_string());
        child_9.text = Some(entry.name);
        child_10.text = Some(entry.serial_number);
        child_11.text = Some(entry.speed.to_string());

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);

        parent_memory.children.push(parent);
    }
    root.children.push(parent_memory);

    let mut parent_sound = Element::new("SoundDevice");
    for entry in WmiSound::get_specific() {
        let mut parent = Element::new("RemoteSound");

        let mut child_1  = Element::new("Manufacturer");
        let mut child_2  = Element::new("Name");

        child_1.text  = Some(entry.manufacturer   );
        child_2.text  = Some(entry.name           );

        parent.children.push(child_1);
        parent.children.push(child_2);

        parent_sound.children.push(parent);
    }
    root.children.push(parent_sound);

    let mut parent_video = Element::new("Videos");
    for entry in WmiVideo::get_specific() {
        let mut parent = Element::new("RemoteVideo");

        let mut child_1 = Element::new("AdapterCompatibility");
        let mut child_2 = Element::new("AdapterDACType");
        let mut child_3 = Element::new("AdapterRam");
        let mut child_4 = Element::new("Availability");
        let mut child_5 = Element::new("DriverVersion");
        let mut child_6 = Element::new("InstalledDisplayDrivers");
        let mut child_7 = Element::new("Name");
        let mut child_8 = Element::new("RefreshRate");
        let mut child_9 = Element::new("ScreenInfo");
        let mut child_10 = Element::new("Status");
        let mut child_11 = Element::new("VideoArchitecture");
        let mut child_12 = Element::new("VideoMemoryType");

        child_1.text = Some(entry.adapter_compatibility);
        child_2.text = Some(entry.adapter_dac_type);
        child_3.text = Some(entry.adapter_ram.to_string());
        child_4.text = Some(entry.availability);
        child_5.text = Some(entry.driver_version);
        for driver in entry.installed_display_driver {
            let mut string = Element::new("string");
            string.text = Some(driver);
            child_6.children.push(string);
        }
        child_7.text = Some(entry.name);
        child_8.text = Some(entry.refresh_rate);
        child_9.text = Some(entry.screen_info);
        child_10.text = Some(entry.status);
        child_11.text = Some(entry.video_architecture);
        child_12.text = Some(entry.video_memory_type);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);

        parent_video.children.push(parent);
    }
    root.children.push(parent_video);

    let mut parent_devices = Element::new("Devices");
    let mut parent_keyboards = Element::new("Keyboards");
    for entry in WmiKeyboard::get_specific() {
        let mut parent = Element::new("RemoteKeyboard");

        let mut child_1  = Element::new("Description");
        let mut child_2  = Element::new("Name");

        child_1.text  = Some(entry.description);
        child_2.text  = Some(entry.name);

        parent.children.push(child_1);
        parent.children.push(child_2);

        parent_keyboards.children.push(parent);
    }
    parent_devices.children.push(parent_keyboards);

    let mut parent_pointing_dev = Element::new("PointingDevices");
    for entry in WmiPointingDevice::get_specific() {
        let mut parent = Element::new("RemotePointingDevice");

        let mut child_1  = Element::new("Description");
        let mut child_2  = Element::new("Manufacturer");
        let mut child_3  = Element::new("Name");

        child_1.text  = Some(entry.description);
        child_2.text  = Some(entry.manufacturer);
        child_2.text  = Some(entry.name);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);

        parent_pointing_dev.children.push(parent);
    }
    parent_devices.children.push(parent_pointing_dev);
    root.children.push(parent_devices);
}

pub fn get_time_zone(ref mut root: &mut Element) {
    let wmi_time_zone = WmiTimeZone::get_specific();
    let mut time_zone_tree = Element::new("TimeZone");

    for time_zone in wmi_time_zone {
        let mut child_1 = Element::new("Description");
        child_1.text = Some(time_zone.description);
        time_zone_tree.children.push(child_1);
    }
    root.children.push(time_zone_tree);
}

pub fn execute_inventory_query() {
    let mut root = Element::new("InventorySystemInformation");

    get_local_accounts_inv(&mut root);
    get_logical_drives_inv(&mut root);
    get_network_adapters_inv(&mut root);
    get_printers_inv(&mut root);
    get_products_inv(&mut root);
    get_services_inv(&mut root);
    get_shares_inv(&mut root);
    get_start_up_inv(&mut root);
    get_system_info_inv(&mut root);
    get_hotfixes_inv(&mut root);
    get_local_accounts_inv(&mut root);
    get_time_zone(&mut root);

    let doc = Document {
        root: Some(root),
        version: Version10,
        ..Document::default()
    };

    println!("{}", doc.to_string().as_str());
}