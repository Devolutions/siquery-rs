use plist::Plist;

use tables::OsVersion;
use macos::SystemReaderInterface;


impl OsVersion {
    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
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

        output.push(
            OsVersion {
                name,
                platform: String::from("MacOS"),
                version,
                major,
                minor,
            }
        );
        output
    }
}