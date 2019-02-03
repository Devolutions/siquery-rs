use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use tables::*;

fn bios() -> Element {
    let bios = WmiBios::get_specific();

    let mut parent = Element::new("Bios");

    for entry in bios {
        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("ReleaseDate");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("SMBIOSBIOSVersion");

        child_1.text = Some(entry.caption);
        child_2.text = Some(entry.manufacturer);
        child_3.text = Some(entry.release_date);
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.smbios_version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
    }

    parent
}

fn cd_roms() -> Element {
    let mut cd_roms = Element::new("CDRoms");

    cd_roms
}

fn devices() -> Element {
    let mut devices = Element::new("Devices");

    let mut keyboards = Element::new("Keyboards");
    let mut remote_keyboard = Element::new("RemoteKeyboard");
    let mut remote_keyboard_description = Element::new("Description");
    let mut remote_keyboard_name = Element::new("Name");

    remote_keyboard.children.push(remote_keyboard_description);
    remote_keyboard.children.push(remote_keyboard_name);
    keyboards.children.push(remote_keyboard);
    devices.children.push(keyboards);

    let mut pointing_devices = Element::new("PointingDevices");
    let mut remote_pointing_device = Element::new("RemotePointingDevice");
    let mut remote_pointing_device_description = Element::new("Description");
    let mut remote_pointing_device_manufacturer = Element::new("Manufacturer");
    let mut remote_pointing_device_name = Element::new("Name");

    remote_pointing_device.children.push(remote_pointing_device_description);
    remote_pointing_device.children.push(remote_pointing_device_manufacturer);
    remote_pointing_device.children.push(remote_pointing_device_name);
    pointing_devices.children.push(remote_pointing_device);
    devices.children.push(pointing_devices);

    devices
}

fn ip_address() -> Element {
    let mut ip_address = Element::new("IPAddress");

    ip_address
}

fn local_accounts() -> Element {
    let wmi_local_accounts = WmiLocalAccounts::get_specific();

    let mut local_accounts = Element::new("LocalAccounts");

    let mut remote_account = Element::new("RemoteAccount");
    let mut child_1 = Element::new("Caption");
    let mut child_2 = Element::new("Domain");
    let mut child_3 = Element::new("LocalAccount");
    let mut child_4 = Element::new("Name");
    let mut child_6 = Element::new("Status");

    for local_account in wmi_local_accounts {
        let mut child_5 = Element::new("SID");
        child_5.text = Some(local_account.sid);
        remote_account.children.push(child_5);
    }
    remote_account.children.push(child_1);
    remote_account.children.push(child_2);
    remote_account.children.push(child_3);
    remote_account.children.push(child_4);
    remote_account.children.push(child_6);
    local_accounts.children.push(remote_account);

    local_accounts
}

fn memory() -> Element {
    let mut memory = Element::new("Memory");

    let mut remote_memory = Element::new("RemoteMemory");
    let mut child_1 = Element::new("Capacity");
    let mut child_2 = Element::new("Description");
    let mut child_3 = Element::new("DeviceLocator");
    let mut child_4 = Element::new("FormFactor");
    let mut child_5 = Element::new("Manufacturer");
    let mut child_6 = Element::new("MemoryType");
    let mut child_7 = Element::new("Name");
    let mut child_8 = Element::new("SerialNumber");
    let mut child_9 = Element::new("VolumeSerialNumber");

    remote_memory.children.push(child_1);
    remote_memory.children.push(child_2);
    remote_memory.children.push(child_3);
    remote_memory.children.push(child_4);
    remote_memory.children.push(child_5);
    remote_memory.children.push(child_6);
    remote_memory.children.push(child_7);
    remote_memory.children.push(child_8);
    remote_memory.children.push(child_9);

    memory.children.push(remote_memory);

    memory
}

fn monitors() -> Element {
    let mut monitors = Element::new("Monitors");

    let mut remote_monitor = Element::new("RemoteMonitor");
    let mut child_1 = Element::new("Availability");
    let mut child_2 = Element::new("Manufacturer");
    let mut child_3 = Element::new("Name");
    let mut child_4 = Element::new("ScreenHeight");
    let mut child_5 = Element::new("ScreenWidth");

    remote_monitor.children.push(child_1);
    remote_monitor.children.push(child_2);
    remote_monitor.children.push(child_3);
    remote_monitor.children.push(child_4);
    remote_monitor.children.push(child_5);

    monitors.children.push(remote_monitor);

    monitors
}

fn motherboards() -> Element {
    let mut motherboards = Element::new("Motherboards");

    let mut remote_motherboards = Element::new("RemoteMotherboard");
    let mut child_1 = Element::new("Manufacturer");
    let mut child_2 = Element::new("Name");
    let mut child_3 = Element::new("Product");
    let mut child_4 = Element::new("SerialNumber");
    let mut child_5 = Element::new("Version");

    remote_motherboards.children.push(child_1);
    remote_motherboards.children.push(child_2);
    remote_motherboards.children.push(child_3);
    remote_motherboards.children.push(child_4);
    remote_motherboards.children.push(child_5);

    motherboards.children.push(remote_motherboards);

    motherboards
}

fn network_adapters() -> Element {
    let mut network_adapters = Element::new("NetworkAdapters");

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

    network_adapters
}

pub fn get_local_accounts_inv() /*-> String*/ {
    #[cfg(feature = "wmi_local_accounts")]
    let table = WmiLocalAccounts::get_specific();
    //return serialized table to xml here
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

pub fn get_printers_inv() /*-> String*/ {
    #[cfg(feature = "wmi_printers")]
    let table = WmiPrinters::get_specific();
    //return serialized table to xml here
}

pub fn get_products_inv() /*-> String*/ {
    #[cfg(feature = "products")]
    let table = Products::get_specific();
    //return serialized table to xml here
}

pub fn get_services_inv() /*-> String*/ {
    #[cfg(feature = "wmi_services")]
    let table = WmiServices::get_specific();
    //return serialized table to xml here
}

pub fn get_shares_inv() /*-> String*/ {
    #[cfg(feature = "wmi_shares")]
    let table = WmiShares::get_specific();
    //return serialized table to xml here
}

pub fn get_startup_inv() /*-> String*/ {
    /*No info*/
}

pub fn get_sysem_info_inv() /*-> String*/ {
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

pub fn get_hotfixes_inv() /*-> String*/ {
    #[cfg(feature = "wmi_hotfixes")]
    let table = WmiHotfixes::get_specific();
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
        get_printers_inv();
    }

    let logical_drives = "Products";
    let logical_drives_idx = query_string.find("Products");
    if let Some(_i) = logical_drives_idx {
        get_services_inv();
    }

    let logical_drives = "Services";
    let logical_drives_idx = query_string.find("Services");
    if let Some(_i) = logical_drives_idx {
        get_products_inv();
    }

    let logical_drives = "Shares";
    let logical_drives_idx = query_string.find("Shares");
    if let Some(_i) = logical_drives_idx {
        get_shares_inv();
    }

    let logical_drives = "Start Up";
    let logical_drives_idx = query_string.find("Start Up");
    if let Some(_i) = logical_drives_idx {
        get_startup_inv();
    }

    let logical_drives = "System Information";
    let logical_drives_idx = query_string.find("System Information");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
        get_sysem_info_inv();
    }

    let logical_drives = "Windows HotFixes";
    let logical_drives_idx = query_string.find("Windows HotFixes");
    if let Some(_i) = logical_drives_idx {
        get_hotfixes_inv();
    }

    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    println!("{}",doc.to_string());
}