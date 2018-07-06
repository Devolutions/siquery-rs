
pub use sys::{SystemInfo, SystemReader};
use sys::{SystemReaderInterface};

use std::borrow::Borrow;

use tables::{
    Table,
    Dummy,
    EtcHosts,
    EtcProtocols,
    EtcServices,
};

fn query_generate<T>(table: &Vec<T>) -> Vec<Vec<String>> where T:Table+Sized {
    let mut res: Vec<Vec<String>> = Vec::new();

    let cols = table.column_names();
    let mut hdr: Vec<String> = Vec::new();
    for col in cols.iter() {
        hdr.push(col.to_string());
    }

    res.push(hdr); // FIXME: store header elsewhere

    for tab in table.iter() {
        let mut row: Vec<String> = Vec::new();
        for col in cols.iter() {
            row.push(tab.get(col));
        }
        res.push(row);
    }
    res
}

pub fn query_table(name: &str) -> Vec<Vec<String>> {
    let system_reader: Box<SystemReaderInterface> = Box::new(SystemReader::new());
    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_hosts(system_reader.borrow());
            query_generate(&table)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_protocols(system_reader.borrow());
            query_generate(&table)
        },
        "etc_services" => {
            let table = EtcServices::get_services(system_reader.borrow());
            query_generate(&table)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            query_generate(&table)
        }
    };
    res
}
