use tables::*;
use vtab::*;
use rusqlite::{version_number, Connection, Error};
use rusqlite::types::Value;
use printer::*;

fn select_all<T>(table: &Vec<T>) -> Vec<Vec<Value>> where T:Table+Sized {
    let mut res: Vec<Vec<Value>> = Vec::new();

    let cols = table.column_names();

    let mut id: u64 = 1;

    let mut columns_id: Vec<u64> = Vec::new();

    for _col in cols.iter() {
        columns_id.push(id);
        id = id << 1;
    }

    for tab in table.iter() {
        let mut row: Vec<Value> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

fn select<T>(table: &Vec<T>, columns: Vec<String>) -> Vec<Vec<Value>> where T:Table+Sized {

    if columns.len() < 1 {
        return select_all(table);
    }

    let mut res: Vec<Vec<Value>> = Vec::new();
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
        let mut row: Vec<Value> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

pub fn query_table(name: &str, columns: Vec<String>) -> Vec<Vec<Value>> {
    let res = match name {
        #[cfg(feature = "etc_hosts")]
        "etc_hosts" => {
            let table = EtcHosts::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "etc_protocols")]
        "etc_protocols" => {
            let table = EtcProtocols::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "etc_services")]
        "etc_services" => {
            let table = EtcServices::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "system_info")]
        "system_info" => {
            let table = SystemInfoData::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "os_version")]
        "os_version" => {
            let table = OsVersion::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "logical_drives")]
        "logical_drives" => {
            let table = LogicalDrive::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "interface_address")]
        "interface_address" => {
            let table = InterfaceAddress::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "interface_details")]
        "interface_details" => {
            let table = InterfaceDetails::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "uptime")]
        "uptime" => {
            let table = Uptime::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "products")]
        "products" => {
            let table = Products::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_computer_info")]
        "wmi_computer_info" => {
            let table = WmiComputerInfo::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_os_version")]
        "wmi_os_version" => {
            let table = WmiOsVersion::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_printers")]
        "wmi_printers" => {
            let table = WmiPrinters::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_services")]
        "wmi_services" => {
            let table = WmiServices::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_hotfixes")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_shares")]
        "wmi_shares" => {
            let table = WmiShares::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_network_adapters")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_local_accounts")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_bios")]
        "wmi_bios" => {
            let table = WmiBios::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_motherboard")]
        "wmi_motherboard" => {
            let table = WmiMotherboard::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_processor")]
        "wmi_processor" => {
            let table = WmiProcessor::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_physical_memory")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_sound")]
        "wmi_sound" => {
            let table = WmiSound::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_video")]
        "wmi_video" => {
            let table = WmiVideo::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_monitors")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_keyboard")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "wmi_pointing_device")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "process_open_sockets")]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "processes")]
        "processes" => {
            let table = ProcessesRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "process_memory_map")]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "process_envs")]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "mounts")]
        "mounts" => {
            let table = MountsRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "users")]
        "users" => {
            let table = Users::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "logged_in_users")]
        "logged_in_users" => {
            let table = LoggedInUsers::get_specific();
            select(&table, columns)
        }
        #[cfg(feature = "logon_sessions")]
        "logon_sessions" => {
            let table = LogonSessions::get_specific();
            select(&table, columns)
        }
        #[cfg(feature = "groups")]
        "groups" => {
            let table = GroupsRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "proxies")]
        "proxies" => {
            let table = ProxiesRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "launchd")]
        "launchd" => {
            let table = LaunchdRow::get_specific();
            select(&table, columns)
        },
        #[cfg(feature = "launchd_overrides")]
        "launchd_overrides" => {
            let table = LaunchdOverridesRow::get_specific();
            select(&table, columns)
        },
        _ => { // for tests only
            let table: Vec<Dummy> = vec![
                Dummy{a:25, b:25},
                Dummy{a:25, b:25}];
            select(&table, columns)
        }
    };
    res
}

pub fn init_db()-> Connection {
    let db = Connection::open_in_memory().unwrap();
    load_module(&db).unwrap();
    register_tables(&db, get_table_list());
    db
}

fn register_tables(db: &Connection, tables: Vec<String>) {
    let version = version_number();
    if version < 3008012 {
        println!("version: '{}' is not supported", version);
        return
    }
    for tab in tables.iter() {
        let mut sql = String::from("CREATE VIRTUAL TABLE ");
        sql.push_str(tab);
        sql.push_str(" USING siquery(table_name=");
        sql.push_str(tab);
        sql.push(')');
        &db.execute_batch(&sql).unwrap();
    }
}

fn create_schema(column_name: &Vec<&'static str>, column_types: &Vec<&'static str>) -> Option<String> {
    let mut schema = None;
    if schema.is_none() {
        let mut sql = String::from("CREATE TABLE x(");
        for (i, col) in column_name.iter().enumerate() {
            sql.push('"');
            sql.push_str(col);
            sql.push_str(column_types[i]);
            if i == column_name.len() - 1 {
                sql.push_str(");");
            } else {
                sql.push_str(", ");
            }
        }
        schema = Some(sql);
    }
    schema
}

pub fn get_schema(table_name: &str) -> Option<String> {
    let mut _schema = None;
    match table_name {
        #[cfg(feature = "etc_hosts")]
        "etc_hosts" => {
            let column_names = EtcHosts::get_columns_name();
            let column_types = EtcHosts::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "etc_protocols")]
        "etc_protocols" => {
            let column_names = EtcProtocols::get_columns_name();
            let column_types = EtcProtocols::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "etc_services")]
        "etc_services" => {
            let column_names = EtcServices::get_columns_name();
            let column_types = EtcServices::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "system_info")]
        "system_info" => {
            let column_names = SystemInfoData::get_columns_name();
            let column_types = SystemInfoData::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "os_version")]
        "os_version" => {
            let column_names = OsVersion::get_columns_name();
            let column_types = OsVersion::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "logical_drives")]
        "logical_drives" => {
            let column_names = LogicalDrive::get_columns_name();
            let column_types = LogicalDrive::get_columns_type();
            _schema = create_schema(& column_names, & column_types)
        },
        #[cfg(feature = "interface_address")]
        "interface_address" => {
            let column_names = InterfaceAddress::get_columns_name();
            let column_types = InterfaceAddress::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "interface_details")]
        "interface_details" => {
            let column_names = InterfaceDetails::get_columns_name();
            let column_types = InterfaceDetails::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "uptime")]
        "uptime" => {
            let column_names = Uptime::get_columns_name();
            let column_types = Uptime::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "products")]
        "products" => {
            let column_names = Products::get_columns_name();
            let column_types = Products::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_computer_info")]
        "wmi_computer_info" => {
            let column_names = WmiComputerInfo::get_columns_name();
            let column_types = WmiComputerInfo::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_os_version")]
        "wmi_os_version" => {
            let column_names = WmiOsVersion::get_columns_name();
            let column_types = WmiOsVersion::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_printers")]
        "wmi_printers" => {
            let column_names = WmiPrinters::get_columns_name();
            let column_types = WmiPrinters::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_services")]
        "wmi_services" => {
            let column_names = WmiServices::get_columns_name();
            let column_types = WmiServices::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_hotfixes")]
        "wmi_hotfixes" => {
            let column_names = WmiHotfixes::get_columns_name();
            let column_types = WmiHotfixes::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_shares")]
        "wmi_shares" => {
            let column_names = WmiShares::get_columns_name();
            let column_types = WmiShares::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_network_adapters")]
        "wmi_network_adapters" => {
            let column_names = WmiNetworkAdapters::get_columns_name();
            let column_types = WmiNetworkAdapters::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_local_accounts")]
        "wmi_local_accounts" => {
            let column_names = WmiLocalAccounts::get_columns_name();
            let column_types = WmiLocalAccounts::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_bios")]
        "wmi_bios" => {
            let column_names = WmiBios::get_columns_name();
            let column_types = WmiBios::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_motherboard")]
        "wmi_motherboard" => {
            let column_names = WmiMotherboard::get_columns_name();
            let column_types = WmiMotherboard::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_processor")]
        "wmi_processor" => {
            let column_names = WmiProcessor::get_columns_name();
            let column_types = WmiProcessor::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_physical_memory")]
        "wmi_physical_memory" => {
            let column_names = WmiMemory::get_columns_name();
            let column_types = WmiMemory::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_sound")]
        "wmi_sound" => {
            let column_names = WmiSound::get_columns_name();
            let column_types = WmiSound::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_video")]
        "wmi_video" => {
            let column_names = WmiVideo::get_columns_name();
            let column_types = WmiVideo::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_monitors")]
        "wmi_monitors" => {
            let column_names = WmiMonitors::get_columns_name();
            let column_types = WmiMonitors::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_keyboard")]
        "wmi_keyboard" => {
            let column_names = WmiKeyboard::get_columns_name();
            let column_types = WmiKeyboard::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "wmi_pointing_device")]
        "wmi_pointing_device" => {
            let column_names = WmiPointingDevice::get_columns_name();
            let column_types = WmiPointingDevice::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "process_open_sockets")]
        "process_open_sockets" => {
            let column_names = ProcessOpenSocketsRow::get_columns_name();
            let column_types = ProcessOpenSocketsRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "processes")]
        "processes" => {
            let column_names = ProcessesRow::get_columns_name();
            let column_types = ProcessesRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "process_memory_map")]
        "process_memory_map" => {
            let column_names = ProcessMemoryMapRow::get_columns_name();
            let column_types = ProcessMemoryMapRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "process_envs")]
        "process_envs" => {
            let column_names = ProcessEnvsRow::get_columns_name();
            let column_types = ProcessEnvsRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "mounts")]
        "mounts" => {
            let column_names = MountsRow::get_columns_name();
            let column_types = MountsRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "users")]
        "users" => {
            let column_names = Users::get_columns_name();
            let column_types = Users::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "logged_in_users")]
        "logged_in_users" => {
            let column_names = LoggedInUsers::get_columns_name();
            let column_types = LoggedInUsers::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "logon_sessions")]
        "logon_sessions" => {
            let column_names = LogonSessions::get_columns_name();
            let column_types = LogonSessions::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        }
        #[cfg(feature = "groups")]
        "groups" => {
            let column_names = GroupsRow::get_columns_name();
            let column_types = GroupsRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "proxies")]
        "proxies" => {
            let column_names = ProxiesRow::get_columns_name();
            let column_types = ProxiesRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "launchd")]
        "launchd" => {
            let column_names = LaunchdRow::get_columns_name();
            let column_types = LaunchdRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        #[cfg(feature = "launchd_overrides")]
        "launchd_overrides" => {
            let column_names = LaunchdOverridesRow::get_columns_name();
            let column_types = LaunchdOverridesRow::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        },
        _ => {
            let column_names = Dummy::get_columns_name();
            let column_types = Dummy::get_columns_type();
            _schema = create_schema(&column_names, &column_types)
        }
    };
    _schema
}

pub fn execute_query(db: &Connection, query: &str, flag: u8) {
    let mut table_result: Vec<Vec<Value>> = Vec::new();
    let mut row: Vec<Value> = Vec::new();
    let stmt = db.prepare(&query);

    match stmt {
        Ok(mut statement_res) => {
            let mut col_name_internal = Vec::new();
            for col_name in statement_res.column_names().iter() {
                col_name_internal.push(col_name.to_string());

                let v: Value = Value::Text(col_name.to_string());
                row.push(v);
            }
            table_result.push(row);

            let mut response = statement_res.query(&[]).unwrap();
            if flag == 2 {
                print_csv(col_name_internal, &mut response);
            } else if flag == 1 {
                print_json(&col_name_internal, &mut response);
            } else {
                print_pretty(col_name_internal, &mut response);
            }
        },
        Err(e) =>
            match e {
                Error::SqliteFailure(_r, m) => {
                    if let Some(msg) = m { println!("{}", msg) };
                },
                _ => println!("{:?}", Error::ModuleError(format!("{}", e)))
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation(){
        let schema =  create_schema(&vec!["name", "id"], &vec!["\" TEXT", "\" INTEGER"] );
        assert_eq!(schema.unwrap(), "CREATE TABLE x(\"name\" TEXT, \"id\" INTEGER);");
    }
}
