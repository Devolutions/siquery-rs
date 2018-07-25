#[macro_use]
extern crate clap;
extern crate siquery;
#[allow(unused_imports)]    // TODO column names with macros
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rusqlite;

use rusqlite::{Result, Error, Statement, Connection};

use prettytable::Table;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use siquery::query::{query_table, init_db, register_table,
                     get_table_list, find_table,   query_header};

use clap::App;
use csv::{Writer, WriterBuilder, Terminator};
use std::time::{Duration, SystemTime};

use std::io;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn print_table_json(mut result: Vec<Vec<String>>, header: Vec<String>){
    for i in 0..result.len() {
        for j in 0..header.len(){
            result[i][j] = header[j].clone() + ": " + &result[i][j];
        }
    }
    let serialized = serde_json::to_string_pretty(&result).unwrap();
    println!("  {}", serialized);
}
fn print_table_csv(mut result: Vec<Vec<String>>, header: Vec<String>) {
    let mut wtr = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .double_quote(true)
        .terminator(Terminator::CRLF)
        .from_writer(vec![]);

    // insert the header to the result
    wtr.write_record(header);
    for res in result.iter(){
        wtr.write_record(res);
    }

    println!("{:?}", String::from_utf8(wtr.into_inner().unwrap()).unwrap());
}
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
    let mut result = query_table(name, columns.clone());

    //print_table_pretty(result.clone());
    //print_table_json(result, query_header(name, columns));
    //print_table_csv(result.clone(), query_header(name, columns).clone());
}
fn siquery_select(siquery: &str) {
    let db = init_db();


    let mut response = init_tables(&db, siquery);
    match response {
        Ok(res) => execute_query(&db, siquery),
        Err(e) => println!("{}", e),
    }
}
fn get_from_query_failure(msg: &str) -> Result<(&str)> {
    let v: Vec<&str> = msg.split("no such table: ").collect();
    if v.len() > 0 {
        if find_table(v[1]) {
            return Ok(v[1])
        }
        Err(Error::ModuleError(format!("{}", msg)))
    } else {
        Err(Error::ModuleError(format!("{}", msg)))
    }
}
fn init_tables(db: &Connection, query: &str) -> Result<(&'static str, &'static str)> {
    let mut is_ok = false;
    let mut s = db.prepare(&query);

    let sys_time = SystemTime::now();

    match s {
        Ok(v) => return Ok(("all tables from query are registred in memory", "ok")) ,
        Err(e) => {
            match e {
                rusqlite::Error::SqliteFailure(r, m) => {
                    if let Some(msg) = m {
                        match get_from_query_failure(&msg) {
                            Ok(table) => register_table(&db, table.to_string()),
                            Err(error) => return Err(Error::ModuleError(format!("{}'", error))),
                        };
                        let difference = SystemTime::now().duration_since(sys_time)
                            .expect("SystemTime::duration_since failed");
                        println!("init table duration : {:?}",  difference);
                        init_tables(db, query)
                    } else {
                        return Err(Error::ModuleError(format!("{:?}", m)));
                    }
                }
                _ => return Err(Error::ModuleError(format!("{}", e)))
            }
        }

    }


}
fn execute_query(db: &Connection, query: &str) {
    let mut s = db.prepare(&query).unwrap();
    // bad type error if querying a counter
    let sys_time = SystemTime::now();
    for i in 0..s.column_names().len() {
        //print!("{} ", s.column_names()[i]);

        let value: Result<Vec<String>> = s
            .query_map(&[], |row| row.get::<_, String>(i))
            .unwrap()
            .collect();

        //println!("{:?} ", value.unwrap());
    }
    let difference = SystemTime::now().duration_since(sys_time)
        .expect("SystemTime::duration_since failed");
    println!("query duration : {:?}",  difference);
}

fn main() {
    let sys_time = SystemTime::now();

    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let select = matches.value_of("select").unwrap_or("").to_string();
    let siquery = matches.value_of("siquery").unwrap_or("").to_string();

    if table.len() > 0 {
        query_select(table.as_str(), select.as_str());
    }
    if siquery.len() > 0 {
        siquery_select(&siquery);
    }

    let difference = SystemTime::now().duration_since(sys_time)
        .expect("SystemTime::duration_since failed");
    println!("All duration: {:?}",  difference);
}
