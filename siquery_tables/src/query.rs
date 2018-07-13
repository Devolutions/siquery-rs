
pub use sys::{SystemInfo, SystemReader};
use sys::{SystemReaderInterface};

use std::borrow::Borrow;

use tables::*;

fn select_all<T>(table: &Vec<T>) -> Vec<Vec<String>> where T:Table+Sized {
    let mut res: Vec<Vec<String>> = Vec::new();

    let cols = table.column_names();

    let mut id: u64 = 1;

    let mut hdr: Vec<String> = Vec::new();
    let mut columns_id: Vec<u64> = Vec::new();

    for col in cols.iter() {
        columns_id.push(id);
        hdr.push(col.to_string());
        id = id << 1;
    }

    res.push(hdr); // FIXME: store header elsewhere

    for tab in table.iter() {
        let mut row: Vec<String> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

fn select<T>(table: &Vec<T>, columns: Vec<String>) -> Vec<Vec<String>> where T:Table+Sized {

    if columns.len() < 1 {
        return select_all(table);
    }

    let mut res: Vec<Vec<String>> = Vec::new();

    let mut hdr: Vec<String> = Vec::new();
    let mut columns_id: Vec<u64> = Vec::new();

    for column in columns.iter() {
        // make sure the header exist in the table
        let id = table[0].get_id(column);
        if id != 0 {
            columns_id.push(id);
            hdr.push(column.to_string());
        }
    }

    res.push(hdr); // FIXME: store header elsewhere

    for tab in table.iter() {
        let mut row: Vec<String> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

pub fn query_table(name: &str, columns: Vec<String>) -> Vec<Vec<String>> {
    let system_reader: Box<SystemReaderInterface> = Box::new(SystemReader::new());
    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "etc_services" => {
            let table = EtcServices::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "system_info" => {
            let table = SystemInfoData::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "os_version" => {
            let table = OsVersion::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "logical_drives" => {
            let table = LogicalDrive::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_address" => {
            let table = InterfaceAddress::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_details" => {
            let table = InterfaceDetails::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "uptime" => {
            let table = Uptime::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "products" => {
            let table = Products::get_products_info();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => {
            let wmi_computer_info = WmiComputerInfo::get_system_info(system_reader.borrow());
            let mut table: Vec<WmiComputerInfo> = Vec::new();
            table.push(wmi_computer_info);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_os_version" => {
            let wmi_os_version = WmiOsVersion::new(system_reader.borrow());
            let mut table: Vec<WmiOsVersion> = Vec::new();
            table.push(wmi_os_version);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_printers" => {
            let table = WmiPrinters::get_printers_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_services" => {
            let table = WmiServices::get_services_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_hotfixes_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_shares" => {
            let table = WmiShares::get_shares_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_netwok_adapters_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_local_accounts_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_bios" => {
            let wmi_bios = WmiBios::get_bios_info(system_reader.borrow());
            let mut table: Vec<WmiBios> = Vec::new();
            table.push(wmi_bios);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => {
            let wmi_motherboard = WmiMotherboard::get_motherboard_info(system_reader.borrow());
            let mut table: Vec<WmiMotherboard> = Vec::new();
            table.push(wmi_motherboard);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_processor" => {
            let wmi_processor = WmiProcessor::get_processor_info(system_reader.borrow());
            let mut table: Vec<WmiProcessor> = Vec::new();
            table.push(wmi_processor);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_physical_memory_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_sound" => {
            let table = WmiSound::get_sound_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_video" => {
            let table = WmiVideo::get_video_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_monitors_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_keyboard_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_pointing_device_info(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "processes" => {
            let table = ProcessesRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "windows"))]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            select(&table, columns)
        }
    };
    res
}
