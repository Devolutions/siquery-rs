use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiBios,WmiBiosIface};
use crate::utils;

pub struct Reader {}
impl WmiBiosIface for Reader {
    fn get_wmi_bios_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["bios", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiBios {
    pub(crate) fn get_specific_ex(reader: &WmiBiosIface) -> Vec<WmiBios> {
        let mut out = Vec::new();
        let mut bios = WmiBios {
            caption : String::new(),
            manufacturer: String::new(),
            release_date: String::new(),
            serial_number: String::new(),
            smbios_version: String::new(),
        };
        if let Some(bios_info) = reader.get_wmi_bios_info() {
            let lines = bios_info.split('\n');

            for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Caption" => {
                        bios.caption = v;
                    },
                    "Manufacturer" => {
                        bios.manufacturer = v;
                    },
                    "ReleaseDate" => {
                        v.truncate(8);
                        bios.release_date = v;
                    },
                    "SerialNumber" => {
                        bios.serial_number = v;
                    },
                    "SMBIOSBIOSVersion" => {
                        bios.smbios_version = v;
                    },
                    _ => ()
                }
            }
        }
        out.push(bios);
        out
    }

    pub(crate) fn get_specific() -> Vec<WmiBios> {
        let reader: Box<WmiBiosIface> = Box::new(Reader{});
        let out = WmiBios::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiBiosIface for Test {
        fn get_wmi_bios_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-bios.txt")))
        }
    }
    #[test]
    fn test_wmi_bios () {
        let reader: Box<WmiBiosIface> = Box::new(Test{});
        let bios_info = &WmiBios::get_specific_ex(reader.borrow())[0];
        assert_eq!(bios_info.caption, "1.23.3");
        assert_eq!(bios_info.manufacturer, "Lucerne Publishing");
        assert_eq!(bios_info.release_date, "20180126");
        assert_eq!(bios_info.serial_number, "AAAAAAAA");
        assert_eq!(bios_info.smbios_version, "1.23.3");
    }
}
