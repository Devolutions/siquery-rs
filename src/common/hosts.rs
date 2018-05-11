use tables::EtcHosts;
use regex::Regex;
use std::net::{IpAddr};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        use linux::SystemReaderInterface;
    } else if #[cfg(target_os = "macos")] {
       use macos::SystemReaderInterface;
    } else if #[cfg(target_os = "windows")] {
        use windows::SystemReaderInterface;
    }
}

lazy_static! {
        //regex filter: remove everything from "#" till the line break
        static ref HOSTS_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
    }

impl EtcHosts {

    pub(crate) fn new() -> EtcHosts {
        EtcHosts {
            address: String::new(),
            hostnames: String::new(),
        }
    }

    pub fn get_hosts (system_reader: &SystemReaderInterface) -> Vec<EtcHosts> {

        let mut hosts: Vec<EtcHosts> = Vec::new();

        for line in system_reader.get_hosts_file().unwrap_or("".to_string()).lines() {
            let captures = HOSTS_FILE_REGEX.captures(&line);
            if let Some(cap) = captures {
                if let Some(ip_group) = cap.get(0) {
                    //omitting empty outputs from regex
                    if ip_group.as_str().len() == 0 {
                        continue
                    }
                    let mut etc_hosts = EtcHosts::new();
                    let v: Vec<_> = ip_group.as_str().trim().split_whitespace().collect();
                    //check ip for format validity
                    match v[0].parse::<IpAddr>() {
                        Ok(_r) => {
                            //the ip address will always be the leftmost entry
                            etc_hosts.address = v[0].to_string();
                            etc_hosts.hostnames = v[1..].join(",");
                            hosts.push(etc_hosts);
                        }
                        Err(_e) => {
                            continue;
                        }
                    };
                }
            }
        }
        hosts
    }
}