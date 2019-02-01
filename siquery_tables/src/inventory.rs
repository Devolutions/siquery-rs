use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use tables::*;

/*pub fn print_xml () {
    // RDM Structures.
    let mut root = Element::new("InventorySystemInformation");
    /*let mut bios = bios();
    let mut cd_roms = cd_roms();
    let mut devices = devices();
    let mut ip_address = ip_address();
    let mut local_accounts = local_accounts();
    let mut logical_drives = logical_drives();
    let mut memory = memory();
    let mut monitors = monitors();
    let mut motherobaords = motherboards();
    let mut network_adapters = network_adapters();*/








    let mut operating_system = Element::new("OperatingSystem");

    let mut printers = Element::new("Printers");
    let mut remote_printer = Element::new("RemotePrinter");

    let mut processors = Element::new("Processors");
    let mut remote_processors = Element::new("RemoteProcessors");

    let mut products = Element::new("Products");
    let mut remote_program = Element::new("RemoteProgram");

    let mut quick_fix_engineerings = Element::new("QuickFixEngineerings");
    let mut remote_quick_fix_engineering = Element::new("RemoteQuickFixEngineering");

    let mut services = Element::new("Services");
    let mut remote_service = Element::new("RemoteService");

    let mut shares = Element::new("Shares");
    let mut remote_share = Element::new("RemoteShare");

    let mut sound_devices = Element::new("SoundDevice");
    let mut remote_sound = Element::new("RemoteSound");

    let mut start_ups = Element::new("StartUps");
    let mut remote_start_up = Element::new("RemoteStartUp");

    let mut system = Element::new("System");

    let mut time_zone = Element::new("TimeZone");
    let mut description = Element::new("Description");

    let mut videos = Element::new("Videos");
    let mut remote_video = Element::new("RemoteVideos");



    root.children.push(local_accounts);
    root.children.push(bios);
    root.children.push(cd_roms);
    root.children.push(devices);
    root.children.push(ip_address);
    //root.children.push(local_accounts);
    root.children.push(logical_drives);
    root.children.push(memory);
    root.children.push(monitors);
    root.children.push(motherobaords);
    root.children.push(network_adapters);

    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    println!("{}",doc.to_string());
}*/

/*fn bios() -> Element {
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
}*/


pub fn get_local_accounts_inv() /*-> String*/ {
    #[cfg(feature = "wmi_local_accounts")]
    let table = WmiLocalAccounts::get_specific();
    //return serialized table to xml here
}

pub fn get_local_drives_inv() /*-> String*/ {
    #[cfg(feature = "logical_drives")]
    let table = LogicalDrive::get_specific();
    //return serialized table to xml here
}

pub fn get_network_adapters_inv() /*-> String*/ {
    #[cfg(feature = "wmi_network_adapters")]
    let table = WmiNetworkAdapters::get_specific();
    //return serialized table to xml here
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
    println!("{:?}", table[0]);
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

pub fn execute_inventory_query(query: &str) -> Vec<String> {
    let mut rdm_inv_queries: Vec<String> = Vec::new();
    let query_string = query.to_string();

    let local_accounts = "Local Accounts";
    let local_accounts_idx = query_string.find("Local Accounts");
    if let Some(_i) = local_accounts_idx {
        rdm_inv_queries.push(local_accounts.to_string());
    }

    let logical_drives = "Logical Drives";
    let logical_drives_idx = query_string.find("Logical Drives");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Network Adapters";
    let logical_drives_idx = query_string.find("Network Adapters");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Printers";
    let logical_drives_idx = query_string.find("Printers");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Products";
    let logical_drives_idx = query_string.find("Products");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Services";
    let logical_drives_idx = query_string.find("Services");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Shares";
    let logical_drives_idx = query_string.find("Shares");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
    }

    let logical_drives = "Start Up";
    let logical_drives_idx = query_string.find("Start Up");
    if let Some(_i) = logical_drives_idx {
        rdm_inv_queries.push(logical_drives.to_string());
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
        println!("{:?}", rdm_inv_queries.push(logical_drives.to_string()));
        //print_xml();
    }

    rdm_inv_queries
}