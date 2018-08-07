use csv::{WriterBuilder, Terminator};
use rusqlite::{version_number, Connection, Rows, Row as RusqliteRow};
use rusqlite::types::{Value, Type};
use prettytable::{Table};
use prettytable::row::Row;
use prettytable::cell::Cell;
use utils;

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
    wtr.write_record(columns);
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
                wtr.write_record(row);
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
    let mut result: Vec<Vec<String>> = Vec::new();
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
    let mut out = "[\n".to_owned();
    loop {
        if let Some(v) = values.next(){
            if let Some (res) = v.ok() {
                out.push_str(&format_to_json(&col_names, &res));
            }
        } else {
            break
        }
    }
    utils::trim_string(&mut out);
    out.push_str("\n]");
    println!("{}",out);
}

fn format_to_json (col_names: &Vec<String>, row_value : &RusqliteRow) -> String {
    let mut value_to_json = String::new();
    match Value::data_type(&row_value.get(0)) {
        Type::Real | Type::Integer => {
            value_to_json.push_str(
                &format!(
                    "{:?}:{:?}",
                    col_names[0],
                    row_value.get::<usize,i64>(0).to_string()
                )
            );
        },
        Type::Text => {
            value_to_json.push_str(
                &format!(
                    "{:?}:{:?}",
                    col_names[0],
                    row_value.get::<usize,String>(0)
                )
            );
        },
        _ => {
            // Do nothing.
        }
    }
    for i in 1..row_value.column_count() {
        let v: Value = row_value.get(i);
        // todo add condition for flag
        match Value::data_type(&v) {
            Type::Real | Type::Integer => {
                value_to_json.push_str(
                    &format!(
                        ",{:?}:{:?}",
                        col_names[i],
                        row_value.get::<usize,i64>(i).to_string()
                    )
                );
            },
            Type::Text => {
                value_to_json.push_str(
                    &format!(
                        ",{:?}:{:?}",
                        col_names[i],
                        row_value.get::<usize,String>(i)
                    )
                );
            },
            _ => {
                // Do nothing.
            }
        }
    }
    format!("  {{{}}},\n", value_to_json)
}