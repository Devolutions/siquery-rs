use regex::Regex;
use tables::EtcProtocols;
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
    //regex filter to extract protocols groups: remove everything from "#" till the line break
    static ref PROTOCOLS_FILE_REGEX: Regex = Regex::new(r"(?m)^([^#]*)").unwrap();
    //regex filter to extract comments: keep everything following a "#"
    static ref PROTOCOLS_FILE_REGEX_COMMENTS: Regex = Regex::new(r"#\s*(.*)").unwrap();
}

impl EtcProtocols {

    pub const COLUMN_NAMES: &'static [&'static str] = &["name", "number", "alias", "comment"];

    pub fn new() -> EtcProtocols {
        EtcProtocols {
            name: String::new(),
            number: 0,
            alias: String::new(),
            comment: String::new(),
        }
    }

    pub fn get_protocols(system_reader: &SystemReaderInterface) -> Vec<EtcProtocols> {
        let mut protocols: Vec<EtcProtocols> = Vec::new();

        for line in system_reader
            .get_protocols_file()
            .unwrap_or_else(|| "".to_string())
            .lines()
        {
            let mut etc_protocols = EtcProtocols::new();

            let captures = PROTOCOLS_FILE_REGEX.captures(&line);
            if let Some(cap) = captures {
                if let Some(protocols_group) = cap.get(0) {
                    //omitting empty outputs from regex
                    if protocols_group.as_str().is_empty() {
                        continue;
                    }

                    let v: Vec<_> = protocols_group.as_str().trim().split_whitespace().collect();

                    //check entry for validity
                    //<protocol name>  <assigned number>  [aliases...]   [#<comment>]
                    if v.len() <= 2 {
                        continue;
                    }

                    etc_protocols.name = v[0].to_string();
                    etc_protocols.number = u16::from_str(v[1]).unwrap_or(0);

                    //get alias if it exists
                    if let Some(alias) = v.get(2) {
                        etc_protocols.alias = alias.to_string();
                    }
                };

                //nested to match only relevant comments
                let comments = PROTOCOLS_FILE_REGEX_COMMENTS.captures(&line);
                if let Some(cap) = comments {
                    if let Some(captured_comments) = cap.get(1) {
                        //omitting empty outputs from regex
                        if !captured_comments.as_str().is_empty() {
                            etc_protocols.comment = captured_comments.as_str().to_owned();
                        }
                    };
                }
            };
            protocols.push(etc_protocols);
        }
        protocols
    }
}
