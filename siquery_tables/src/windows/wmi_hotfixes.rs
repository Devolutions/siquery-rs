use tables::WmiHotfixes;
use utils;
use windows::SystemReaderInterface;

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

    pub(crate) fn get_hotfixes_info(system_reader: &SystemReaderInterface) -> Vec<WmiHotfixes> {

        let mut hotfixes: Vec<WmiHotfixes> = Vec::new();

        if let Some(hotfix_info) = system_reader.get_wmi_hotfixes_info() {

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
}