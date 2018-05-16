use plist::Plist;
use serde_json;

use tables::OsVersion;
use macos::SystemReaderInterface;


impl OsVersion {
    pub(crate) fn new(system_reader: &SystemReaderInterface) -> OsVersion {
        let system_version = system_reader.system_version();

        let mut name = String::new();
        let mut version = String::new();
        let mut major = 0;
        let mut minor = 0;
        if let Some(s) = system_version {
            if let Ok(Plist::Dict(dict)) = Plist::from_xml_reader(&mut s.as_bytes()) {
                    if let Some(&Plist::String(ref n)) = dict.get("ProductName") {
                        name = n.clone();
                    }

                    if let Some(&Plist::String(ref n)) = dict.get("ProductVersion") {
                        version = n.clone();
                        let v: Vec<_> = version.split(".").collect();
                        if v.len() >= 2 {
                            major = v[0].parse::<u32>().unwrap_or(0);
                            minor = v[1].parse::<u32>().unwrap_or(0);
                        }
                    }
                }
        }

        OsVersion {
            name,
            platform: String::from("MacOS"),
            version,
            major,
            minor,
        }
    }
}