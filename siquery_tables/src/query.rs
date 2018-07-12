
pub use sys::{SystemInfo, SystemReader};
use sys::{SystemReaderInterface};

use std::borrow::Borrow;

use tables::{
    Table,
    Dummy,
    EtcHosts,
    EtcProtocols,
    EtcServices,
    OsVersion,
    LogicalDrive,
    SystemInfoData,
    InterfaceAddress,
    InterfaceDetails,
    Uptime,
    Products,
    WmiComputerInfo,
    WmiOsVersion,
    WmiPrinters,
    WmiServices,
    WmiHotfixes,
    WmiShares,
    WmiNetworkAdapters,
    WmiBios,
    WmiLocalAccounts,
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
    let mut res: Vec<Vec<String>> = Vec::new();

    let mut hdr: Vec<String> = Vec::new();
    let mut columns_id: Vec<u64> = Vec::new();

    for column in columns.iter() {
        // make sur the header exist in the table
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
            let table = EtcHosts::get_hosts(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_protocols(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "etc_services" => {
            let table = EtcServices::get_services(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "system_info" => {
            let mut system_info_data = SystemInfoData::new();
            system_info_data.update(system_reader.borrow());

            let mut table: Vec<SystemInfoData> = Vec::new();
            table.push(system_info_data);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "os_version" => {
            let os_version = OsVersion::new(system_reader.borrow());

            let mut table: Vec<OsVersion> = Vec::new();
            table.push(os_version);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "logical_drives" => {
            let table = LogicalDrive::get_drives(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "interface_addresses" => {
            let table = InterfaceAddress::get_interfaces(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "interface_details" => {
            let table = InterfaceDetails::get_interface_details(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "uptime" => {
            let uptime = Uptime::get_uptime().unwrap();

            let mut table: Vec<Uptime> = Vec::new();
            table.push(uptime);

            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "products" => {
            let table = Products::get_products_info();
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_computer_info" => {
            let wmi_computer_info = WmiComputerInfo::get_system_info(system_reader.borrow());

            let mut table: Vec<WmiComputerInfo> = Vec::new();
            table.push(wmi_computer_info);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_os_version" => {
            let wmi_os_version = WmiOsVersion::new(system_reader.borrow());

            let mut table: Vec<WmiOsVersion> = Vec::new();
            table.push(wmi_os_version);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_printers" => {
            let table = WmiPrinters::get_printers_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_services" => {
            let table = WmiServices::get_services_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_hotfixes_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_shares" => {
            let table = WmiShares::get_shares_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_netwok_adapters_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_local_accounts_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_bios" => {
            let wmi_bios = WmiBios::get_bios_info(system_reader.borrow());

            let mut table: Vec<WmiBios> = Vec::new();
            table.push(wmi_bios);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_motherboard" => {
            let wmi_motherboard = WmiMotherboard::get_motherboard_info(system_reader.borrow());

            let mut table: Vec<WmiMotherboard> = Vec::new();
            table.push(wmi_motherboard);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_processor" => {
            let wmi_processor = WmiProcessor::get_processor_info(system_reader.borrow());

            let mut table: Vec<WmiProcessor> = Vec::new();
            table.push(wmi_processor);
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_physical_memory" => {
            let table = WmiMemory::get_physical_memory_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_sound" => {
            let table = WmiSound::get_sound_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_video" => {
            let table = WmiVideo::get_video_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_monitors" => {
            let table = WmiMonitors::get_monitors_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_keyboard_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_pointing_device_info(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::gen_process_open_sockets_table();
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "processes" => {
            let table = ProcessesRow::gen_processes_table(system_reader.borrow());
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::gen_memory_map_table();
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            if columns.len() > 0 {
                select(&table, columns)
            } else {
                select_all(&table)
            }
        }
    };
    res
}
