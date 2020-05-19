use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiHotfixes,WmiHotfixesIface};
use crate::utils;

pub struct Reader {}
impl WmiHotfixesIface for Reader {
    fn get_wmi_hotfixes_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["qfe",
                "get",
                "Caption,CSName,Description,HotFixID,InstalledBy,InstalledOn",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiHotfixes {
    pub(crate) fn new() -> WmiHotfixes {
        WmiHotfixes {
            caption: String::new(),
            csname: String::new(),
            description: String::new(),
            hotfix_id: String::new(),
            installed_by: String::new(),
            installed_on: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &dyn WmiHotfixesIface) -> Vec<WmiHotfixes> {

        let mut hotfixes: Vec<WmiHotfixes> = Vec::new();

        if let Some(hotfix_info) = reader.get_wmi_hotfixes_info() {

            let mut hotfix = WmiHotfixes::new();
            let lines = hotfix_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if hotfix.caption != "" {
                        hotfixes.push(hotfix);
                    }
                    hotfix = WmiHotfixes::new();
                }

                let v: Vec<_> = line.splitn(2, '=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Caption" => {
                        hotfix.caption = v;
                    },
                    "CSName" => {
                        hotfix.csname = v;
                    },
                    "Description" => {
                        hotfix.description = v;
                    },
                    "HotFixID" => {
                        hotfix.hotfix_id = v;
                    },
                    "InstalledBy" => {
                        hotfix.installed_by = v;
                    },
                    "InstalledOn" => {
                        hotfix.installed_on = v;
                    },
                    _ => ()
                }
            }
        }
        hotfixes
    }

    pub(crate) fn get_specific() -> Vec<WmiHotfixes> {
        let reader: Box<dyn WmiHotfixesIface> = Box::new(Reader{});
        let out = WmiHotfixes::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiHotfixesIface for Test {
        fn get_wmi_hotfixes_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-hotfixes.txt")))
        }
    }
    #[test]
    fn test_wmi_hotfixes () {
        let reader: Box<dyn WmiHotfixesIface> = Box::new(Test{});
        let wmi_hotfixes = &WmiHotfixes::get_specific_ex(reader.borrow())[0];
        assert_eq!(wmi_hotfixes.caption, "http://support.microsoft.com/?kbid=4103");
        assert_eq!(wmi_hotfixes.csname, "wakwaka");
        assert_eq!(wmi_hotfixes.description, "Update");
        assert_eq!(wmi_hotfixes.hotfix_id, "KB4103");
        assert_eq!(wmi_hotfixes.installed_by, "wakwaka\\johnCena");
        assert_eq!(wmi_hotfixes.installed_on, "5/10/2018");
    }
}