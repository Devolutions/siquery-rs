use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct EtcHosts {
    pub address: String,
    pub hostnames: String,
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
