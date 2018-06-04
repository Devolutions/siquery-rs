use tables::WmiHotfixes;
use utils;
use windows::SystemReaderInterface;

impl WmiHotfixes {
    pub(crate) fn new() -> WmiHotfixes {
        WmiHotfixes {
            caption: String::new(),
            csname: String::new(),
            description: String::new(),
            hotfixe_id: String::new(),
            installed_by: String::new(),
            installed_on: String::new(),
        }
    }

    pub(crate) fn get_hotfixes_info(system_reader: &SystemReaderInterface) -> Vec<WmiHotfixes> {

        let mut hotfixes: Vec<WmiHotfixes> = Vec::new();

        if let Some(hotfixe_info) = system_reader.get_wmi_hotfixes_info() {

            let mut hotfixe = WmiHotfixes::new();
            let lines = hotfixe_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if hotfixe.caption != "" {
                        hotfixes.push(hotfixe);
                    }
                    hotfixe = WmiHotfixes::new();
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
                        hotfixe.caption = v;
                    },
                    "CSName" => {
                        hotfixe.csname = v;
                    },
                    "Description" => {
                        hotfixe.description = v;
                    },
                    "HotFixID" => {
                        hotfixe.hotfixe_id = v;
                    },
                    "InstalledBy" => {
                        hotfixe.installed_by = v;
                    },
                    "InstalledOn" => {
                        hotfixe.installed_on = v;
                    },
                    _ => ()
                }
            }
        }
        hotfixes
    }
}