use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use tables::*;

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

pub fn execute_inventory_query(query: &str) {
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
        get_local_drives_inv();
    }

    let logical_drives = "Network Adapters";
    let logical_drives_idx = query_string.find("Network Adapters");
    if let Some(_i) = logical_drives_idx {
        get_network_adapters_inv();
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
}