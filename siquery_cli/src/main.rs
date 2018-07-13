#[macro_use]
extern crate clap;
extern crate siquery;

#[macro_use]
extern crate prettytable;

use prettytable::Table;

use siquery::query::query_table;

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

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    let select = matches.value_of("select").unwrap_or("").to_string();
    query_select(table.as_str(), select.as_str());
}
