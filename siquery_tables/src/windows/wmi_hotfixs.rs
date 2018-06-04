use tables::WmiHotfixs;
use utils;
use windows::SystemReaderInterface;

impl WmiHotfixs {
    pub(crate) fn new() -> WmiHotfixs {
        WmiHotfixs {
            caption: String::new(),
            csname: String::new(),
            description: String::new(),
            hotfixe_id: String::new(),
            installed_by: String::new(),
            installed_on: String::new(),
        }
    }

    pub(crate) fn get_hotfixs_info(system_reader: &SystemReaderInterface) -> Vec<WmiHotfixs> {

        let mut hotfixs: Vec<WmiHotfixs> = Vec::new();

        if let Some(hotfix_info) = system_reader.get_wmi_hotfixs_info() {

            let mut hotfix = WmiHotfixs::new();
            let lines = hotfix_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if hotfix.caption != "" {
                        hotfixs.push(hotfix);
                    }
                    hotfix = WmiHotfixs::new();
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
                        hotfix.hotfixe_id = v;
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
        hotfixs
    }
}