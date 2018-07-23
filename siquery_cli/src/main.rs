#[macro_use]
extern crate clap;
extern crate siquery;

#[allow(unused_imports)]    // TODO column names with macros
#[macro_use]
extern crate prettytable;
extern crate rusqlite;
use rusqlite::{Result};

use prettytable::Table;

use siquery::query::{query_table, init_db, register_tables,
                     get_table_list, get_form_query, register_first};

use clap::App;

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
    let result = query_table(name, columns);
    print_table_pretty(result);
}

fn siquery_select(siquery: &str) {
    let first_table = get_form_query(&siquery);

    let db = init_db();

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

                println!("{:?} ", value.unwrap());
            }
        }
        Some(false) => println!("Table {} does not exit ", first_table),
        None => println!("Table {} does not exist ", first_table),
    }

    register_tables(&db, get_table_list(), first_table);
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
