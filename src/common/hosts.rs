use tables::EtcHosts;
use regex::Regex;
use std::net::{IpAddr};
use windows::SystemReaderInterface;

impl EtcHosts {

    pub(crate) fn new() -> EtcHosts {
        EtcHosts {
            address: String::new(),
            hostnames: String::new(),
        }
    }

    pub fn get_hosts (system_reader: &SystemReaderInterface) -> Vec<EtcHosts> {

        let mut hosts: Vec<EtcHosts> = Vec::new();

        lazy_static! {
        //regex filter: remove everything from "#" till the line break
        static ref RE: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
    }

        for line in system_reader.get_hosts_file().unwrap().lines() {
            let captures = RE.captures(&line);
            if let Some(cap) = captures {
                //omitting empty outputs from regex
                if cap.get(0).unwrap().as_str().len() == 0 {
                    continue
                }
                    else {
                        let mut etc_hosts = EtcHosts::new();
                        let v: Vec<_> = cap.get(0).unwrap().as_str().trim().split_whitespace().collect();
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
        }   hosts
    }
}