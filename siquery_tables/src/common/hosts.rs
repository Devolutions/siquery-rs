use regex::Regex;
use std::net::IpAddr;
use tables::{EtcHosts,EtcHostsIface};
use std::borrow::Borrow;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        use linux::EtcHostsReader;
    } else if #[cfg(target_os = "macos")] {
       use macos::EtcHostsReader;
    } else if #[cfg(target_os = "windows")] {
        use windows::EtcHostsReader;
    }
}

lazy_static! {
    //regex filter: remove everything from "#" till the line break
    static ref HOSTS_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
}



impl EtcHosts {

    pub fn new() -> EtcHosts {
        EtcHosts{
            address: String::new(),
            hostnames: String::new(),
        }
    }

    pub fn get_specific_ex(hosts_reader: &EtcHostsIface) -> Vec<EtcHosts> {
        let mut hosts: Vec<EtcHosts> = Vec::new();

        for line in hosts_reader
            .get_hosts_file()
            .unwrap_or_else(|| "".to_string())
            .lines()
        {
            let captures = HOSTS_FILE_REGEX.captures(&line);
            if let Some(cap) = captures {
                if let Some(ip_group) = cap.get(0) {
                    // Omitting empty outputs from regex.
                    if ip_group.as_str().is_empty() {
                        continue;
                    }
                    let v: Vec<_> = ip_group.as_str().trim().split_whitespace().collect();
                    
                    if v.len() < 2 {
                        continue;
                    }

                    // Check the ip for format validity.
                    if v[0].parse::<IpAddr>().is_ok() {
                        hosts.push(EtcHosts {
                            address: v[0].to_string(),
                            hostnames: v[1..].join(","),
                        });
                    };
                }
            }
        }
        hosts
    }

    pub fn get_specific() -> Vec<EtcHosts> {
        let hosts_reader: Box<EtcHostsIface> = Box::new(EtcHostsReader{});
        let out = EtcHosts::get_specific_ex(hosts_reader.borrow());
        out
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    pub struct EtcHostsTest {}

    impl EtcHostsTest {
        fn new () -> EtcHostsTest {
            EtcHostsTest {}
        }
    }

    impl EtcHostsIface for EtcHostsTest {
        fn get_hosts_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/hosts.txt")))
        }
    }

    #[test]
    fn test_etc_hosts() {
        let hosts_reader: Box<EtcHostsIface> = Box::new(EtcHostsTest::new());
        let etc_hosts = EtcHosts::get_specific_ex(hosts_reader.borrow());
        assert_eq!(etc_hosts.get(0).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(0).unwrap().hostnames, "localhost");
        assert_eq!(etc_hosts.get(1).unwrap().address, "255.255.255.255");
        assert_eq!(etc_hosts.get(1).unwrap().hostnames, "broadcasthost");
        assert_eq!(etc_hosts.get(2).unwrap().address, "::1");
        assert_eq!(etc_hosts.get(2).unwrap().hostnames, "localhost");
        assert_eq!(etc_hosts.get(3).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(3).unwrap().hostnames, "example.com,example");
        assert_eq!(etc_hosts.get(4).unwrap().address, "127.0.0.1");
        assert_eq!(etc_hosts.get(4).unwrap().hostnames, "example.net");
        assert_eq!(etc_hosts.len(), 5);
    }
}