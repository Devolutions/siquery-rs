use std::fs::File;
use std::io::Read;
#[allow(unused_imports)]
// TODO implement to_json
use serde_json;

mod logical_drive;
mod os_version;
mod system_info;
mod uptime;
mod processes;
mod process_envs;

use tables::{EtcHostsIface,EtcProtocolsIface,EtcServicesIface};

pub struct EtcHostsReader {}
impl EtcHostsIface for EtcHostsReader {
    fn get_hosts_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/hosts").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcProtocolsReader {}
impl EtcProtocolsIface for EtcProtocolsReader {
    fn get_protocols_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/protocols").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

pub struct EtcServicesReader {}
impl EtcServicesIface for EtcServicesReader {
    fn get_services_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/etc/services").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}
