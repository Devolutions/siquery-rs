use tables::OsVersion;
use utils;
use windows::SystemReaderInterface;

impl OsVersion {
    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let mut os_version = OsVersion {
            name: String::new(),
            platform: String::from("Windows"),
            version: String::new(),
            major: 0,
            minor: 0,
        };

        if let Some(os_info) = system_reader.get_os_info() {
            let lines = os_info.split('\n');

            for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }
                if v[0].starts_with("Caption") {
                    os_version.name = String::from(v[1]);
                    utils::trim_string(&mut os_version.name);
                } else if v[0].starts_with("Version") {
                    os_version.version = String::from(v[1]);
                    utils::trim_string(&mut os_version.version);
                    let n: Vec<_> = os_version.version.split(".").collect();
                    if n.len() >= 2 {
                        os_version.major = n[0].parse::<u32>().unwrap_or(0);
                        os_version.minor = n[1].parse::<u32>().unwrap_or(0);
                    }
                }
            }
        }

        output.push(os_version);
        output
    }
}