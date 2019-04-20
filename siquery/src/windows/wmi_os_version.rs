use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiOsVersion,WmiOsVersionIface};
use crate::utils;

pub struct Reader {}
impl WmiOsVersionIface for Reader {
    fn get_wmi_os_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["os", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiOsVersion {
    pub fn get_specific_ex(reader: &WmiOsVersionIface) -> Vec<WmiOsVersion> {
        let mut output : Vec<WmiOsVersion> = Vec::new();
        let mut os_version = WmiOsVersion {
            csname: String::new(),
            platform: String::from("Windows"),
            version: String::new(),
            major: String::new(),
            minor: String::new(),
            build_number: String::new(),
            caption: String::new(),
            free_physical_mem: String::new(),
            free_virtual_mem: String::new(),
            manufacturer: String::new(),
            name: String::new(),
            service_pack_major: String::new(),
            service_pack_minor: String::new(),
            size_stored_in_paging_file: String::new(),
            total_virtual_mem_size: String::new(),
            total_visible_mem_size: String::new(),
            win_directory: String::new(),
            install_date: String::new(),
            last_boot_up_time: String::new(),
            locale: String::new(),
            os_type: String::new(),
        };

        if let Some(os_info) = reader.get_wmi_os_info() {
            let lines = os_info.split('\n');

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
                    "BuildNumber" => {
                        os_version.build_number = v;
                    },
                    "CSName" => {
                        os_version.csname = v;
                    },
                    "Caption" => {
                        os_version.caption = v;
                    },
                    "FreePhysicalMemory" => {
                        os_version.free_physical_mem = v;
                    },
                    "FreeVirtualMemory" => {
                        os_version.free_virtual_mem = v;
                    },
                    "Version" => {
                        os_version.version = v;
                        let n: Vec<_> = os_version.version.split(".").collect();
                        if n.len() >= 2 {
                            os_version.major = n[0].to_string();
                            os_version.minor = n[1].to_string();
                        }
                    },
                    "Manufacturer" => {
                        os_version.manufacturer = v;
                    },
                    "Name" => {
                        let n: Vec<_> = v.split("|").collect();
                        if n.len() >= 1 {
                            os_version.name = n[0].to_string();
                        }
                    },
                    "ServicePackMajorVersion" => {
                        os_version.service_pack_major = v;
                    },
                    "ServicePackMinorVersion" => {
                        os_version.service_pack_minor = v;
                    },
                    "SizeStoredInPagingFiles" => {
                        os_version.size_stored_in_paging_file = v;
                    },
                    "TotalVirtualMemorySize" => {
                        os_version.total_virtual_mem_size = v;
                    },
                    "TotalVisibleMemorySize" => {
                        os_version.total_visible_mem_size = v;
                    },
                    "WindowsDirectory" => {
                        os_version.win_directory = v;
                    },
                    "InstallDate" => {
                        os_version.install_date = v;
                    },

                    "LastBootUpTime" => {
                        os_version.last_boot_up_time = v;
                    },

                    "Locale" => {
                        os_version.locale = v;
                    },

                    "OSType" => {
                        os_version.os_type = v;
                    },
                    _ => ()
                }
            }
        }

        output.push(os_version);
        output
    }

    pub(crate) fn get_specific() -> Vec<WmiOsVersion> {
        let reader: Box<WmiOsVersionIface> = Box::new(Reader{});
        let out = WmiOsVersion::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiOsVersionIface for Test {
        fn get_wmi_os_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-os-version.txt")))
        }
    }
    #[test]
    fn test_wmi_os_version () {
        let reader: Box<WmiOsVersionIface> = Box::new(Test{});
        let wmi_os_version = &WmiOsVersion::get_specific_ex(reader.borrow())[0];
        assert_eq!(wmi_os_version.platform, "Windows");
        assert_eq!(wmi_os_version.csname, "Olympia");
        assert_eq!(wmi_os_version.version, "10.10.16299");
        assert_eq!(wmi_os_version.major, "10");
        assert_eq!(wmi_os_version.minor, "10");
        assert_eq!(wmi_os_version.build_number, "9999");
        assert_eq!(wmi_os_version.caption, "describe something here");
        assert_eq!(wmi_os_version.free_physical_mem, "10138896");
        assert_eq!(wmi_os_version.free_virtual_mem, "10900164");
        assert_eq!(wmi_os_version.manufacturer, "Wide World Importers");
        assert_eq!(wmi_os_version.name, "Wide World Importers 10 Home");
        assert_eq!(wmi_os_version.service_pack_major, "0");
        assert_eq!(wmi_os_version.service_pack_minor, "0");
        assert_eq!(wmi_os_version.size_stored_in_paging_file, "2490368");
        assert_eq!(wmi_os_version.total_virtual_mem_size, "19134092");
        assert_eq!(wmi_os_version.total_visible_mem_size, "16643724");
        assert_eq!(wmi_os_version.win_directory, "C:\\WINDOWS");
    }
}