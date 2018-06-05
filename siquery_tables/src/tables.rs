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

#[derive(Serialize)]
pub struct SystemInfoData {
    pub computer_name: String,
    pub cpu_brand: String,
    pub cpu_logical_cores: u32,
    pub physical_memory: u64,
}

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
