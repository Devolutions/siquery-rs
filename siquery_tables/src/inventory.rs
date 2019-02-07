use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use tables::*;

use std::fs::File;
use std::io::prelude::*;

fn ip_address() -> Element {
    let mut ip_address = Element::new("IPAddress");

    ip_address
}

pub fn get_local_accounts_inv() {
    let table = WmiLocalAccounts::get_specific();
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
            child_2.text = Some(product.install_date);
            remote_product.children.push(child_2);
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

pub fn get_startup_inv() /*-> String*/ {
    /*No info*/
}

pub fn get_system_info_inv() /*-> String*/ {
    #[cfg(feature = "wmi_computer_info")]
    let table = WmiComputerInfo::get_specific();
    // serialize table to xml

    #[cfg(feature = "wmi_bios")]
    let table = WmiBios::get_specific();
    // append serialize table to xml string

    #[cfg(feature = "wmi_os_version")]
    let table = WmiOsVersion::get_specific();
    // append serialize table to xml string

    // rdm creates a subdivision of motherboards
    #[cfg(feature = "wmi_motherboard")]
    let table = WmiMotherboard::get_specific(); // change table name to baseboard
    // append serialize table to xml string

    #[cfg(feature = "wmi_processor")]
    let table = WmiProcessor::get_specific(); // to check
    // append serialize table to xml string

    #[cfg(feature = "wmi_physical_memory")]
    let table = WmiMemory::get_specific(); // to check
    // append serialize table to xml string

    #[cfg(feature = "wmi_sound")]
    let table = WmiSound::get_specific(); // to check
    // append serialize table to xml string

    #[cfg(feature = "wmi_video")]
    let table = WmiVideo::get_specific(); // to check
    // append serialize table to xml string

    #[cfg(feature = "wmi_keyboard")] // devices tree
    let table = WmiKeyboard::get_specific();
    // append serialize table to xml string

    #[cfg(feature = "wmi_pointing_device")] // devices tree
    let table = WmiPointingDevice::get_specific();
    // append serialize table to xml string
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

pub fn execute_inventory_query(query: &str) {
    let mut root = Element::new("InventorySystemInformation");

    let mut rdm_inv_queries: Vec<String> = Vec::new();
    let query_string = query.to_string();

    let local_accounts = "Local Accounts";
    let local_accounts_idx = query_string.find("Local Accounts");
    if let Some(_i) = local_accounts_idx {
        get_local_accounts_inv();
    }

    let logical_drives = "Logical Drives";
    let logical_drives_idx = query_string.find("Logical Drives");
    if let Some(_i) = logical_drives_idx {
        get_logical_drives_inv(&mut root);
    }

    let logical_drives = "Network Adapters";
    let logical_drives_idx = query_string.find("Network Adapters");
    if let Some(_i) = logical_drives_idx {
        get_network_adapters_inv(&mut root);
    }

    let logical_drives = "Printers";
    let logical_drives_idx = query_string.find("Printers");
    if let Some(_i) = logical_drives_idx {
        get_printers_inv(&mut root);
    }

    let logical_drives = "Products";
    let logical_drives_idx = query_string.find("Products");
    if let Some(_i) = logical_drives_idx {
        get_products_inv(&mut root);
    }

    let logical_drives = "Services";
    let logical_drives_idx = query_string.find("Services");
    if let Some(_i) = logical_drives_idx {
        get_services_inv(&mut root);
    }

    let logical_drives = "Shares";
    let logical_drives_idx = query_string.find("Shares");
    if let Some(_i) = logical_drives_idx {
        get_shares_inv(&mut root);
    }

    let logical_drives = "Start Up";
    let logical_drives_idx = query_string.find("Start Up");
    if let Some(_i) = logical_drives_idx {
        get_startup_inv();
    }

    let logical_drives = "System Information";
    let logical_drives_idx = query_string.find("System Information");
    if let Some(_i) = logical_drives_idx {
        get_system_info_inv();
    }

    let logical_drives = "Windows HotFixes";
    let logical_drives_idx = query_string.find("Windows HotFixes");
    if let Some(_i) = logical_drives_idx {
        get_hotfixes_inv(&mut root);
    }

    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    let mut file = File::create("inventory.inv").ok();
    file.unwrap().write_all(doc.to_string().as_str().as_bytes()).ok();

    //println!("{}",doc.to_string());
}