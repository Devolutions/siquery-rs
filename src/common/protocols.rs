use tables::EtcProtocols;
use regex::Regex;

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
        //regex filter to extract protocols groups: remove everything from "#" till the line break
        static ref PROTOCOLS_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
        //regex filter to extract relevant comments: keep only what follows a "#" on the lines that were previously matched
        static ref PROTOCOLS_FILE_REGEX_COMMENTS: Regex = Regex::new(r"(?m)^[^#].*#\s*(.*)").unwrap();
    }

impl EtcProtocols {

    pub(crate) fn new() -> EtcProtocols {
        EtcProtocols {
            name: String::new(),
            number: String::new(),
            alias: String::new(),
            comment: String::new(),
        }
    }

    pub fn get_protocols (system_reader: &SystemReaderInterface) -> Vec<EtcProtocols> {

        let mut protocols: Vec<EtcProtocols> = Vec::new();

        for line in system_reader.get_protocols_file().unwrap_or("".to_string()).lines() {
            let mut etc_protocols = EtcProtocols::new();
            let captures = PROTOCOLS_FILE_REGEX.captures(&line);
            if let Some(cap) = captures {
                if let Some(protocols_group) = cap.get(0) {
                    //omitting empty outputs from regex
                    if protocols_group.as_str().len() == 0 {
                        continue
                    }
                    let v: Vec<_> = protocols_group.as_str().trim().split_whitespace().collect();
                    //the ip address will always be the leftmost entry
                    etc_protocols.name = v[0].to_string();
                    etc_protocols.number = v[1].to_string();
                    if v[2].to_string().len() != 0 {
                        etc_protocols.alias = v[2].to_string();
                    };
                };
            }
            let comments = PROTOCOLS_FILE_REGEX_COMMENTS.captures(&line);
            if let Some(cap) = comments {
                if let Some(cap_comments) = cap.get(1) {
                    //omitting empty outputs from regex
                    if cap_comments.as_str().len() == 0 {
                        continue
                    }
                    etc_protocols.comment = cap_comments.as_str().to_owned();
                };
            }
            protocols.push(etc_protocols);
        }
        protocols
    }
}

