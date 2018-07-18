use plist::Plist;

use tables::{OsVersion,OsVersionIface};
use macos::SystemReaderInterface;

pub struct Reader {}
impl OsVersionIface for Reader {
    fn get_os_info(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/System/Library/CoreServices/SystemVersion.plist").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

impl OsVersion {
    pub(crate) fn get_specific_ex(system_reader: &SystemReaderInterface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let system_version = system_reader.get_os_info();

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
    pub(crate) fn get_specific() -> Vec<OsVersion> {
        let reader: Box<OsVersionIface> = Box::new(Reader{});
        let out = OsVersion::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl OsVersionIface for Test {
        fn get_os_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/SystemVersion.plist")))
        }
    }
    #[test]
    fn test_os_version () {
        let system_reader: Box<SystemReaderInterface> = Box::new(MockSystemReader {});

        //os_version
        let os_version = &OsVersion::get_specific(system_reader.borrow())[0];
        assert_eq!(os_version.platform, "MacOS");
        assert_eq!(os_version.name, "Mac OS X");
        assert_eq!(os_version.version, "10.13.3");
        assert_eq!(os_version.major, 10);
        assert_eq!(os_version.minor, 13);
    }
}