use std::process::Command;
use std::borrow::Borrow;

use crate::utils;
use crate::tables::{WmiComputerInfo,WmiComputerInfoIface};

pub struct Reader {}
impl WmiComputerInfoIface for Reader {
    fn get_wmi_computer_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["computersystem", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiComputerInfo {
    pub(crate) fn new() -> WmiComputerInfo {
        WmiComputerInfo {
            computer_name: String::new(),
            domain: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            number_of_processors: 0,
            system_type: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiComputerInfoIface) -> Vec<WmiComputerInfo> {
        let mut output : Vec<WmiComputerInfo> = Vec::new();
        let mut computer = WmiComputerInfo::new();

        if let Some(computer_info) = reader.get_wmi_computer_info() {

            let lines = computer_info.split('\n');

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
                        computer.computer_name = v;
                    },
                    "Domain" => {
                        computer.domain = v;
                    },
                    "Manufacturer" => {
                        computer.manufacturer = v;
                    },
                    "Model" => {
                        computer.model = v;
                    },
                    "NumberOfProcessors" => {
                        computer.number_of_processors = v.parse::<u32>().unwrap_or(0);
                    },
                    "SystemType" => {
                        computer.system_type = v;
                    },
                    _ => {}
                }
            }
        }
        output.push(computer);
        output
    }

    pub(crate) fn get_specific() -> Vec<WmiComputerInfo> {
        let reader: Box<WmiComputerInfoIface> = Box::new(Reader{});
        let out = WmiComputerInfo::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiComputerInfoIface for Test {
        fn get_wmi_computer_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-computerinfo.txt")))
        }
    }
    #[test]
    fn test_wmi_computer_info () {
        let reader: Box<WmiComputerInfoIface> = Box::new(Test{});
        let wmi_computer_info = &WmiComputerInfo::get_specific_ex(reader.borrow())[0];
        assert_eq!(wmi_computer_info.computer_name, "Lucerne Publishing");
        assert_eq!(wmi_computer_info.domain, "STANDALONE");
        assert_eq!(wmi_computer_info.manufacturer, "Lucerne Publishing");
        assert_eq!(wmi_computer_info.model, "TailSpin Toys");
        assert_eq!(wmi_computer_info.number_of_processors, 18);
        assert_eq!(wmi_computer_info.system_type, "x128-based PC");
    }
}
