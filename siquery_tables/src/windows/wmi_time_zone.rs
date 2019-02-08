use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiTimeZone, WmiTimeZoneIface};
use utils;

pub struct Reader {}
impl WmiTimeZoneIface for Reader {
    fn get_wmi_time_zone_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["TIMEZONE",
                "get",
                "Description",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiTimeZone {
    pub(crate) fn new() -> WmiTimeZone {
        WmiTimeZone {
            description: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiTimeZoneIface) -> Vec<WmiTimeZone> {

        let mut time_zones: Vec<WmiTimeZone> = Vec::new();

        if let Some(time_zone_info) = reader.get_wmi_time_zone_info() {
            let mut time_zone = WmiTimeZone::new();
            let lines = time_zone_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if time_zone.description != "" {
                        time_zones.push(time_zone);
                    }
                    time_zone = WmiTimeZone::new();
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
                    "Description" => {
                        time_zone.description = v;
                    },
                    _ => ()
                }
            }
        }

        time_zones
    }

    pub(crate) fn get_specific() -> Vec<WmiTimeZone> {
        let reader: Box<WmiTimeZoneIface> = Box::new(Reader{});
        let out = WmiTimeZone::get_specific_ex(reader.borrow());
        out
    }
}

// todo : test table