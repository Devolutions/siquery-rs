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

fn print_table_json(mut result: Vec<Vec<String>>, header: Vec<String>){
    for i in 0..result.len() {
        for j in 0..header.len(){
            result[i][j] = header[j].clone() + ": " + &result[i][j];
        }
    }
    let serialized = serde_json::to_string_pretty(&result).unwrap();
    println!("  {}", serialized);
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
}

fn siquery_select(siquery: &str)/*-> Vec<Vec<Value>>*/ {
    let db = init_db();
    //println!("query result {:?}", exec_query(&db, siquery).unwrap());
    let begin = std::time::SystemTime::now();
    execute_query(&db, siquery);
    println!("{:?}", begin.elapsed());
}

fn main() {
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
}
