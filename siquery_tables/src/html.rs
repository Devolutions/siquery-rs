use rusqlite::{
    Rows,
    types::{
        Value,
        Type
    },
};
use horrorshow::{
    helper::doctype,
};
use chrono::offset::Local;
use std::io::prelude::*;
use std::fs::OpenOptions;

use tables::{
    SystemInfoData
};

pub fn map(values: &mut Rows) -> Vec<Vec<String>> {
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    loop {
        if let Some(v) = values.next() {
            if let Some(res) = v.ok() {
                for i in 0..res.column_count() {
                    let val = Value::data_type(&res.get(i));
                    match val {
                        Type::Real | Type::Integer => {
                            row.push(res.get::<usize, i64>(i).to_string());
                        },
                        Type::Text => {
                            row.push(res.get::<usize, String>(i))
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                table.push(row);
                row = Vec::new();
            }
        } else {
            break
        }
    }
    table
}

pub fn print_html(columns: Vec<String>, values: &mut Rows, query: &str) {
    let map = map(values);
    let table_name = query.split(' ').collect::<Vec<&str>>();
    let hostname = format!(
        "{}",
        SystemInfoData::get_specific()
        .get(0)
        .unwrap_or(&SystemInfoData::new()).computer_name
    );
    let html_data = format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title : hostname.clone();
                }
                body {
                    TABLE(frame="hsides", rules="groups", cellpadding="1") {
                        CAPTION {
                            : format!(
                            "Inventory Report of {} - {}",
                            hostname,
                            Local::now()
                            );
                        }
                        COLGROUP(align="center");
                        COLGROUP(align="left");
                        THEAD(valign="top"){
                            TR{
                                TH(colspan="2"){
                                    : table_name.get(3);
                                }
                            }
                        }
                        THEAD(valign="top"){
                            TR {
                                TH {
                                    :"labels";
                                }
                                TH {
                                    :"values";
                                }
                            }
                        }
                        @ for j in 0..map.len() {
                            TBODY {
                                @ for i in 0..columns.len() {
                                    TR {
                                        TD {
                                            : columns[i].clone();
                                        }
                                        TD {
                                            : map[j][i].clone();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    write_file(html_data).unwrap_or_else(|e| println!("html printer failed with: {}",e));
}

fn write_file (data: String) -> std::io::Result<()> {
    let mut file: std::fs::File = OpenOptions::new().append(true).open("inventory.html")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}