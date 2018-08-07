#[macro_use]
extern crate clap;
extern crate siquery;
// TODO column names with macros\
extern crate prettytable;
extern crate serde;
extern crate serde_json;
extern crate rusqlite;

use prettytable::Table;
use siquery::query::{query_table, init_db, execute_query};
use clap::App;
use rusqlite::Connection;

fn print_table_pretty(result: Vec<Vec<String>>) {
    let table = Table::from(result);
    table.printstd();
}

fn query_select(name: &str, select: &str) {
    let mut columns: Vec<String> = vec![];
    if select != "*" {
        let cols: Vec<_> = select.split(',').collect();
        for col in cols {
            let column = col.trim();
            if column.len() < 1 {
                continue;
            }
            columns.push(col.to_string());
        }
    }
    let _result = query_table(name, columns.clone());
}

fn siquery_select(db: &Connection, siquery: &str, flag: u8){
    let begin = std::time::SystemTime::now();
    execute_query(&db, siquery, flag);
    println!("{:?}", begin.elapsed());
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let select = matches.value_of("select").unwrap_or("").to_string();
    let siquery = matches.value_of("siquery").unwrap_or("").to_string();
    let db = init_db();
    if matches.is_present("json_mode") {
        if siquery.len() > 0 {
            siquery_select(&db, &siquery, 1);
        }
    } else if matches.is_present("csv_mode") {
        if siquery.len() > 0 {
            siquery_select(&db, &siquery, 2);
        }
    } else {
        if siquery.len() > 0 {
            siquery_select(&db, &siquery, 0);
        }
    }
}
