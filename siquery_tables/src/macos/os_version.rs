use plist::Plist;
use std::fs::File;
use std::borrow::Borrow;

use tables::OsVersion;

pub trait OsVersionReaderIface {
    fn get_os_info(&self) -> Option<Plist>;
}

struct Reader {}

impl OsVersionReaderIface for Reader {
    fn get_os_info(&self) -> Option<Plist> {
        File::open("/System/Library/CoreServices/SystemVersion.plist").ok()
            .and_then(|file| Plist::read(file).ok())
    }
}

impl OsVersion {
    pub(crate) fn get_specific_ex(reader: &OsVersionReaderIface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let system_version = reader.get_os_info();

        let mut name = String::new();
        let mut version = String::new();
        let mut major = 0;
        let mut minor = 0;
        if let Some(Plist::Dictionary(dict)) = system_version {
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
        let reader: Box<OsVersionReaderIface> = Box::new(Reader{});
        let out = OsVersion::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl OsVersionReaderIface for Test {
        fn get_os_info(&self) -> Option<Plist> {
            File::open("../siquery_tables/test_data/SystemVersion.plist").ok()
                .and_then(|file| Plist::read(file).ok())
        }
    }
    #[test]
    fn test_os_version () {
        let reader: Box<OsVersionReaderIface> = Box::new(Test{});
        let os_version = &OsVersion::get_specific_ex(reader.borrow())[0];
        assert_eq!(os_version.platform, "MacOS");
        assert_eq!(os_version.name, "Mac OS X");
        assert_eq!(os_version.version, "10.13.3");
        assert_eq!(os_version.major, 10);
        assert_eq!(os_version.minor, 13);
    }
}