#[macro_use]
extern crate clap;
extern crate siquery;

use siquery::query::{init_db, execute_query};
use siquery::tables::get_table_list;
use siquery::printer::print_schema;
use clap::App;

#[cfg(target_os = "windows")]
use siquery::inventory::execute_inventory_query;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let siquery = matches.value_of("siquery").unwrap_or("").to_string();
    let schema = matches.value_of("schema").unwrap_or("").to_string();
    let db = init_db();

    if matches.is_present("list_all") {
        for table in get_table_list().iter() {
            println!("{}", table);
        }
    } else if matches.is_present("rdm_inventory") {
        #[cfg(target_os = "windows")]
        execute_inventory_query();
    } else if matches.is_present("schema") {
        print_schema(schema);
    } else if matches.is_present("json_mode") {
        if table.len() > 0 {
            let v: Vec<_> = table.split(',').collect();
            for t in get_table_list().iter() {
                if let Some(table_name) = v.iter().find(|&&x| x == *t) {
                    let query = format!("select * from {}", table_name);
                    println!("table : {}", table_name);
                    execute_query(&db, &query, 1);
                }
            }

        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 1);
        }
    } else if matches.is_present("csv_mode") {
        if table.len() > 0 {
            let v: Vec<_> = table.split(',').collect();
            for t in get_table_list().iter() {
                if let Some(table_name) = v.iter().find(|&&x| x == *t) {
                    let query = format!("select * from {}", table_name);
                    println!("table : {}", table_name);
                    execute_query(&db, &query, 2);
                }
            }
        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 2);
        }
    } else {
        if table.len() > 0 {
            let v: Vec<_> = table.split(',').collect();
            for t in get_table_list().iter() {
                if let Some(table_name) = v.iter().find(|&&x| x == *t) {
                    let query = format!("select * from {}", table_name);
                    println!("table : {}", table_name);
                    execute_query(&db, &query, 0);
                }
            }
        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 0);
        }
    }
}
