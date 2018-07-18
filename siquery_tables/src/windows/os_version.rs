use std::process::Command;
use std::borrow::Borrow;

use tables::{OsVersion,OsVersionIface};
use utils;

pub struct Reader {}
impl OsVersionIface for Reader {
    fn get_os_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["os", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl OsVersion {
    pub(crate) fn get_specific_ex(reader: &OsVersionIface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let mut os_version = OsVersion {
            name: String::new(),
            platform: String::from("Windows"),
            version: String::new(),
            major: 0,
            minor: 0,
        };

        if let Some(os_info) = reader.get_os_info() {
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
            Some(String::from(include_str!("../../test_data/wmi-osinfo.txt")))
        }
    }
    #[test]
    fn test_os_version () {
        let reader: Box<OsVersionIface> = Box::new(Test{});
        let os_version = &OsVersion::get_specific_ex(reader.borrow())[0];
        assert_eq!(os_version.platform, "Windows");
        assert_eq!(os_version.name, "Microsoft Windows 10 Pro");
        assert_eq!(os_version.version, "10.0.16299");
        assert_eq!(os_version.major, 10);
        assert_eq!(os_version.minor, 0);
    }
}