#[allow(unused_imports)]
// TODO implement to_json
use serde_json;
use std::fs::File;
use std::io::Read;
use std::env;
#[cfg(feature = "etc_hosts")] use tables::EtcHostsIface;
#[cfg(feature = "etc_services")] use tables::EtcServicesIface;
#[cfg(feature = "etc_protocols")] use tables::EtcProtocolsIface;
#[cfg(feature = "interface_address")] mod interface_address;
#[cfg(feature = "interface_details")] mod interface_details;
#[cfg(feature = "logical_drives")] mod logical_drive;
#[cfg(feature = "wmi_os_version")] mod wmi_os_version;
#[cfg(feature = "wmi_computer_info")] mod wmi_computer_info;
#[cfg(feature = "wmi_printers")] mod wmi_printers;
#[cfg(feature = "wmi_services")] mod wmi_services;
#[cfg(feature = "wmi_hotfixes")] mod wmi_hotfixes;
#[cfg(feature = "wmi_shares")] mod wmi_shares;
#[cfg(feature = "wmi_network_adapters")] mod wmi_network_adapters;
#[cfg(feature = "wmi_local_accounts")] mod wmi_local_accounts;
#[cfg(feature = "wmi_bios")] mod wmi_bios;
#[cfg(feature = "wmi_motherboard")] mod wmi_motherboard;
#[cfg(feature = "wmi_processor")] mod wmi_processor;
#[cfg(feature = "wmi_physical_memory")] mod wmi_physical_memory;
#[cfg(feature = "wmi_sound")] mod wmi_sound;
#[cfg(feature = "wmi_video")] mod wmi_video;
#[cfg(feature = "wmi_monitors")] mod wmi_monitors;
#[cfg(feature = "wmi_keyboard")] mod wmi_keyboard;
#[cfg(feature = "wmi_pointing_device")] mod wmi_pointing_device;
#[cfg(feature = "users")] mod users;
#[cfg(feature = "logged_in_users")] mod logged_in_users;
#[cfg(feature = "logon_sessions")] mod logon_sessions;
#[cfg(feature = "groups")] mod groups;
#[cfg(feature = "process_open_sockets")] #[cfg(not(fuzzing))] mod process_open_sockets;
#[cfg(feature = "processes")] #[cfg(not(fuzzing))] mod processes;
#[cfg(feature = "process_memory_map")] #[cfg(not(fuzzing))] mod process_memory_map;
#[cfg(feature = "os_version")] #[cfg(not(fuzzing))] mod os_version;
#[cfg(feature = "system_info")] #[cfg(not(fuzzing))] mod system_info;
#[cfg(feature = "products")] #[cfg(not(fuzzing))] mod products;
#[cfg(feature = "uptime")] #[cfg(not(fuzzing))] mod uptime;

pub struct EtcHostsReader {}
#[cfg(feature = "etc_hosts")]
impl EtcHostsIface for EtcHostsReader {
    fn get_hosts_file(&self) -> Option<String>{
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\hosts").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}

pub struct EtcProtocolsReader {}
#[cfg(feature = "etc_protocols")]
impl EtcProtocolsIface for EtcProtocolsReader {
    fn get_protocols_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\protocol").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}

pub struct EtcServicesReader {}
#[cfg(feature = "etc_services")]
impl EtcServicesIface for EtcServicesReader {
    fn get_services_file(&self) -> Option<String> {
        let mut string = String::new();
        let file_location = env::var("SYSTEMROOT").unwrap_or("".to_string()).to_string();
        File::open(file_location + "\\system32\\drivers\\etc\\services").ok()?.read_to_string(&mut string).ok()?;
        Some(string)
    }
}