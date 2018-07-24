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

use rusqlite::{Result, Statement};

use prettytable::Table;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use siquery::query::{query_table, init_db, register_tables,
                     get_table_list, get_form_query, register_first, query_header};

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
    print_table_csv(result.clone(), query_header(name, columns).clone());
}

fn siquery_select(siquery: &str) {
    let first_table = get_form_query(&siquery);

    let db = init_db();
    let sys_time = SystemTime::now();
    match register_first(&db, first_table.clone()) {
        Some(true) => {
            let mut s = db.prepare(&siquery).unwrap();
            // bad type error if querying a counter
            for i in 0..s.column_names().len() {
                print!("{} ", s.column_names()[i]);

               let value: Result<Vec<String>> = s
                    .query_map(&[], |row| row.get::<_, String>(i))
                    .unwrap()
                    .collect();

                //println!("{:?} ", value.unwrap());
            }
        }
        Some(false) => println!("Table {} does not exit ", first_table),
        None => println!("Table {} does not exist ", first_table),
    }

    let difference = SystemTime::now().duration_since(sys_time)
        .expect("SystemTime::duration_since failed");
    println!("query duration : {:?}", difference);

    /*let sys_time = SystemTime::now();
    register_tables(&db, get_table_list(), first_table);
    let difference = SystemTime::now().duration_since(sys_time)
        .expect("SystemTime::duration_since failed");
    println!("registering tables duration : {:?}", difference);*/
}

fn main() {
    let sys_time = SystemTime::now();

    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let select = matches.value_of("select").unwrap_or("").to_string();
    let siquery = matches.value_of("siquery").unwrap_or("").to_string();
    //let mode = matches.value_of("csv_mode").unwrap_or("").to_string();
    if table.len() > 0 {
        query_select(table.as_str(), select.as_str());

        /*println!("mode ? {}", mode);
        if mode.len() > 0 {

        }*/
    }
    if siquery.len() > 0 {
        siquery_select(&siquery);
    }

    let difference = SystemTime::now().duration_since(sys_time)
        .expect("SystemTime::duration_since failed");
    println!("All duration : {:?}", difference);
}
