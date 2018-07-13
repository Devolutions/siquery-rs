use tables::OsVersion;
use linux::SystemReaderInterface;

impl OsVersion {
    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<OsVersion> {
        let mut output : Vec<OsVersion> = Vec::new();
        let os_release = system_reader.os_release();
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
                platform: system_reader.os_platform().unwrap_or_else(|| String::from("")),
                version,
                major,
                minor,
            }
        );
        output
    }
}