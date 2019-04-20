use std::fs::File;
use std::io::Read;
#[allow(unused_imports)]
// TODO implement to_json
use serde_json;
#[cfg(feature = "logical_drive")]       mod logical_drive;
#[cfg(feature = "os_version")]          mod os_version;
#[cfg(feature = "system_info")]         mod system_info;
#[cfg(feature = "uptime")]              mod uptime;
#[cfg(feature = "processes")]           mod processes;
#[cfg(feature = "process_envs")]        mod process_envs;
#[cfg(feature = "etc_hosts")]           use crate::tables::EtcHostsIface;
#[cfg(feature = "etc_services")]        use crate::tables::EtcServicesIface;
#[cfg(feature = "etc_protocols")]       use crate::tables::EtcProtocolsIface;
#[cfg(feature = "mounts")]              mod mounts;
#[cfg(feature = "users")]               mod users;
#[cfg(feature = "groups")]              mod groups;
#[cfg(feature = "proxies")]             mod proxies;
#[cfg(feature = "logged_in_users")]     mod logged_in_users;
#[cfg(feature = "launchd")]             mod launchd;
#[cfg(feature = "launchd_overrides")]   mod launchd_overrides;


pub struct EtcHostsReader {}
#[cfg(feature = "etc_hosts")]
impl EtcHostsIface for EtcHostsReader {
    fn get_hosts_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/hosts").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcProtocolsReader {}
#[cfg(feature = "etc_protocols")]
impl EtcProtocolsIface for EtcProtocolsReader {
    fn get_protocols_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/protocols").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcServicesReader {}
#[cfg(feature = "etc_services")]
impl EtcServicesIface for EtcServicesReader {
    fn get_services_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/services").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}
