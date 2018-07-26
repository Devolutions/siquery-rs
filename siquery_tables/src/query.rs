use tables::*;

use vtab::*;
use rusqlite::{version_number, Connection, Result, Error};

fn select_all<T>(table: &Vec<T>) -> Vec<Vec<String>> where T:Table+Sized {
    let mut res: Vec<Vec<String>> = Vec::new();

    let cols = table.column_names();

    let mut id: u64 = 1;

    let mut columns_id: Vec<u64> = Vec::new();

    for _col in cols.iter() {
        columns_id.push(id);
        id = id << 1;
    }

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
    let mut columns_id: Vec<u64> = Vec::new();
    for column in columns.iter() {
        // make sure the header exist in the table
        if table.len() > 0 {
            let id = table[0].get_id(column);
            if id != 0 {
                columns_id.push(id);
            }
        }
    }

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
    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_specific();
            select(&table, columns)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_specific();
            select(&table, columns)
        },
        "etc_services" => {
            let table = EtcServices::get_specific();
            select(&table, columns)
        },
        "system_info" => {
            let table = SystemInfoData::get_specific();
            select(&table, columns)
        },
        "os_version" => {
            let table = OsVersion::get_specific();
            select(&table, columns)
        },
        "logical_drives" => {
            let table = LogicalDrive::get_specific();
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_address" => {
            let table = InterfaceAddress::get_specific();
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_details" => {
            let table = InterfaceDetails::get_specific();
            select(&table, columns)
        },
        "uptime" => {
            let table = Uptime::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "products" => {
            let table = Products::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => {
            let table = WmiComputerInfo::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_os_version" => {
            let table = WmiOsVersion::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_printers" => {
            let table = WmiPrinters::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_services" => {
            let table = WmiServices::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_shares" => {
            let table = WmiShares::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_bios" => {
            let table = WmiBios::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => {
            let table = WmiMotherboard::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_processor" => {
            let table = WmiProcessor::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_sound" => {
            let table = WmiSound::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_video" => {
            let table = WmiVideo::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_specific();
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_specific();
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific();
            select(&table, columns)
        },
        "processes" => {
            let table = ProcessesRow::get_specific();
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific();
            select(&table, columns)
        },
        #[cfg(not(target_os = "windows"))]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific();
            select(&table, columns)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            select(&table, columns)
        }
    };
    res
}

fn select_header<T>(table: &Vec<T>, columns: Vec<String>) -> Vec<String> where T:Table+Sized {
    let mut hdr: Vec<String> = Vec::new();
    if columns.len() < 1 {
        for col in table.column_names().iter(){
            hdr.push(col.to_string());
        }
        return hdr;
    }

    for column in columns.iter() {
        // make sure the header exist in the table
        let id = table[0].get_id(column);
        if id != 0 {
            hdr.push(column.to_string());
        }
    }

    hdr
}

pub fn query_header(name: &str, columns: Vec<String>) -> Vec<String> {

    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_specific();
            select_header(&table, columns)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_specific();
            select_header(&table, columns)
        },
        "etc_services" => {
            let table = EtcServices::get_specific();
            select_header(&table, columns)
        },
        "system_info" => {
            let table = SystemInfoData::get_specific();
            select_header(&table, columns)
        },
        "os_version" => {
            let table = OsVersion::get_specific();
            select_header(&table, columns)
        },
        "logical_drives" => {
            let table = LogicalDrive::get_specific();
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_address" => {
            let table = InterfaceAddress::get_specific();
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_details" => {
            let table = InterfaceDetails::get_specific();
            select_header(&table, columns)
        },
        "uptime" => {
            let table = Uptime::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "products" => {
            let table = Products::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => {
            let table = WmiComputerInfo::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_os_version" => {
            let table = WmiOsVersion::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_printers" => {
            let table = WmiPrinters::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_services" => {
            let table = WmiServices::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_shares" => {
            let table = WmiShares::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_bios" => {
            let table = WmiBios::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => {
            let table = WmiMotherboard::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_processor" => {
            let table = WmiProcessor::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_sound" => {
            let table = WmiSound::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_video" => {
            let table = WmiVideo::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_specific();
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_specific();
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific();
            select_header(&table, columns)
        },
        "processes" => {
            let table = ProcessesRow::get_specific();
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific();
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "windows"))]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific();
            select_header(&table, columns)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            select_header(&table, columns)
        }
    };
    res
}

pub fn get_table_list() -> Vec<String> {
    vec![
        "etc_hosts".to_string(),
        "etc_protocols".to_string(),
        "etc_services".to_string(),
        "system_info".to_string(),
        "os_version".to_string(),
        "logical_drives".to_string(),
        "uptime".to_string(),
        "processes".to_string(),
        #[cfg(not(target_os = "macos"))]
        "interface_address".to_string(),
        #[cfg(not(target_os = "macos"))]
        "interface_details".to_string(),
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets".to_string(),
        #[cfg(not(target_os = "macos"))]
        "process_memory_map".to_string(),
        #[cfg(target_os = "windows")]
        "products".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_computer_info".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_os_version".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_printers".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_services".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_hotfixes".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_shares".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_network_adapters".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_local_accounts".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_bios".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_motherboard".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_processor".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_physical_memory".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_sound".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_video".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_monitors".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_keyboard".to_string(),
        #[cfg(target_os = "windows")]
        "wmi_pointing_device".to_string(),
        #[cfg(not(target_os = "windows"))]
        "process_envs".to_string(),
    ]
}

pub fn find_table(table: &str) -> bool {
    match table {
        "etc_hosts" => true,
        "etc_protocols" => true,
        "etc_services" => true,
        "system_info" => true,
        "os_version" => true,
        "logical_drives" => true,
        "uptime" => true,
        "processes" => true,
        #[cfg(not(target_os = "macos"))]
        "interface_address" => true,
        #[cfg(not(target_os = "macos"))]
        "interface_details" => true,
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => true,
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => true,
        #[cfg(target_os = "windows")]
        "products" => true,
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => true,
        #[cfg(target_os = "windows")]
        "wmi_os_version" => true,
        #[cfg(target_os = "windows")]
        "wmi_printers" => true,
        #[cfg(target_os = "windows")]
        "wmi_services" => true,
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => true,
        #[cfg(target_os = "windows")]
        "wmi_shares" => true,
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => true,
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => true,
        #[cfg(target_os = "windows")]
        "wmi_bios" => true,
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => true,
        #[cfg(target_os = "windows")]
        "wmi_processor" => true,
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => true,
        #[cfg(target_os = "windows")]
        "wmi_sound" => true,
        #[cfg(target_os = "windows")]
        "wmi_video" => true,
        #[cfg(target_os = "windows")]
        "wmi_monitors" => true,
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => true,
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => true,
        #[cfg(not(target_os = "windows"))]
        "process_envs" => true,
        _ => false,
    }
}

pub fn init_db()-> Connection {
    let db = Connection::open_in_memory().unwrap();
    load_module(&db).unwrap();
    db
}

pub fn register_table(db:  &Connection, table: String) -> bool {
    let version = version_number();

    if version < 3008012 {
        println!("version: '{}' is not supported", version);
        return false;
    }
    if find_table(&table) {
            let command = format!("{}{}{}{}{}",
                                  "CREATE VIRTUAL TABLE ",
                                  &table,
                                  " USING siquery(table_name=",
                                  &table, ")");
            &db.execute_batch(&command).unwrap();
            return true;
        }

    false
}

pub fn register_tables(db:  &Connection, tables: Vec<String>, first_table: String) {
    let version = version_number();

    if version < 3008012 {
        println!("version: '{}' is not supported", version);
        return
    }
    for table in tables.iter() {
        if *table != first_table {
            let command = format!("{}{}{}{}{}",
                                  "CREATE VIRTUAL TABLE ",
                                  table,
                                  " USING siquery(table_name=",
                                  table, ")");
            &db.execute_batch(&command).unwrap();
        }
    }
}

pub fn execute_query(db: &Connection, query: &str) {
    let mut s = db.prepare(&query).unwrap();
    // bad type error if querying a counter
    // todo get col by type
    for i in 0..s.column_names().len() {
        print!("{} ", s.column_names()[i]);
        let value: Result<Vec<String>> = s
            .query_map(&[], |row| row.get::<_, String>(i))
            .unwrap()
            .collect();
        println!("{:?} ", value.unwrap());
    }
}

pub fn get_from_query_failure(msg: &str) -> Result<(&str)> {
    let v: Vec<&str> = msg.split("no such table: ").collect();
    if v.len() > 1 && find_table(v[1]) {
        return Ok(v[1])
    } else {
        Err(Error::ModuleError(format!("{}", msg)))
    }
}

pub fn init_query_tables(db: &Connection, query: &str) -> Result<(&'static str, &'static str)> {
    let s = db.prepare(&query);
    match s {
        Ok(_v) => return Ok(("all tables from query are registred in memory", "ok")) ,
        Err(e) => {
            match e {
                Error::SqliteFailure(_r, m) => {
                    if let Some(msg) = m {
                        match get_from_query_failure(&msg) {
                            Ok(table) => {
                                register_table(&db, table.to_string());
                            },
                            Err(error) => return Err(Error::ModuleError(format!("{}'", error))),
                        };
                        init_query_tables(db, query)
                    } else {
                        return Err(Error::ModuleError(format!("{:?}", m)));
                    }
                }
                _ => return Err(Error::ModuleError(format!("{}", e)))
            }
        }
    }
}