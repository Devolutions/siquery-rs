extern crate serde;
extern crate serde_json;
use csv::{WriterBuilder, Terminator};
use rusqlite::{Rows, Row as RusqliteRow, types::{Value, Type}};
use prettytable::{Table, row::Row, cell::Cell};
use tables::get_table_list;
use query::get_schema;
use serde_json::{Value as serdValue, Map};
use treexml::{Element,Document};
use heck::CamelCase;

pub fn print_csv(columns: Vec<String>, values: &mut Rows) {
    let mut row: Vec<String> = Vec::new();
    //init writer
    let mut wtr = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .double_quote(true)
        .terminator(Terminator::CRLF)
        .from_writer(vec![]);
    //write header first
    wtr.write_record(columns).expect("could not write columns");
    loop {
        if let Some(v) = values.next(){
            if let Some (res) = v.ok() {
                for i in 0..res.column_count() {
                    let val = Value::data_type(&res.get(i));
                    match val {
                        Type::Real | Type::Integer => {
                            row.push(res.get::<usize,i64>(i).to_string());
                        },
                        Type::Text => {
                            row.push(res.get::<usize,String>(i))
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                // write row values
                wtr.write_record(row).expect("could not write row");;
                row = Vec::new();
            }
        } else {
            break
        }
    }
    println!("{}", String::from_utf8(wtr.into_inner().unwrap()).unwrap());
}

pub fn print_pretty(columns: Vec<String>, values: &mut Rows) {
    let mut row = Row::empty();
    let mut table: Table = Table::new();
    //write header first
    table.set_titles(columns.iter().collect());
    loop {
        if let Some(v) = values.next(){
            if let Some (res) = v.ok() {
                for i in 0..res.column_count() {
                    let val = Value::data_type(&res.get(i));
                    match val {
                        Type::Real | Type::Integer => {
                            row.add_cell(Cell::new(&res.get::<usize,i64>(i).to_string()));
                        },
                        Type::Text => {
                            row.add_cell(Cell::new(&res.get::<usize,String>(i)))
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                table.add_row(row);
                row = Row::empty();
            }
        } else {
            break
        }
    }
    println!("{}", table);
}

pub fn print_json (col_names: &Vec<String>, values: &mut Rows) {
    let mut writer = Vec::new();
    let mut _value:  Map<String, serdValue> = Map::new();
    loop {
        if let Some(v) = values.next() {
            if let Some(res) = v.ok() {
                _value = format_to_json(&col_names, &res);
                writer.push(_value);
            }
        } else {
            break
        }
    }
    let json = serde_json::to_string_pretty(&writer).unwrap();
    println!("{}", json);
}

pub fn print_xml (col_names: &Vec<String>, rows: &mut Rows) {
    let mut root = Element::new("InventorySystemInformation");

    //for _column in col_names.iter() {
        let mut local_account = Element::new("LocalAccounts");
        loop {
            if let Some(row) = rows.next() {
                if let Some(row_data) = row.ok() {
                    let mut remote_account = Element::new("RemoteAccount");
                    for i in 0..(row_data.column_count()-1) {
                        let mut col = Element::new(col_names.get(i).unwrap().to_camel_case());
                        match Value::data_type(&row_data.get_checked(i).unwrap()) {
                            Type::Text => {
                                col.text = Some(row_data.get_checked::<usize, String>(i).unwrap());
                                remote_account.children.push(col);
                            },
                            Type::Real | Type::Integer => {
                                col.text = Some(row_data.get_checked::<usize, i64>(i).unwrap().to_string());
                                remote_account.children.push(col);
                            },
                            _ => {
                                // Do nothing.
                            }
                        }
                    }
                    local_account.children.push(remote_account);
                }
            } else {
                break
            }
        }
        root.children.push(local_account);
    //}

    let doc = Document {
        root: Some(root),
        encoding: "utf-8".to_string(),
        ..Document::default()
    };

    println!("{}",doc.to_string());
}

//fn format_to_xml(col_names: &Vec<String>, row_value : &RusqliteRow) -> Map<String, serdValue> {}

fn format_to_json (col_names: &Vec<String>, row_value : &RusqliteRow) -> Map<String, serdValue> {
    let mut value_json: Map<String, serdValue> = Map::new();
    match Value::data_type(&row_value.get(0)) {
        Type::Real | Type::Integer => {
            value_json.insert(col_names[0].clone(),json!(row_value.get::<usize,i64>(0)));
        },
        Type::Text => {
            value_json.insert(col_names[0].clone(),json!(row_value.get::<usize,String>(0)));
        },
        _ => {
            // Do nothing.
        }
    }
    for i in 1..row_value.column_count() {
        let v: Value = row_value.get(i);
        match Value::data_type(&v) {
            Type::Real | Type::Integer => {
                value_json.insert(col_names[i].clone(),json!(row_value.get::<usize,i64>(i)));
            },
            Type::Text => {
                value_json.insert(col_names[i].clone(),json!(row_value.get::<usize,String>(i)));
            },
            _ => {
                // Do nothing.
            }
        }
    }
    value_json
}

pub fn print_schema(table: String) {
    if table.len() > 0 {
        if let Some(value) = get_table_list().iter().find(|x| *x.as_str() == *table) {
            let mut schema = get_schema(value.as_str()).unwrap();
            schema = schema.replace("x(", &format!("{}{}", value.as_str(), "("));
            println!("{}", schema);
        } else {
            println!("no such table {}", table);
        }
    } else {
        for table in get_table_list().iter() {
            let mut schema = get_schema(table.as_str()).unwrap();
            schema = schema.replace("x(", &format!("{}{}", table.as_str(), "("));
            println!("{}", schema);
        }
    }
}