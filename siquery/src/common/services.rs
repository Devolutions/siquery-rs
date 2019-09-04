use crate::tables::{EtcServices,EtcServicesIface};
use regex::Regex;
use std::str::FromStr;
use std::borrow::Borrow;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        use crate::linux::EtcServicesReader;
    } else if #[cfg(target_os = "macos")] {
       use crate::macos::EtcServicesReader;
    } else if #[cfg(target_os = "windows")] {
        use crate::windows::EtcServicesReader;
    }
}

lazy_static! {
    // Regex filter: remove everything from "#" till the line break
    static ref SERVICES_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
    // Regex filter to extract comments: keep everything following a "#" char
    static ref PROTOCOLS_FILE_REGEX_COMMENTS: Regex = Regex::new(r"#\s*(.*)").unwrap();
}

impl EtcServices {

    pub fn new() -> EtcServices {
        EtcServices {
            name: String::new(),
            port: 0,
            protocol: String::new(),
            aliases: String::new(),
            comment: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &dyn EtcServicesIface) -> Vec<EtcServices> {

        let mut services: Vec<EtcServices> = Vec::new();

        for line in reader
            .get_services_file()
            .unwrap_or_else(|| "".to_string())
            .lines() {

            let mut etc_services = EtcServices::new();

            let captures = SERVICES_FILE_REGEX.captures(&line);

            if let Some(cap) = captures {

                if let Some(services_group) = cap.get(0) {

                    // Omitting empty outputs from regex
                    if services_group.as_str().is_empty() {
                        continue;
                    }

                    let v: Vec<_> = services_group
                        .as_str()
                        .trim()
                        .split_whitespace()
                        .collect();

                    // Check entry for validity
                    // <service name>  <port number>/<protocol>  [aliases...]   [#<comment>]
                    if v.len() < 2 {
                        continue;
                    }

                    // Split <port number>/<protocol>
                    let v_1: Vec<_> = v[1].split('/').collect();

                    // Check <port number>/<protocol> format validity
                    if v_1.len() < 2 {
                        continue;
                    }

                    etc_services.name = v[0].to_string();
                    etc_services.port = u16::from_str(v_1[0]).unwrap_or(0);
                    etc_services.protocol = v_1[1].to_string();

                    // Get aliases if they exist
                    if let Some(_alias) = v.get(2) {
                        etc_services.aliases = v[2..].join(" ");
                    };
                };

                // Nested to extract relevant comments
                let comments = PROTOCOLS_FILE_REGEX_COMMENTS.captures(&line);

                if let Some(cap) = comments {

                    if let Some(captured_comments) = cap.get(1) {

                        // Omitting empty outputs from regex
                        if captured_comments.as_str().is_empty() {
                            continue;
                        }

                        etc_services.comment = captured_comments.as_str().trim().to_owned();
                    };
                }
            }
            services.push(etc_services);
        }
        services
    }

    pub fn get_specific() -> Vec<EtcServices> {
        let reader: Box<dyn EtcServicesIface> = Box::new(EtcServicesReader{});
        let out = EtcServices::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct EtcServicesTest {}
    impl EtcServicesIface for EtcServicesTest {
        fn get_services_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/services.txt")))
        }
    }
    #[test]
    fn test_etc_services() {
        let reader: Box<dyn EtcServicesIface> = Box::new(EtcServicesTest {});
        let etc_services = EtcServices::get_specific_ex(reader.borrow());
        assert_eq!(etc_services.get(0).unwrap().name, "echo");
        assert_eq!(etc_services.get(0).unwrap().port, 7);
        assert_eq!(etc_services.get(0).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(0).unwrap().aliases, "");
        assert_eq!(etc_services.get(0).unwrap().comment, "");
        assert_eq!(etc_services.get(2).unwrap().name, "discard");
        assert_eq!(etc_services.get(2).unwrap().port, 9);
        assert_eq!(etc_services.get(2).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(2).unwrap().aliases, "sink null");
        assert_eq!(etc_services.get(2).unwrap().comment, "");
        assert_eq!(etc_services.get(12).unwrap().name, "ftp-data");
        assert_eq!(etc_services.get(12).unwrap().port, 20);
        assert_eq!(etc_services.get(12).unwrap().protocol, "tcp");
        assert_eq!(etc_services.get(12).unwrap().aliases, "");
        assert_eq!(etc_services.get(12).unwrap().comment, "FTP, data");
        assert_eq!(etc_services.len(), 15);
    }
}