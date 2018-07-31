#[macro_use]
extern crate clap;
extern crate siquery;
// TODO column names with macros\
extern crate prettytable;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rusqlite;

use prettytable::Table;
use siquery::query::{query_table, init_db, execute_query};

use clap::App;
use csv::{WriterBuilder, Terminator};
use std::time::{SystemTime};

fn print_table_json(mut result: Vec<Vec<String>>, header: Vec<String>){
    for i in 0..result.len() {
        for j in 0..header.len(){
            result[i][j] = header[j].clone() + ": " + &result[i][j];
        }
    }
    let serialized = serde_json::to_string_pretty(&result).unwrap();
    println!("  {}", serialized);
}
fn print_table_csv(result: Vec<Vec<String>>, header: Vec<String>) {
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
    let _result = query_table(name, columns.clone());

    //print_table_pretty(result.clone());
    //print_table_json(result, query_header(name, columns));
    //print_table_csv(result.clone(), query_header(name, columns).clone());
}

fn siquery_select(siquery: &str)-> Vec<Vec<String>> {
    let db = init_db();
    execute_query(&db, siquery)
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
        print_table_pretty(siquery_select(&siquery));
    }
}
