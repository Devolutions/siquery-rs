use plist::Value;
use std::borrow::Borrow;

use crate::tables::OsVersion;

pub trait OsVersionReaderIface {
    fn get_os_info(&self) -> Option<Value>;
}

struct Reader {}

impl OsVersionReaderIface for Reader {
    fn get_os_info(&self) -> Option<Value> {
        Value::from_file("/System/Library/CoreServices/SystemVersion.plist").ok()
    }
}

impl OsVersion {
    pub(crate) fn get_specific_ex(reader: &dyn OsVersionReaderIface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let system_version = reader.get_os_info();

        let mut name = String::new();
        let mut version = String::new();
        let mut major = 0;
        let mut minor = 0;
        if let Some(Value::Dictionary(dict)) = system_version {
                    if let Some(&Value::String(ref n)) = dict.get("ProductName") {
                        name = n.clone();
                    }

                    if let Some(&Value::String(ref n)) = dict.get("ProductVersion") {
                        version = n.clone();
                        let v: Vec<_> = version.split(".").collect();
                        if v.len() >= 2 {
                            major = v[0].parse::<u32>().unwrap_or(0);
                            minor = v[1].parse::<u32>().unwrap_or(0);
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
    pub(crate) fn get_specific() -> Vec<OsVersion> {
        let reader: Box<dyn OsVersionReaderIface> = Box::new(Reader{});
        let out = OsVersion::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor,BufReader};
    pub struct Test {}
    impl OsVersionReaderIface for Test {
        fn get_os_info(&self) -> Option<Value> {
            const SYSTEM_VERSION_PLIST: &'static [u8] = include_bytes!("../../test_data/SystemVersion.plist");
            let plist_reader = BufReader::new(Cursor::new(SYSTEM_VERSION_PLIST));
            Value::from_reader(plist_reader).ok()
        }
    }
    #[test]
    fn test_os_version () {
        let reader: Box<dyn OsVersionReaderIface> = Box::new(Test{});
        let os_version = &OsVersion::get_specific_ex(reader.borrow())[0];
        assert_eq!(os_version.platform, "MacOS");
        assert_eq!(os_version.name, "Mac OS X");
        assert_eq!(os_version.version, "10.13.3");
        assert_eq!(os_version.major, 10);
        assert_eq!(os_version.minor, 13);
    }
}