use std::process::Command;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use uname;

use tables::{OsVersion,OsVersionIface};

pub struct Reader {
    uname_info: Option<uname::Info>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            uname_info: uname::uname().ok()
        }
    }
}

impl OsVersionIface for Reader {
    fn get_os_info(&self) -> Option<String> {
        Some(String::from("For windows only"))
    }
    fn os_release(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/os-release").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
    fn os_platform(&self) -> Option<String> {
        match self.uname_info {
            Some(ref info) => Some(info.sysname.clone()),
            None => None
        }
    }
}

impl OsVersion {
    pub(crate) fn get_specific_ex(reader: &OsVersionIface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let os_release = reader.os_release();
        let name = match os_release {
            Some(ref s) => {
                let n = s.split('\n').find(|line| line.starts_with("NAME"))
                    .and_then(|line| line.split('=').last())
                    .and_then(|val| Some(val.replace("\"", "")));
                n.unwrap_or_else(|| String::from(""))
            }

            None => String::from("")
        };

        let version = match os_release {
            Some(ref s) => {
                let n = s.split('\n').find(|line| line.starts_with("VERSION_ID"))
                    .and_then(|line| line.split('=').last())
                    .and_then(|val| Some(val.replace("\"", "")));
                n.unwrap_or_else(|| String::from(""))
            }

            None => String::from("")
        };

        let mut major = 0;
        let mut minor = 0;

        if !version.is_empty() {
            let v: Vec<_> = version.split('.').collect();
            if v.len() == 2 {
                major = v[0].parse::<u32>().unwrap_or(0);
                minor = v[1].parse::<u32>().unwrap_or(0);
            }
        }

        output.push(
            OsVersion {
                name,
                platform: reader.os_platform().unwrap_or_else(|| String::from("")),
                version,
                major,
                minor,
            }
        );
        output
    }

    pub(crate) fn get_specific() -> Vec<OsVersion> {
        let reader: Box<OsVersionIface> = Box::new(Reader::new());
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
            Some(String::from("For windows only"))
        }
        fn os_release(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/os-release.txt")))
        }
        fn os_platform(&self) -> Option<String> {
            Some(String::from("Linux"))
        }
    }
    #[test]
    fn test_system_info () {
        let reader: Box<OsVersionIface> = Box::new(Test{});
        let os_version = &OsVersion::get_specific_ex(reader.borrow())[0];
        assert_eq!(os_version.platform, "Linux");
        assert_eq!(os_version.name, "Ubuntu");
        assert_eq!(os_version.version, "17.10");
        assert_eq!(os_version.major, 17);
        assert_eq!(os_version.minor, 10);}
}