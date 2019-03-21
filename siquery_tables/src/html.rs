#![allow(unused_imports)]

use rusqlite::{
    Rows,
    Statement,
    types::{
        Value,
        Type
    },
};
use horrorshow::{
    helper::doctype,
    prelude::*
};
use chrono::offset::Local;
use std::fs::File;
use std::io::prelude::*;

use tables::{
    WmiOsVersion,
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
    let actual = format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title : "title";
                    style(type="text/css") : r#"
                        body {
                            background: white;
                            padding: 0;
                            margin: 0;
                        }

                        .value {
                            vertical-align: center;
                            max-width:50%;
                        }

                        .HeaderHolder {
                            padding: 9px;
                        }

                        .HeaderTitle {
                            color: #454545;
                            padding: 10px 10px;
                            text-align: center;
                        }

                        .Container {
                            background-color: #F5F5F5;
                            border: 1px solid silver;
                            padding: 5px 10px;
                            width: 97%;

                            border-radius: 0 3px 3px 0;
                            -moz-border-radius: 0 3px 3px 0;
                            -webkit-border-radius: 0 3px 3px 0;

                            /* min-height ie fix */
                            height: auto !important;
                            height: 26px;
                            min-height: 26px;
                            overflow: hidden;

                            display: -webkit-box;
                            display: -webkit-flex;
                            display: -ms-flexbox;
                            display: flex;
                            -webkit-flex-wrap: wrap;
                            -ms-flex-wrap: wrap;
                            flex-wrap: wrap;
                        }


                        .HeaderHolder .Drive table, .HeaderHolder .LocalAccount table, .HeaderHolder .NetworkAdapter table {
                            float: left;
                            margin: 0 6px 6px 0;
                            padding: 3px 6px;
                            width: 48%;
                            border: 1px solid transparent;
                        }

                        .Box {
                            margin-left: auto;
                            margin-right: auto;
                            margin-top: 2px;
                            margin-bottom: 2px;
                            border: 1px solid lightgray;
                            max-width: 50%;
                        }

                        .title2 {
                            color: #808080;
                            font-family: segoe ui, arial;
                            font-size: 14px;
                            font-weight: bold;
                            line-height: 24px;
                            width: 100%;
                            align-text: center;
                        }

                        th {
                            font-family: segoe ui, arial;
                            font-size: 11px;
                            font-weight: normal;
                            line-height: 16px;
                            padding: 0 5px 0 0;
                            white-space: nowrap;
                            width: 50%;
                            text-align: center;
                        }

                        td {
                            vertical-align: top;
                            empty-cells: show;
                            font-family: segoe ui, arial;
                            font-size: 11px;
                            line-height: 16px;
                            padding: 0 2px;
                            width: 50%;
                        }

                        .title {
                            color: #5E5E5E;
                            font-family: segoe ui, arial;
                            font-size: 16px;
                            font-weight: bold;
                            line-height: 24px;
                            width: 100%;
                            text-align: center;
                            padding-bottom: 5px;
                        }

                        td.label {
                            color: #808080;
                        }"#
                }
                body {
                    div(class="HeaderHolder") {
                        div(class="HeaderTitle") {
                            table {
                                tr {
                                    td(colspan="2", class="title2") {
                                        : format!(
                                        "Inventory Report of {} - {}",
                                        SystemInfoData::get_specific()
                                        .get(0)
                                        .unwrap_or(&SystemInfoData::new()).computer_name,
                                        Local::now()
                                        );  //fixme
                                    }
                                }
                                tr {
                                    td(colspan="2", class="header")
                                }
                            }
                        }
                        div (class="Container") {
                            div (class="title2") {
                                : Raw(table_name.get(3).unwrap_or(&""));
                            }
                            @ for j in 0..map.len() {
                                table(class="Box") {
                                    tbody{
                                        @ for i in 0..columns.len() {
                                            tr {
                                                td(class="label") {
                                                    : columns[i].clone();
                                                }
                                                td(class="value") {
                                                    : map[j][i].clone();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    let mut file = File::create(
        "C:\\Users\\admin\\CLionProjects\\mounted\\siquery-rs\\siquery_cli\\src\\inventory.html"
    ).unwrap();    //fixme
    file.write_all(actual.as_bytes()).unwrap();    //fixme
    //println!("{}",actual);
}