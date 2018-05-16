use regex::Regex;
use std::net::IpAddr;
use tables::EtcHosts;

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

    pub fn get_hosts(system_reader: &SystemReaderInterface) -> Vec<EtcHosts> {
        let mut hosts: Vec<EtcHosts> = Vec::new();

        for line in system_reader
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
}
