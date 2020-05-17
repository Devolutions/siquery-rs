use csv::{WriterBuilder, Terminator};
use rusqlite::{Rows, Connection, Row as RusqliteRow, types::{Value, Type}};
use prettytable::{Table, row::Row, cell::Cell};
use crate::tables::get_table_list;
use crate::query::{get_schema, execute_query};
use serde_json::{Value as serdValue, Map};

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
        if let Ok(Some(res)) = values.next(){
            for i in 0..res.column_count() {
                let val = Value::data_type(&res.get_unwrap(i));
                match val {
                    Type::Real | Type::Integer => {
                        row.push(res.get_unwrap::<usize,i64>(i).to_string());
                    },
                    Type::Text => {
                        row.push(res.get_unwrap::<usize,String>(i))
                    },
                    _ => {
                        // Do nothing.
                    }
                }
            }
            // write row values
            wtr.write_record(row).expect("could not write row");;
            row = Vec::new();
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
        if let Ok(Some(res)) = values.next(){
            for i in 0..res.column_count() {
                let val = Value::data_type(&res.get_unwrap(i));
                match val {
                    Type::Real | Type::Integer => {
                        row.add_cell(Cell::new(&res.get_unwrap::<usize,i64>(i).to_string()));
                    },
                    Type::Text => {
                        row.add_cell(Cell::new(&res.get_unwrap::<usize,String>(i)))
                    },
                    _ => {
                        // Do nothing.
                    }
                }
            }
            table.add_row(row);
            row = Row::empty();
        } else {
            break
        }
    }
    println!("{}", table);
}

pub fn print_json(table_name: String, col_names: &Vec<String>, values: &mut Rows) -> Vec<Map<String,serdValue>> {
    let mut writer: Vec<Map<String,serdValue>> = Vec::new();
    let mut _value: Map<String, serdValue> = Map::new();
    loop {
        if let Ok(Some(res)) = values.next() {
            _value = format_to_json(&col_names, &res);
            writer.push(_value);
        } else {
            break
        }
    }
    if table_name.is_empty() {
        println!("{}", serde_json::to_string_pretty(&writer).unwrap());
    }

    writer
}

fn format_to_json(col_names: &Vec<String>, row_value : &RusqliteRow) -> Map<String, serdValue> {
    let mut value_json: Map<String, serdValue> = Map::new();
    match Value::data_type(&row_value.get_unwrap(0)) {
        Type::Real | Type::Integer => {
            value_json.insert(col_names[0].clone(),json!(row_value.get_unwrap::<usize,i64>(0)));
        },
        Type::Text => {
            value_json.insert(col_names[0].clone(),json!(row_value.get_unwrap::<usize,String>(0)));
        },
        _ => {
            // Do nothing.
        }
    }
    for i in 1..row_value.column_count() {
        let v: Value = row_value.get_unwrap(i);
        match Value::data_type(&v) {
            Type::Real | Type::Integer => {
                value_json.insert(col_names[i].clone(),json!(row_value.get_unwrap::<usize,i64>(i)));
            },
            Type::Text => {
                value_json.insert(col_names[i].clone(),json!(row_value.get_unwrap::<usize,String>(i)));
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

pub fn print_table_by_name(db: Connection, table: String, mode: u8) {
    let v: Vec<_> = table.split(',').collect();
    let mut json_table: Map<String, serdValue> = Map::new();
    for t in get_table_list().iter() {
        if let Some(table_name) = v.iter().find(|&&x| x == *t) {
            let query = format!("select * from {}", table_name);
            let writer = execute_query(&db, &query, table_name.to_string() , mode);
            json_table.insert(table_name.to_string(), json!(writer));
        }
    }
    if mode != 3 {
        println!("{}", serde_json::to_string_pretty(&json_table).unwrap());
    }
}