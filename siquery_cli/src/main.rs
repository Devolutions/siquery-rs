#[macro_use]
extern crate clap;
extern crate siquery;

use siquery::query::{init_db, execute_query};
use siquery::tables::get_table_list;
use siquery::printer::print_schema;
use siquery::inventory::execute_inventory_query;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let siquery = matches.value_of("siquery").unwrap_or("").to_string();
    let schema  = matches.value_of("schema").unwrap_or("").to_string();
    let rdm_inventory = matches.value_of("rdmInventory").unwrap_or("").to_string();
    let db = init_db();

    if matches.is_present("list_all") {
        for table in get_table_list().iter() {
            println!("{}", table);
        }
    } else if matches.is_present("schema") {
        print_schema(schema);
    } else if matches.is_present("json_mode") {
        if table.len() > 0 {
            let query = format!("select * from {}", table);
            execute_query(&db, &query, 1);
        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 1);
        }
    } else if matches.is_present("csv_mode") {
        if table.len() > 0 {
            let query = format!("select * from {}", table);
            execute_query(&db, &query, 2);
        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 2);
        }
    } else {
        if table.len() > 0 {
            let query = format!("select * from {}", table);
            execute_query(&db, &query, 0);
        } else if siquery.len() > 0 {
            execute_query(&db, &siquery, 0);
        }else if rdm_inventory.len() > 0 {
            println!("{:?}", rdm_inventory);
            execute_inventory_query(&rdm_inventory);
        }
    }
}
