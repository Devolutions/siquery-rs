use tables::EtcServices;
use regex::Regex;
use std::str::FromStr;

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
    // Regex filter: remove everything from "#" till the line break
    static ref SERVICES_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
    // Regex filter to extract comments: keep everything following a "#" char
    static ref PROTOCOLS_FILE_REGEX_COMMENTS: Regex = Regex::new(r"#\s*(.*)").unwrap();
}

impl EtcServices {

    pub const COLUMN_NAMES: &'static [&'static str] = &["name", "port", "protocol", "aliases", "comment"];

    pub(crate) fn new() -> EtcServices {
        EtcServices {
            name: String::new(),
            port: 0,
            protocol: String::new(),
            aliases: String::new(),
            comment: String::new(),
        }
    }

    pub fn get_services(system_reader: &SystemReaderInterface) -> Vec<EtcServices> {

        let mut services: Vec<EtcServices> = Vec::new();

        for line in system_reader
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
}
