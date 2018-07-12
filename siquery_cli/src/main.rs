#[macro_use]
extern crate clap;
extern crate siquery;
use siquery::query::query_table;

use clap::App;

fn query_select_all(name: &str) {
    let columns: Vec<String> = vec![];
    let res = query_table(name, columns);

    println!("{:?}", name);
    for row in res.iter() {
        println!("{:?}", row);
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();
    let table = matches.value_of("table").unwrap_or("").to_string();
    query_select_all(table.as_str());
}