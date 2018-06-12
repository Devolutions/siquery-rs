use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct EtcHosts {
    pub address: String,
    pub hostnames: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtcProtocols {
    pub name: String,
    pub number: u16,
    pub alias: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtcServices {
    pub name: String,
    pub port: u16,
    pub protocol: String,
    pub aliases: String,
    pub comment: String,
}

#[cfg(target_os = "windows")]
#[derive(Serialize)]
pub struct WmiComputerInfo {
    pub computer_name: String,
    pub domain: String,
    pub manufacturer: String,
    pub model: String,
    pub number_of_processors: String,
    pub system_type: String,
}

#[cfg(not(windows))]
#[derive(Serialize)]
pub struct SystemInfoData {
    pub computer_name: String,
    pub cpu_brand: String,
    pub cpu_logical_cores: u32,
    pub physical_memory: u64,
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize)]
pub struct OsVersion {
    pub build_number: String,
    pub csname: String,
    pub caption: String,
    pub free_physical_mem: String,
    pub free_virtual_mem: String,
    pub platform: String,
    pub version: String,
    pub major: String,
    pub manufacturer: String,
    pub minor: String,
    pub name: String,
    pub service_pack_major: String,
    pub service_pack_minor: String,
    pub size_stored_in_paging_file: String,
    pub total_virtual_mem_size: String,
    pub total_visible_mem_size: String,
    pub win_directory: String,
}

#[cfg(not(windows))]
#[derive(Serialize, Deserialize)]
pub struct OsVersion {
    pub name: String,
    pub platform: String,
    pub version: String,
    pub major: u32,
    pub minor: u32,
}

#[derive(Debug)]
pub struct LogicalDrive {
    pub device_id: String,
    pub drive_type: String,
    pub free_space: u64,
    pub size: u64,
    pub file_system: String,
}

impl Serialize for LogicalDrive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("LogicalDrive", 5)?;
        state.serialize_field("device_id", &self.device_id)?;
        state.serialize_field("type", &self.drive_type)?;
        state.serialize_field("free_space", &self.free_space)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("file_system", &self.file_system)?;
        state.end()
    }
}

#[derive(Debug)]
pub struct InterfaceAddress {
    pub interface: String,
    pub address: String,
    pub mask: String,
    pub interface_type: String,
    pub friendly_name: String,
}

impl Serialize for InterfaceAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("InterfaceAddress", 5)?;
        state.serialize_field("interface", &self.interface)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("mask", &self.mask)?;
        state.serialize_field("type", &self.interface_type)?;
        state.serialize_field("friendly_name", &self.friendly_name)?;
        state.end()
    }
}

#[derive(Debug, Serialize)]
pub struct InterfaceDetails {
    pub interface: String,
    pub mac: String,
    pub mtu: u32,
    pub enabled: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uptime {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total_seconds: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiPrinters {
    pub attributes: String,
    pub caption: String,
    pub creation_class_name: String,
    pub device_id: String,
    pub do_complete_first: String,
    pub driver_name: String,
    pub extended_printer_status: String,
    pub horizontal_resolution: String,
    pub local: String,
    pub name: String,
    pub port_name: String,
    pub printer_status: String,
    pub print_job_data_type: String,
    pub print_processor: String,
    pub priority: String,
    pub status: String,
    pub system_creation_class_name: String,
    pub system_name: String,
    pub vertical_resolution: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiServices {
    pub accept_pause: String,
    pub accept_stop: String,
    pub caption: String,
    pub creation_class_name: String,
    pub description: String,
    pub desktop_interact: String,
    pub display_name: String,
    pub error_control: String,
    pub exit_code: u32,
    pub name: String,
    pub path_name: String,
    pub service_type: String,
    pub started: String,
    pub start_mode: String,
    pub start_name: String,
    pub state: String,
    pub status: String,
    pub system_creation_class_name: String,
    pub system_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiHotfixes {
    pub caption: String,
    pub csname: String,
    pub description: String,
    pub hotfix_id: String,
    pub installed_by : String,
    pub installed_on : String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    pub install_date: String,
    pub install_location: String,
    pub help_link: String,
    pub name: String,
    pub vendor: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiNetworkAdapters {
    pub caption: String,
    pub description: String,
    pub setting_id: String,
    pub arp_always_source_route: String,
    pub arp_use_ether_snap: String,
    pub database_path: String,
    pub dead_gw_detect_enabled: String,
    pub default_ip_gateway: Vec<String>,
    pub default_tos: String,
    pub default_ttl: String,
    pub dhcp_enabled: String,
    pub dhcp_lease_expires: String,
    pub dhcp_lease_obtained: String,
    pub dhcp_server: String,
    pub dns_domain: String,
    pub dns_domain_suffix_search_order: Vec<String>,
    pub dns_enabled_for_wins_resolution: String,
    pub dns_host_name: String,
    pub dns_server_search_order: Vec<String>,
    pub domain_dns_registration_enabled: String,
    pub forward_buffer_memory: String,
    pub full_dns_registration_enabled: String,
    pub gateway_cost_metric: Vec<String>,
    pub igmp_level: String,
    pub index: String,
    pub interface_index: String,
    pub ip_address: Vec<String>,
    pub ip_connection_metric: String,
    pub ip_enabled: String,
    pub ip_filter_security_enabled: String,
    pub ip_port_security_enabled: String,
    pub ip_sec_permit_ip_protocols: Vec<String>,
    pub ip_sec_permit_tcp_ports: Vec<String>,
    pub ip_sec_permit_udp_ports: Vec<String>,
    pub ip_subnet: Vec<String>,
    pub ip_use_zero_broadcast: String,
    pub ip_x_address: String,
    pub ip_x_enabled: String,
    pub ip_x_frame_type: Vec<String>,
    pub ip_x_media_type: String,
    pub ip_x_network_number: Vec<String>,
    pub ip_x_virtual_net_number: String,
    pub keep_alive_interval: String,
    pub keep_alive_time: String,
    pub mac_address: String,
    pub mtu: String,
    pub num_forward_packets: String,
    pub pmtu_bh_detect_enabled: String,
    pub pmtu_discovery_enabled: String,
    pub service_name: String,
    pub tcp_ip_netbios_options: String,
    pub tcp_max_connect_retransmissions: String,
    pub tcp_max_data_retransmissions: String,
    pub tcp_num_connections: String,
    pub tcp_use_rfc1122_urgent_pointer: String,
    pub tcp_window_size: String,
    pub wins_enable_lm_hosts_lookup: String,
    pub wins_host_lookup_file: String,
    pub wins_primary_server: String,
    pub wins_scope_id: String,
    pub wins_secondary_server: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiShares {
    pub caption: String,
    pub description: String,
    pub name: String,
    pub path: String,
    pub status: String,
    pub _type: String,
    pub allow_maximum: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WmiLocalAccounts {
    pub account_type: String,
    pub caption: String,
    pub description: String,
    pub _domain: String,
    pub local_account: String,
    pub name: String,
    pub sid: String,
    pub sid_type: String,
    pub status: String,
}
