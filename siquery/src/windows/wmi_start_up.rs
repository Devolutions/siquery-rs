use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiStartUp, WmiStartUpIface};
use utils;

pub struct Reader {}
impl WmiStartUpIface for Reader {
    fn get_wmi_start_up_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["STARTUP",
                "get",
                "Command,Location,Name,User",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiStartUp {
    pub(crate) fn new() -> WmiStartUp {
        WmiStartUp {
            command: String::new(),
            location: String::new(),
            name: String::new(),
            user: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiStartUpIface) -> Vec<WmiStartUp> {

        let mut start_ups: Vec<WmiStartUp> = Vec::new();

        if let Some(start_up_info) = reader.get_wmi_start_up_info() {
            let mut start_up = WmiStartUp::new();
            let lines = start_up_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if start_up.name != "" {
                        start_ups.push(start_up);
                    }
                    start_up = WmiStartUp::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Command" => {
                        start_up.command = v;
                    },
                    "Location" => {
                        start_up.location = v;
                    },
                    "Name" => {
                        start_up.name = v;
                    },
                    "User" => {
                        start_up.user = v;
                    },
                    _ => ()
                }
            }
        }

        start_ups
    }

    pub(crate) fn get_specific() -> Vec<WmiStartUp> {
        let reader: Box<WmiStartUpIface> = Box::new(Reader{});
        let out = WmiStartUp::get_specific_ex(reader.borrow());
        out
    }
}

// todo : test table