use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiMotherboard,WmiMotherboardIface};
use crate::utils;

pub struct Reader {}
impl WmiMotherboardIface for Reader {
    fn get_wmi_motherboard_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["baseboard", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiMotherboard {
    pub fn get_specific_ex(reader: &dyn WmiMotherboardIface) -> Vec<WmiMotherboard> {
        let mut output : Vec<WmiMotherboard> = Vec::new();
        let mut motherboard = WmiMotherboard {
            name: String::new(),
            manufacturer: String::new(),
            product: String::new(),
            serial_number: String::new(),
            version: String::new(),
        };
        if let Some(motherboard_info) = reader.get_wmi_motherboard_info() {
            let lines = motherboard_info.split('\n');

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
                    "Name" => {
                        motherboard.name = v;
                    },
                    "Manufacturer" => {
                        motherboard.manufacturer = v;
                    },
                    "Product" => {
                        motherboard.product = v;
                    },
                    "SerialNumber" => {
                        motherboard.serial_number = v;
                    },
                    "Version" => {
                        motherboard.version = v;
                    },
                    _ => ()
                }
            }
        }

        output.push(motherboard);
        output
    }

    pub(crate) fn get_specific() -> Vec<WmiMotherboard> {
        let reader: Box<dyn WmiMotherboardIface> = Box::new(Reader{});
        let out = WmiMotherboard::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiMotherboardIface for Test {
        fn get_wmi_motherboard_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-motherboard-info.txt")))
        }
    }
    #[test]
    fn test_wmi_motherboard () {
        let reader: Box<dyn WmiMotherboardIface> = Box::new(Test{});
        let motherboard_info = &WmiMotherboard::get_specific_ex(reader.borrow())[0];
        assert_eq!(motherboard_info.name, "Base Board");
        assert_eq!(motherboard_info.manufacturer, " The Phone Company");
        assert_eq!(motherboard_info.product, " 958B84C99");
        assert_eq!(motherboard_info.serial_number, " /D8D8DH2/ETFSC0070C000T/");
        assert_eq!(motherboard_info.version, " A11");
    }
}