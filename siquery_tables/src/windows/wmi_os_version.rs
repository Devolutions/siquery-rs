use tables::WmiOsVersion;
use utils;
use windows::SystemReaderInterface;

impl WmiOsVersion {
    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<WmiOsVersion> {
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
        };

        if let Some(os_info) = system_reader.get_wmi_os_info() {
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
                    _ => ()
                }
            }
        }

        output.push(os_version);
        output
    }
}