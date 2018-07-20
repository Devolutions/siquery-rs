#[allow(unused_imports)]
// TODO implement to_json
use serde_json;
use std::fs::File;
use std::io::Read;
use tables::{
    EtcHostsIface,
    EtcProtocolsIface,
    EtcServicesIface,
};
use std::env;

mod interface_address;
mod interface_details;
mod logical_drive;
mod os_version;
mod system_info;
mod wmi_os_version;
mod wmi_computer_info;
mod uptime;
mod wmi_printers;
mod wmi_services;
mod wmi_hotfixes;
mod products;
mod wmi_shares;
mod wmi_network_adapters;
mod wmi_local_accounts;
mod wmi_bios;
mod wmi_motherboard;
mod wmi_processor;
mod wmi_physical_memory;
mod wmi_sound;
mod wmi_video;
mod wmi_monitors;
mod wmi_keyboard;
mod wmi_pointing_device;
mod process_open_sockets;
mod processes;
mod process_memory_map;

pub struct EtcHostsReader {}
impl EtcHostsIface for EtcHostsReader {
    fn get_hosts_file(&self) -> Option<String>{
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\hosts").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}

pub struct EtcProtocolsReader {}
impl EtcProtocolsIface for EtcProtocolsReader {
    fn get_protocols_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\protocol").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}

pub struct EtcServicesReader {}
impl EtcServicesIface for EtcServicesReader {
    fn get_services_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\services").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}