use serde::ser::{Serialize, SerializeStruct, Serializer};

pub trait Table {
    const COLUMN_NAMES: &'static [&'static str];

    fn column_names(&self) -> &'static [&'static str] {
        Self::COLUMN_NAMES
    }

    fn get_by_name(&self, _name: &str) -> String;
    fn get_by_id(&self, _id: u64) -> String;
    fn get_id(&self, _name: &str) -> u64;
}

impl<T: Table> Table for Vec<T> {
    const COLUMN_NAMES: &'static [&'static str] = T::COLUMN_NAMES;

    fn get_by_name(&self, _name: &str) -> String {
        unimplemented!()
    }
    fn get_by_id(&self, _id: u64) -> String {
        unimplemented!()
    }
    fn get_id(&self, _name: &str) -> u64 {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dummy {}

impl Dummy {}

impl Table for Dummy {
    const COLUMN_NAMES: &'static [&'static str] = &[];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcHosts {
    pub address: String,
    pub hostnames: String,
}

impl EtcHosts {
    const ADDRESS_ID: u64 = 0x00000001;
    const HOSTNAMES_ID: u64 = 0x00000002;
}

impl Table for EtcHosts {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "address", "hostnames"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "address" => self.address.clone(),
            "hostnames" => self.hostnames.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::ADDRESS_ID => self.address.clone(),
            Self::HOSTNAMES_ID => self.hostnames.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "address" => Self::ADDRESS_ID as u64,
            "hostnames" => Self::HOSTNAMES_ID as u64,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcProtocols {
    pub name: String,
    pub number: u16,
    pub alias: String,
    pub comment: String,
}

#[allow(non_upper_case_globals)]
impl EtcProtocols {
    const NAME_ID: u64 = 0x00000001;
    const NUMBER_ID: u64 = 0x00000002;
    const ALIAS_ID: u64 = 0x00000004;
    const COMMENT_ID: u64 = 0x00000008;
}

impl Table for EtcProtocols {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name", "number", "alias", "comment"];

    fn get_by_name(&self, _name: &str) -> String {
        let value = match _name {
            "name" => self.name.clone(),
            "number" => self.number.to_string(),
            "alias" => self.alias.clone(),
            "comment" => self.comment.clone(),
            _ => "".to_string()
        };
        value
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::NUMBER_ID => self.number.to_string(),
            Self::ALIAS_ID => self.alias.clone(),
            Self::COMMENT_ID => self.comment.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "number" => Self::NUMBER_ID,
            "alias" => Self::ALIAS_ID,
            "comment" => Self::COMMENT_ID,
            _ => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcServices {
    pub name: String,
    pub port: u16,
    pub protocol: String,
    pub aliases: String,
    pub comment: String,
}

#[allow(non_upper_case_globals)]
impl EtcServices {
    const NAME_ID: u64 = 0x00000001;
    const PORT_ID: u64 = 0x00000002;
    const PROTOCOL_ID: u64 = 0x00000004;
    const ALIASES_ID: u64 = 0x00000008;
    const COMMENT_ID: u64 = 0x00000010;
}

impl Table for EtcServices {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name", "port", "protocol", "aliases", "comment"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "port" => self.port.to_string(),
            "protocol" => self.protocol.clone(),
            "aliases" => self.aliases.clone(),
            "comment" => self.comment.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::PORT_ID => self.port.to_string(),
            Self::PROTOCOL_ID => self.protocol.clone(),
            Self::ALIASES_ID => self.aliases.clone(),
            Self::COMMENT_ID => self.comment.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "port" => Self::PORT_ID,
            "protocol" => Self::PROTOCOL_ID,
            "aliases" => Self::ALIASES_ID,
            "comment" => Self::COMMENT_ID,
            _ => 0
        }
    }
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

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl WmiComputerInfo {
    const COMPUTER_NAME_ID: u64 = 0x00000001;
    const DOMAIN_ID: u64 = 0x00000002;
    const MANUFACTURER_ID: u64 = 0x00000004;
    const MODEL_ID: u64 = 0x00000008;
    const NUMBER_OF_PROCESSORS_ID: u64 = 0x00000010;
    const SYSTEM_TYPE_ID: u64 = 0x00000020;
}

#[cfg(target_os = "windows")]
impl Table for WmiComputerInfo {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "computer_name",
        "domain",
        "manufacturer",
        "model",
        "number_of_processors",
        "system_type"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "computer_name" => self.computer_name.clone(),
            "domain" => self.domain.to_string(),
            "manufacturer" => self.manufacturer.clone(),
            "model" => self.model.clone(),
            "number_of_processors" => self.number_of_processors.clone(),
            "system_type" => self.system_type.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::COMPUTER_NAME_ID => self.computer_name.clone(),
            Self::DOMAIN_ID => self.domain.to_string(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::MODEL_ID => self.model.clone(),
            Self::NUMBER_OF_PROCESSORS_ID => self.number_of_processors.clone(),
            Self::SYSTEM_TYPE_ID => self.system_type.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "computer_name" => Self::COMPUTER_NAME_ID,
            "domain" => Self::DOMAIN_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "model" => Self::MODEL_ID,
            "number_of_processors" => Self::NUMBER_OF_PROCESSORS_ID,
            "system_type" => Self::SYSTEM_TYPE_ID,
            _ => 0
        }
    }
}

#[derive(Serialize)]
pub struct SystemInfoData {
    pub computer_name: String,
    pub cpu_brand: String,
    pub cpu_logical_cores: u32,
    pub physical_memory: u64,
}

#[allow(non_upper_case_globals)]
impl SystemInfoData {
    const COMPUTER_NAME_ID: u64 = 0x00000001;
    const CPU_BRAND_ID: u64 = 0x00000002;
    const CPU_LOGICAL_CORES_ID: u64 = 0x00000004;
    const PHYSICAL_MEMORY_ID: u64 = 0x00000008;
}

impl Table for SystemInfoData {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "computer_name",
        "cpu_brand",
        "cpu_logical_cores",
        "physical_memory"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "computer_name" => self.computer_name.clone(),
            "cpu_brand" => self.cpu_brand.clone(),
            "cpu_logical_cores" => self.cpu_logical_cores.to_string(),
            "physical_memory" => self.physical_memory.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::COMPUTER_NAME_ID => self.computer_name.clone(),
            Self::CPU_BRAND_ID => self.cpu_brand.clone(),
            Self::CPU_LOGICAL_CORES_ID => self.cpu_logical_cores.to_string(),
            Self::PHYSICAL_MEMORY_ID => self.physical_memory.to_string(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "computer_name" => Self::COMPUTER_NAME_ID,
            "cpu_brand" => Self::CPU_BRAND_ID,
            "cpu_logical_cores" => Self::CPU_LOGICAL_CORES_ID,
            "physical_memory" => Self::PHYSICAL_MEMORY_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize)]
pub struct WmiOsVersion {
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

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl WmiOsVersion {
    const BUILDER_NUMBER_ID: u64 = 0x00000001;
    const CSNAME_ID: u64 = 0x00000002;
    const CAPTION_ID: u64 = 0x00000004;
    const FREE_PHYSICAL_MEMORY_ID: u64 = 0x00000008;
    const FREE_VIRTUAL_MEMORY_ID: u64 = 0x00000010;
    const PLATFORM_ID: u64 = 0x00000020;
    const VERSION_ID: u64 = 0x00000040;
    const MAJOR_ID: u64 = 0x00000080;
    const MANUFACTURER_ID: u64 = 0x00000100;
    const MINOR_ID: u64 = 0x00000200;
    const NAME_ID: u64 = 0x00000400;
    const SERVICE_PACK_MAJOR_ID: u64 = 0x00000800;
    const SERVICE_PACK_MINOR_ID: u64 = 0x00001000;
    const SIZE_STORED_IN_PAGING_FILE_ID: u64 = 0x00002000;
    const TOTAL_VIRTUAL_MEM_SIZE_ID: u64 = 0x00004000;
    const TOTAL_VISIBLE_MEM_SIZE_ID: u64 = 0x00008000;
    const WIN_DIRECTORY_ID: u64 = 0x00010000;
}

#[cfg(target_os = "windows")]
impl Table for WmiOsVersion {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "build_number",
        "csname",
        "caption",
        "free_physical_mem",
        "free_virtual_mem",
        "platform",
        "version",
        "major",
        "manufacturer",
        "minor",
        "name",
        "service_pack_major",
        "service_pack_minor",
        "size_stored_in_paging_file",
        "total_virtual_mem_size",
        "total_visible_mem_size",
        "win_directory"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "build_number" => self.build_number.clone(),
            "csname" => self.csname.to_string(),
            "caption" => self.caption.clone(),
            "free_physical_mem" => self.free_physical_mem.clone(),
            "free_virtual_mem" => self.free_virtual_mem.clone(),
            "platform" => self.platform.clone(),
            "version" => self.version.to_string(),
            "major" => self.major.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "minor" => self.minor.clone(),
            "name" => self.name.to_string(),
            "service_pack_major" => self.service_pack_major.clone(),
            "service_pack_minor" => self.service_pack_minor.clone(),
            "size_stored_in_paging_file" => self.size_stored_in_paging_file.clone(),
            "total_virtual_mem_size" => self.total_virtual_mem_size.to_string(),
            "total_visible_mem_size" => self.total_visible_mem_size.clone(),
            "win_directory" => self.win_directory.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::BUILDER_NUMBER_ID => self.build_number.clone(),
            Self::CSNAME_ID => self.csname.to_string(),
            Self::CAPTION_ID => self.caption.clone(),
            Self::FREE_PHYSICAL_MEMORY_ID => self.free_physical_mem.clone(),
            Self::FREE_VIRTUAL_MEMORY_ID => self.free_virtual_mem.clone(),
            Self::PLATFORM_ID => self.platform.clone(),
            Self::VERSION_ID => self.version.to_string(),
            Self::MAJOR_ID => self.major.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::MINOR_ID => self.minor.clone(),
            Self::NAME_ID => self.name.to_string(),
            Self::SERVICE_PACK_MAJOR_ID => self.service_pack_major.clone(),
            Self::SERVICE_PACK_MINOR_ID => self.service_pack_minor.clone(),
            Self::SIZE_STORED_IN_PAGING_FILE_ID => self.size_stored_in_paging_file.clone(),
            Self::TOTAL_VIRTUAL_MEM_SIZE_ID => self.total_virtual_mem_size.to_string(),
            Self::TOTAL_VISIBLE_MEM_SIZE_ID => self.total_visible_mem_size.clone(),
            Self::WIN_DIRECTORY_ID => self.win_directory.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "build_number" => Self::BUILDER_NUMBER_ID,
            "csname" => Self::CSNAME_ID,
            "caption" => Self::CAPTION_ID,
            "free_physical_mem" => Self::FREE_PHYSICAL_MEMORY_ID,
            "free_virtual_mem" => Self::FREE_VIRTUAL_MEMORY_ID,
            "platform" => Self::PLATFORM_ID,
            "version" => Self::VERSION_ID,
            "major" => Self::MAJOR_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "minor" => Self::MINOR_ID,
            "name" => Self::NAME_ID,
            "service_pack_major" => Self::SERVICE_PACK_MAJOR_ID,
            "service_pack_minor" => Self::SERVICE_PACK_MINOR_ID,
            "size_stored_in_paging_file" => Self::SIZE_STORED_IN_PAGING_FILE_ID,
            "total_virtual_mem_size" => Self::TOTAL_VIRTUAL_MEM_SIZE_ID,
            "total_visible_mem_size" => Self::TOTAL_VISIBLE_MEM_SIZE_ID,
            "win_directory" => Self::WIN_DIRECTORY_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OsVersion {
    pub name: String,
    pub platform: String,
    pub version: String,
    pub major: u32,
    pub minor: u32,
}

#[allow(non_upper_case_globals)]
impl OsVersion {
    const NAME_ID: u64 = 0x00000001;
    const PLATFORM_ID: u64 = 0x00000002;
    const VERSION_ID: u64 = 0x00000004;
    const MAJOR_ID: u64 = 0x00000008;
    const MINOR_ID: u64 = 0x00000010;
}

impl Table for OsVersion {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "platform",
        "version",
        "major",
        "minor"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "platform" => self.platform.clone(),
            "version" => self.version.clone(),
            "major" => self.major.to_string(),
            "minor" => self.minor.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::PLATFORM_ID => self.platform.clone(),
            Self::VERSION_ID => self.version.clone(),
            Self::MAJOR_ID => self.major.to_string(),
            Self::MINOR_ID => self.minor.to_string(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "platform" => Self::PLATFORM_ID,
            "version" => Self::VERSION_ID,
            "major" => Self::MAJOR_ID,
            "minor" => Self::MINOR_ID,
            _ => 0
        }
    }
}

#[derive(Debug)]
pub struct LogicalDrive {
    pub device_id: String,
    pub drive_type: String,
    pub free_space: u64,
    pub size: u64,
    pub file_system: String,
}

#[allow(non_upper_case_globals)]
impl LogicalDrive {
    const DEVICE_ID: u64 = 0x00000001;
    const DRIVE_TYPE_ID: u64 = 0x00000002;
    const FREE_SPACE_ID: u64 = 0x00000004;
    const SIZE_ID: u64 = 0x00000008;
    const FILE_SYSTEM_ID: u64 = 0x00000010;
}

impl Table for LogicalDrive {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "device_id",
        "drive_type",
        "free_space",
        "size",
        "file_system"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "device_id" => self.device_id.clone(),
            "drive_type" => self.drive_type.clone(),
            "free_space" => self.free_space.to_string(),
            "size" => self.size.to_string(),
            "file_system" => self.file_system.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::DEVICE_ID => self.device_id.clone(),
            Self::DRIVE_TYPE_ID => self.drive_type.clone(),
            Self::FREE_SPACE_ID => self.free_space.to_string(),
            Self::SIZE_ID => self.size.to_string(),
            Self::FILE_SYSTEM_ID => self.file_system.clone(),

            _ => "".to_string()
        }
    }

    fn get_id(&self, name: &str) -> u64 {
        match name {
            "device_id" => Self::DEVICE_ID,
            "drive_type" => Self::DRIVE_TYPE_ID,
            "free_space" => Self::FREE_SPACE_ID,
            "size" => Self::SIZE_ID,
            "file_system" => Self::FILE_SYSTEM_ID,
            _ => 0
        }
    }
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

#[cfg(not(target_os = "macos"))]
#[derive(Debug)]
pub struct InterfaceAddress {
    pub interface: String,
    pub address: String,
    pub mask: String,
    pub interface_type: String,
    pub friendly_name: String,
}

#[cfg(not(target_os = "macos"))]
#[allow(non_upper_case_globals)]
impl InterfaceAddress {
    const INTERFACE_ID: u64 = 0x00000001;
    const ADDRESS_ID: u64 = 0x00000002;
    const MASK_ID: u64 = 0x00000004;
    const INTERFACE_TYPE_ID: u64 = 0x00000008;
    const FRIENDLY_NAME_ID: u64 = 0x00000010;
}

#[cfg(not(target_os = "macos"))]
impl Table for InterfaceAddress {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "interface",
        "address",
        "mask",
        "interface_type",
        "friendly_name"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "interface" => self.interface.clone(),
            "address" => self.address.clone(),
            "mask" => self.mask.to_string(),
            "interface_type" => self.interface_type.to_string(),
            "friendly_name" => self.friendly_name.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::INTERFACE_ID => self.interface.clone(),
            Self::ADDRESS_ID => self.address.clone(),
            Self::MASK_ID => self.mask.to_string(),
            Self::INTERFACE_TYPE_ID => self.interface_type.to_string(),
            Self::FRIENDLY_NAME_ID => self.friendly_name.clone(),

            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "interface" => Self::INTERFACE_ID,
            "address" => Self::ADDRESS_ID,
            "mask" => Self::MASK_ID,
            "interface_type" => Self::INTERFACE_TYPE_ID,
            "friendly_name" => Self::FRIENDLY_NAME_ID,
            _ => 0
        }
    }
}

#[cfg(not(target_os = "macos"))]
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

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Serialize)]
pub struct InterfaceDetails {
    pub interface: String,
    pub mac: String,
    pub mtu: u32,
    pub enabled: u8,
}

#[cfg(not(target_os = "macos"))]
#[allow(non_upper_case_globals)]
impl InterfaceDetails {
    const INTERFACE_ID: u64 = 0x00000001;
    const MAC_ID: u64 = 0x00000002;
    const MTU_ID: u64 = 0x00000004;
    const ENABLED_ID: u64 = 0x00000008;
}

#[cfg(not(target_os = "macos"))]
impl Table for InterfaceDetails {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "interface",
        "mac",
        "mtu",
        "enabled", ];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "interface" => self.interface.clone(),
            "mac" => self.mac.clone(),
            "mtu" => self.mtu.to_string(),
            "enabled" => self.enabled.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::INTERFACE_ID => self.interface.clone(),
            Self::MAC_ID => self.mac.clone(),
            Self::MTU_ID => self.mtu.to_string(),
            Self::ENABLED_ID => self.enabled.to_string(),
            _ => "".to_string()
        }
    }


    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "interface" => Self::INTERFACE_ID,
            "mac" => Self::MAC_ID,
            "mtu" => Self::MTU_ID,
            "enabled" => Self::ENABLED_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uptime {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total_seconds: f64,
}

#[allow(non_upper_case_globals)]
impl Uptime {
    const DAYS_ID: u64 = 0x00000001;
    const HOURS_ID: u64 = 0x00000002;
    const MINUTES_ID: u64 = 0x00000004;
    const SECONDS_ID: u64 = 0x00000008;
    const TOTAL_SECONDS_ID: u64 = 0x00000010;
}

impl Table for Uptime {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "days",
        "hours",
        "minutes",
        "seconds",
        "total_seconds", ];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "days" => self.days.to_string(),
            "hours" => self.hours.to_string(),
            "minutes" => self.minutes.to_string(),
            "seconds" => self.seconds.to_string(),
            "total_seconds" => self.total_seconds.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::DAYS_ID => self.days.to_string(),
            Self::HOURS_ID => self.hours.to_string(),
            Self::MINUTES_ID => self.minutes.to_string(),
            Self::SECONDS_ID => self.seconds.to_string(),
            Self::TOTAL_SECONDS_ID => self.total_seconds.to_string(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "days" => Self::DAYS_ID,
            "hours" => Self::HOURS_ID,
            "minutes" => Self::MINUTES_ID,
            "seconds" => Self::SECONDS_ID,
            "total_seconds" => Self::TOTAL_SECONDS_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl WmiPrinters {
    const ATTRIBUTES_ID: u64 = 0x00000001;
    const CAPTION_ID: u64 = 0x00000002;
    const CREATION_CLASS_ID: u64 = 0x00000004;
    const DEVICE_ID: u64 = 0x00000008;
    const DO_COMPLETE_FIRST_ID: u64 = 0x00000010;
    const DRIVER_NAME_ID: u64 = 0x00000020;
    const EXTENDED_PRINTER_STATUS_ID: u64 = 0x00000040;
    const HORIZONTAL_RESOLUTION_ID: u64 = 0x00000080;
    const LOCAL_ID: u64 = 0x00000100;
    const NAME_ID: u64 = 0x00000200;
    const PORT_NAME_ID: u64 = 0x00000400;
    const PRINTER_STATUS_ID: u64 = 0x00000800;
    const PRINT_JOB_DATA_TYPE_ID: u64 = 0x00001000;
    const PRINT_PROCESSOR_ID: u64 = 0x00002000;
    const PRIORITY_ID: u64 = 0x00004000;
    const STATUS_ID: u64 = 0x00008000;
    const SYSTEM_CREATION_CLASS_NAME_ID: u64 = 0x00010000;
    const SYSTEM_NAME_ID: u64 = 0x00020000;
    const VERTICAL_RESOLUTION_ID: u64 = 0x00040000;
}

#[cfg(target_os = "windows")]
impl Table for WmiPrinters {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "attributes",
        "caption",
        "creation_class_name",
        "device_id",
        "do_complete_first",
        "driver_name",
        "extended_printer_status",
        "horizontal_resolution",
        "local",
        "name",
        "port_name",
        "printer_status",
        "print_job_data_type",
        "print_processor",
        "priority",
        "status",
        "system_creation_class_name",
        "system_name",
        "vertical_resolution"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "attributes" => self.attributes.clone(),
            "caption" => self.caption.clone(),
            "creation_class_name" => self.creation_class_name.clone(),
            "device_id" => self.device_id.clone(),
            "do_complete_first" => self.do_complete_first.clone(),
            "driver_name" => self.driver_name.clone(),
            "extended_printer_status" => self.extended_printer_status.clone(),
            "horizontal_resolution" => self.horizontal_resolution.clone(),
            "local" => self.local.clone(),
            "name" => self.name.clone(),
            "port_name" => self.port_name.clone(),
            "printer_status" => self.printer_status.clone(),
            "print_job_data_type" => self.print_job_data_type.clone(),
            "print_processor" => self.print_processor.clone(),
            "priority" => self.priority.clone(),
            "status" => self.status.clone(),
            "system_creation_class_name" => self.system_creation_class_name.clone(),
            "system_name" => self.system_name.clone(),
            "vertical_resolution" => self.vertical_resolution.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::ATTRIBUTES_ID => self.attributes.clone(),
            Self::CAPTION_ID => self.caption.clone(),
            Self::CREATION_CLASS_ID => self.creation_class_name.clone(),
            Self::DEVICE_ID => self.device_id.clone(),
            Self::DO_COMPLETE_FIRST_ID => self.do_complete_first.clone(),
            Self::DRIVER_NAME_ID => self.driver_name.clone(),
            Self::EXTENDED_PRINTER_STATUS_ID => self.extended_printer_status.clone(),
            Self::HORIZONTAL_RESOLUTION_ID => self.horizontal_resolution.clone(),
            Self::LOCAL_ID => self.local.clone(),
            Self::NAME_ID => self.name.clone(),
            Self::PORT_NAME_ID => self.port_name.clone(),
            Self::PRINTER_STATUS_ID => self.printer_status.clone(),
            Self::PRINT_JOB_DATA_TYPE_ID => self.print_job_data_type.clone(),
            Self::PRINT_PROCESSOR_ID => self.print_processor.clone(),
            Self::PRIORITY_ID => self.priority.clone(),
            Self::STATUS_ID => self.status.clone(),
            Self::SYSTEM_CREATION_CLASS_NAME_ID => self.system_creation_class_name.clone(),
            Self::SYSTEM_NAME_ID => self.system_name.clone(),
            Self::VERTICAL_RESOLUTION_ID => self.vertical_resolution.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "attributes" => Self::ATTRIBUTES_ID,
            "caption" => Self::CAPTION_ID,
            "creation_class_name" => Self::CREATION_CLASS_ID,
            "device_id" => Self::DEVICE_ID,
            "do_complete_first" => Self::DO_COMPLETE_FIRST_ID,
            "driver_name" => Self::DRIVER_NAME_ID,
            "extended_printer_status" => Self::EXTENDED_PRINTER_STATUS_ID,
            "horizontal_resolution" => Self::HORIZONTAL_RESOLUTION_ID,
            "local" => Self::LOCAL_ID,
            "name" => Self::NAME_ID,
            "port_name" => Self::PORT_NAME_ID,
            "printer_status" => Self::PRINTER_STATUS_ID,
            "print_job_data_type" => Self::PRINT_JOB_DATA_TYPE_ID,
            "print_processor" => Self::PRINT_PROCESSOR_ID,
            "priority" => Self::PRIORITY_ID,
            "status" => Self::STATUS_ID,
            "system_creation_class_name" => Self::SYSTEM_CREATION_CLASS_NAME_ID,
            "system_name" => Self::SYSTEM_NAME_ID,
            "vertical_resolution" => Self::VERTICAL_RESOLUTION_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl WmiServices {
    const ACCEPT_PAUSE_ID: u64 = 0x00000001;
    const ACCEPT_STOP_ID: u64 = 0x00000002;
    const CAPTION_ID: u64 = 0x00000004;
    const CREATION_CLASS_NAME_ID: u64 = 0x00000008;
    const DESCRIPTION_ID: u64 = 0x00000010;
    const DESKTOP_INTERACT_ID: u64 = 0x00000020;
    const DISPLAY_NAME_ID: u64 = 0x00000040;
    const ERROR_CONTROL_ID: u64 = 0x00000080;
    const EXIT_CODE_ID: u64 = 0x00000100;
    const NAME_ID: u64 = 0x00000200;
    const PATH_NAME_ID: u64 = 0x00000400;
    const SERVICE_TYPE_ID: u64 = 0x00000800;
    const STARTED_ID: u64 = 0x00001000;
    const START_MODE_ID: u64 = 0x00002000;
    const START_NAME_ID: u64 = 0x00004000;
    const STATE_ID: u64 = 0x00008000;
    const STATUS_ID: u64 = 0x00010000;
    const SYSTEM_CREATION_CLASS_NAME_ID: u64 = 0x00020000;
    const SYSTEM_NAME_ID: u64 = 0x00040000;
}

#[cfg(target_os = "windows")]
impl Table for WmiServices {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "accept_pause",
        "accept_stop",
        "caption",
        "creation_class_name",
        "description",
        "desktop_interact",
        "display_name",
        "error_control",
        "exit_code",
        "name",
        "path_name",
        "service_type",
        "started",
        "start_mode",
        "start_name",
        "state",
        "status",
        "system_creation_class_name",
        "system_name"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "accept_pause" => self.accept_pause.clone(),
            "accept_stop" => self.accept_stop.clone(),
            "caption" => self.caption.clone(),
            "creation_class_name" => self.creation_class_name.clone(),
            "description" => self.description.clone(),
            "desktop_interact" => self.desktop_interact.clone(),
            "display_name" => self.display_name.clone(),
            "error_control" => self.error_control.clone(),
            "exit_code" => self.exit_code.to_string(),
            "name" => self.name.clone(),
            "path_name" => self.path_name.clone(),
            "service_type" => self.service_type.clone(),
            "started" => self.started.clone(),
            "start_mode" => self.start_mode.clone(),
            "start_name" => self.start_name.clone(),
            "state" => self.state.clone(),
            "status" => self.status.clone(),
            "system_creation_class_name" => self.system_creation_class_name.clone(),
            "system_name" => self.system_name.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::ACCEPT_PAUSE_ID => self.accept_pause.clone(),
            Self::ACCEPT_STOP_ID => self.accept_stop.clone(),
            Self::CAPTION_ID => self.caption.clone(),
            Self::CREATION_CLASS_NAME_ID => self.creation_class_name.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::DESKTOP_INTERACT_ID => self.desktop_interact.clone(),
            Self::DISPLAY_NAME_ID => self.display_name.clone(),
            Self::ERROR_CONTROL_ID => self.error_control.clone(),
            Self::EXIT_CODE_ID => self.exit_code.to_string(),
            Self::NAME_ID => self.name.clone(),
            Self::PATH_NAME_ID => self.path_name.clone(),
            Self::SERVICE_TYPE_ID => self.service_type.clone(),
            Self::STARTED_ID => self.started.clone(),
            Self::START_MODE_ID => self.start_mode.clone(),
            Self::START_NAME_ID => self.start_name.clone(),
            Self::STATE_ID => self.state.clone(),
            Self::STATUS_ID => self.status.clone(),
            Self::SYSTEM_CREATION_CLASS_NAME_ID => self.system_creation_class_name.clone(),
            Self::SYSTEM_NAME_ID => self.system_name.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "accept_pause" => Self::ACCEPT_PAUSE_ID,
            "accept_stop" => Self::ACCEPT_STOP_ID,
            "caption" => Self::CAPTION_ID,
            "creation_class_name" => Self::CREATION_CLASS_NAME_ID,
            "description" => Self::DESCRIPTION_ID,
            "desktop_interact" => Self::DESKTOP_INTERACT_ID,
            "display_name" => Self::DISPLAY_NAME_ID,
            "error_control" => Self::ERROR_CONTROL_ID,
            "exit_code" => Self::EXIT_CODE_ID,
            "name" => Self::NAME_ID,
            "path_name" => Self::PATH_NAME_ID,
            "service_type" => Self::SERVICE_TYPE_ID,
            "started" => Self::STARTED_ID,
            "start_mode" => Self::START_MODE_ID,
            "start_name" => Self::START_NAME_ID,
            "state" => Self::STATE_ID,
            "status" => Self::STATUS_ID,
            "system_creation_class_name" => Self::SYSTEM_CREATION_CLASS_NAME_ID,
            "system_name" => Self::SYSTEM_NAME_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiHotfixes {
    pub caption: String,
    pub csname: String,
    pub description: String,
    pub hotfix_id: String,
    pub installed_by: String,
    pub installed_on: String,
}

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl WmiHotfixes {
    const CAPTION_ID: u64 = 0x00000001;
    const CSNAME_ID: u64 = 0x00000002;
    const DESCRIPTION_ID: u64 = 0x00000004;
    const HOTFIX_ID: u64 = 0x00000008;
    const INSTALLED_BY_ID: u64 = 0x00000010;
    const INSTALLED_ON_ID: u64 = 0x00000020;
}

#[cfg(target_os = "windows")]
#[cfg(target_os = "windows")]
impl Table for WmiHotfixes {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "csname",
        "description",
        "hotfix_id",
        "installed_by",
        "installed_ON"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "caption" => self.caption.clone(),
            "csname" => self.csname.clone(),
            "description" => self.description.clone(),
            "hotfix_id" => self.hotfix_id.clone(),
            "installed_by" => self.installed_by.clone(),
            "installed_on" => self.installed_on.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::CAPTION_ID => self.caption.clone(),
            Self::CSNAME_ID => self.csname.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::HOTFIX_ID => self.hotfix_id.clone(),
            Self::INSTALLED_BY_ID => self.installed_by.clone(),
            Self::INSTALLED_ON_ID => self.installed_on.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "caption" => Self::CAPTION_ID,
            "csname" => Self::CSNAME_ID,
            "description" => Self::DESCRIPTION_ID,
            "hotfix_id" => Self::HOTFIX_ID,
            "installed_by" => Self::INSTALLED_BY_ID,
            "installed_on" => Self::INSTALLED_ON_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    pub install_date: String,
    pub install_location: String,
    pub help_link: String,
    pub name: String,
    pub vendor: String,
    pub version: String,
}

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
impl Products {
    const INSTALL_DATE_ID: u64 = 0x00000001;
    const INSTALL_LOCATION_ID: u64 = 0x00000002;
    const HELP_LINK_ID: u64 = 0x00000004;
    const NAME_ID: u64 = 0x00000008;
    const VENDOR_ID: u64 = 0x00000010;
    const VERSION_ID: u64 = 0x00000020;
}

#[cfg(target_os = "windows")]
impl Table for Products {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "install_date",
        "install_location",
        "help_link",
        "name",
        "vendor",
        "version"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "install_date" => self.install_date.clone(),
            "install_location" => self.install_location.clone(),
            "help_link" => self.help_link.clone(),
            "name" => self.name.clone(),
            "vendor" => self.vendor.clone(),
            "version" => self.version.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::INSTALL_DATE_ID => self.install_date.clone(),
            Self::INSTALL_LOCATION_ID => self.install_location.clone(),
            Self::HELP_LINK_ID => self.help_link.clone(),
            Self::NAME_ID => self.name.clone(),
            Self::VENDOR_ID => self.vendor.clone(),
            Self::VERSION_ID => self.version.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "install_date" => Self::INSTALL_DATE_ID,
            "install_location" => Self::INSTALL_LOCATION_ID,
            "help_link" => Self::HELP_LINK_ID,
            "name" => Self::NAME_ID,
            "vendor" => Self::VENDOR_ID,
            "version" => Self::VERSION_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiNetworkAdapters {
    pub description: String,
    pub database_path: String,
    pub dhcp_enabled: String,
    pub ip_address: Vec<String>,
    pub ip_enabled: String,
    pub ip_subnet: Vec<String>,
    pub mac_address: String,
}

#[cfg(target_os = "windows")]
impl WmiNetworkAdapters {
    const DESCRIPTION_ID: u64 = 0x00000001;
    const DATE_BASE_PATH_ID: u64 = 0x00000002;
    const DHCP_ENABLED_ID: u64 = 0x00000004;
    const IP_ADDRESS_ID: u64 = 0x00000008;
    const IP_SUBNET_ID: u64 = 0x00000010;
    const MAC_ADDRESS_ID: u64 = 0x00000020;
}

#[cfg(target_os = "windows")]
impl Table for WmiNetworkAdapters {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "description",
        "database_path",
        "dhcp_enabled",
        "ip_address",
        "ip_subnet",
        "mac_address"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "description" => self.description.clone(),
            "database_path" => self.database_path.clone(),
            "dhcp_enabled" => self.dhcp_enabled.clone(),
            "ip_address" => {
                let mut ip_address_str: String = "".to_owned();
                for address in self.ip_address.iter() {
                    ip_address_str.push_str(&address);
                    ip_address_str.push_str("\t");
                }
                ip_address_str
            }
            "ip_subnet" => {
                let mut ip_subnet_str: String = "".to_owned();
                for subnet in self.ip_subnet.iter() {
                    ip_subnet_str.push_str(&subnet);
                    ip_subnet_str.push_str("\t");
                }
                ip_subnet_str
            }
            "mac_address" => self.mac_address.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::DATE_BASE_PATH_ID => self.database_path.clone(),
            Self::DHCP_ENABLED_ID => self.dhcp_enabled.clone(),
            Self::IP_ADDRESS_ID => {
                let mut ip_address_str: String = "".to_owned();
                for address in self.ip_address.iter() {
                    ip_address_str.push_str(&address);
                    ip_address_str.push_str("\t");
                }
                ip_address_str
            }
            Self::IP_SUBNET_ID => {
                let mut ip_subnet_str: String = "".to_owned();
                for subnet in self.ip_subnet.iter() {
                    ip_subnet_str.push_str(&subnet);
                    ip_subnet_str.push_str("\t");
                }
                ip_subnet_str
            }
            Self::MAC_ADDRESS_ID => self.mac_address.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "description" => Self::DESCRIPTION_ID,
            "database_path" => Self::DATE_BASE_PATH_ID,
            "dhcp_enabled" => Self::DHCP_ENABLED_ID,
            "ip_address" => Self::IP_ADDRESS_ID,
            "ip_subnet" => Self::IP_SUBNET_ID,
            "mac_address" => Self::MAC_ADDRESS_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
impl WmiShares {
    const CAPTION_ID: u64 = 0x00000001;
    const DESCRIPTION_ID: u64 = 0x00000002;
    const NAME_ID: u64 = 0x00000004;
    const PATH_ID: u64 = 0x00000008;
    const STATUS_ID: u64 = 0x00000010;
    const TYPE_ID: u64 = 0x00000020;
    const ALLOW_MAXIMUM_ID: u64 = 0x00000040;
}

#[cfg(target_os = "windows")]
impl Table for WmiShares {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "description",
        "name",
        "path",
        "status",
        "type",
        "allow_maximum"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "caption" => self.caption.clone(),
            "description" => self.description.clone(),
            "name" => self.name.clone(),
            "path" => self.path.clone(),
            "status" => self.status.clone(),
            "type" => self._type.clone(),
            "allow_maximum" => self.allow_maximum.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::CAPTION_ID => self.caption.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::NAME_ID => self.name.clone(),
            Self::PATH_ID => self.path.clone(),
            Self::STATUS_ID => self.status.clone(),
            Self::TYPE_ID => self._type.clone(),
            Self::ALLOW_MAXIMUM_ID => self.allow_maximum.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "caption" => Self::CAPTION_ID,
            "description" => Self::DESCRIPTION_ID,
            "name" => Self::NAME_ID,
            "path" => Self::PATH_ID,
            "status" => Self::STATUS_ID,
            "type" => Self::TYPE_ID,
            "allow_maximum" => Self::ALLOW_MAXIMUM_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
impl WmiLocalAccounts {
    const ACCOUNT_TYPE_ID: u64 = 0x00000001;
    const CAPTION_ID: u64 = 0x00000002;
    const DESCRIPTION_ID: u64 = 0x00000004;
    const DOMAIN_ID: u64 = 0x00000008;
    const LOCAL_ACCOUNT_ID: u64 = 0x00000010;
    const NAME_ID: u64 = 0x00000020;
    const SID_ID: u64 = 0x00000040;
    const SID_TYPE_ID: u64 = 0x00000080;
    const STATUS_ID: u64 = 0x00000100;
}

#[cfg(target_os = "windows")]
impl Table for WmiLocalAccounts {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "account_type",
        "caption",
        "description",
        "domain",
        "local_account",
        "name",
        "sid",
        "sid_type",
        "status"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "account_type" => self.account_type.clone(),
            "caption" => self.caption.clone(),
            "description" => self.description.clone(),
            "domain" => self._domain.clone(),
            "local_account" => self.local_account.clone(),
            "name" => self.name.clone(),
            "sid" => self.sid.clone(),
            "sid_type" => self.sid_type.clone(),
            "status" => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::ACCOUNT_TYPE_ID => self.account_type.clone(),
            Self::CAPTION_ID => self.caption.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::DOMAIN_ID => self._domain.clone(),
            Self::LOCAL_ACCOUNT_ID => self.local_account.clone(),
            Self::NAME_ID => self.name.clone(),
            Self::SID_ID => self.sid.clone(),
            Self::SID_TYPE_ID => self.sid_type.clone(),
            Self::STATUS_ID => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "account_type" => Self::ACCOUNT_TYPE_ID,
            "caption" => Self::CAPTION_ID,
            "description" => Self::DESCRIPTION_ID,
            "domain" => Self::DOMAIN_ID,
            "local_account" => Self::LOCAL_ACCOUNT_ID,
            "name" => Self::NAME_ID,
            "sid" => Self::SID_ID,
            "sid_type" => Self::SID_TYPE_ID,
            "status" => Self::STATUS_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiBios {
    pub caption: String,
    pub manufacturer: String,
    pub release_date: String,
    pub serial_number: String,
    pub smbios_version: String,
}

#[cfg(target_os = "windows")]
impl WmiBios {
    const CAPTION_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const RELEASE_DATE_ID: u64 = 0x00000004;
    const SERIAL_NUMBER_ID: u64 = 0x00000008;
    const SMBIOS_VERSION_ID: u64 = 0x00000010;
}

#[cfg(target_os = "windows")]
impl Table for WmiBios {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "manufacturer",
        "release_date",
        "serial_number",
        "smbios_version"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "caption" => self.caption.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "release_date" => self.release_date.clone(),
            "serial_number" => self.serial_number.clone(),
            "smbios_version" => self.smbios_version.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::CAPTION_ID => self.caption.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::RELEASE_DATE_ID => self.release_date.clone(),
            Self::SERIAL_NUMBER_ID => self.serial_number.clone(),
            Self::SMBIOS_VERSION_ID => self.smbios_version.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "caption" => Self::CAPTION_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "release_date" => Self::RELEASE_DATE_ID,
            "serial_number" => Self::SERIAL_NUMBER_ID,
            "smbios_version" => Self::SMBIOS_VERSION_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMotherboard {
    pub name: String,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
    pub version: String,
}

#[cfg(target_os = "windows")]
impl WmiMotherboard {
    const NAME_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const PRODUCT_ID: u64 = 0x00000004;
    const SERIAL_NUMBER_ID: u64 = 0x00000008;
    const VERSION_ID: u64 = 0x00000010;
}

#[cfg(target_os = "windows")]
impl Table for WmiMotherboard {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "manufacturer",
        "product",
        "serial_number",
        "version"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "product" => self.product.clone(),
            "serial_number" => self.serial_number.clone(),
            "version" => self.version.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::PRODUCT_ID => self.product.clone(),
            Self::SERIAL_NUMBER_ID => self.serial_number.clone(),
            Self::VERSION_ID => self.version.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "product" => Self::PRODUCT_ID,
            "serial_number" => Self::SERIAL_NUMBER_ID,
            "version" => Self::VERSION_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiProcessor {
    pub address_width: String,
    pub cpu_satus: String,
    pub current_clock_speed: String,
    pub current_voltage: String,
    pub description: String,
    pub external_clock: String,
    pub hyper_threading_enabled: String,
    pub l2_cache_size: String,
    pub l2_cache_speed: String,
    pub l3_cache_size: String,
    pub l3_cache_speed: String,
    pub manufacturer: String,
    pub max_clock_speed: String,
    pub name: String,
    pub number_of_cores: String,
    pub number_of_logical_processors: String,
    pub socket_designation: String,
}

#[cfg(target_os = "windows")]
impl WmiProcessor {
    const ADDRESS_WIDTH_ID: u64 = 0x00000001;
    const CPU_STATUS_ID: u64 = 0x00000002;
    const CURRENT_CLOCK_SPEED_ID: u64 = 0x00000004;
    const CURRENT_VOLTAGE_ID: u64 = 0x00000008;
    const DESCRIPTION_ID: u64 = 0x00000010;
    const EXTERNAL_CLOCK_ID: u64 = 0x00000020;
    const HYPER_THREADING_ENABLED_ID: u64 = 0x00000040;
    const L2_CACHE_SIZE_ID: u64 = 0x00000080;
    const L2_CACHE_SPEED_ID: u64 = 0x00000100;
    const L3_CACHE_SIZE_ID: u64 = 0x00000200;
    const L3_CACHE_SPEED_ID: u64 = 0x00000400;
    const MANUFACTURER_ID: u64 = 0x00000800;
    const MAX_CLOCK_SPEED_ID: u64 = 0x00001000;
    const NAME_ID: u64 = 0x00002000;
    const NUMBER_OF_CORES_ID: u64 = 0x00004000;
    const NUMBER_OF_LOGICAL_PROCESSORS_ID: u64 = 0x00008000;
    const SOCKET_DESIGNATION_ID: u64 = 0x00010000;
}

#[cfg(target_os = "windows")]
impl Table for WmiProcessor {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "address_width",
        "cpu_satus",
        "current_clock_speed",
        "current_voltage",
        "description",
        "external_clock",
        "hyper_threading_enabled",
        "l2_cache_size",
        "l2_cache_speed",
        "l3_cache_size",
        "l3_cache_speed",
        "manufacturer",
        "max_clock_speed",
        "name",
        "number_of_cores",
        "number_of_logical_processors",
        "socket_designation", ];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "address_width" => self.address_width.clone(),
            "cpu_satus" => self.cpu_satus.clone(),
            "current_clock_speed" => self.current_clock_speed.clone(),
            "current_voltage" => self.current_voltage.clone(),
            "description" => self.description.clone(),
            "external_clock" => self.external_clock.clone(),
            "hyper_threading_enabled" => self.hyper_threading_enabled.clone(),
            "l2_cache_size" => self.l2_cache_size.clone(),
            "l2_cache_speed" => self.l2_cache_speed.clone(),
            "l3_cache_size" => self.l3_cache_size.clone(),
            "l3_cache_speed" => self.l3_cache_speed.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "max_clock_speed" => self.max_clock_speed.clone(),
            "name" => self.name.clone(),
            "number_of_cores" => self.number_of_cores.clone(),
            "number_of_logical_processors" => self.number_of_logical_processors.clone(),
            "socket_designation" => self.socket_designation.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::ADDRESS_WIDTH_ID => self.address_width.clone(),
            Self::CPU_STATUS_ID => self.cpu_satus.clone(),
            Self::CURRENT_CLOCK_SPEED_ID => self.current_clock_speed.clone(),
            Self::CURRENT_VOLTAGE_ID => self.current_voltage.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::EXTERNAL_CLOCK_ID => self.external_clock.clone(),
            Self::HYPER_THREADING_ENABLED_ID => self.hyper_threading_enabled.clone(),
            Self::L2_CACHE_SIZE_ID => self.l2_cache_size.clone(),
            Self::L2_CACHE_SPEED_ID => self.l2_cache_speed.clone(),
            Self::L3_CACHE_SIZE_ID => self.l3_cache_size.clone(),
            Self::L3_CACHE_SPEED_ID => self.l3_cache_speed.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::MAX_CLOCK_SPEED_ID => self.max_clock_speed.clone(),
            Self::NAME_ID => self.name.clone(),
            Self::NUMBER_OF_CORES_ID => self.number_of_cores.clone(),
            Self::NUMBER_OF_LOGICAL_PROCESSORS_ID => self.number_of_logical_processors.clone(),
            Self::SOCKET_DESIGNATION_ID => self.socket_designation.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "address_width" => Self::ADDRESS_WIDTH_ID,
            "cpu_satus" => Self::CPU_STATUS_ID,
            "current_clock_speed" => Self::CURRENT_CLOCK_SPEED_ID,
            "current_voltage" => Self::CURRENT_VOLTAGE_ID,
            "description" => Self::DESCRIPTION_ID,
            "external_clock" => Self::EXTERNAL_CLOCK_ID,
            "hyper_threading_enabled" => Self::HYPER_THREADING_ENABLED_ID,
            "l2_cache_size" => Self::L2_CACHE_SIZE_ID,
            "l2_cache_speed" => Self::L2_CACHE_SPEED_ID,
            "l3_cache_size" => Self::L3_CACHE_SIZE_ID,
            "l3_cache_speed" => Self::L3_CACHE_SPEED_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "max_clock_speed" => Self::MAX_CLOCK_SPEED_ID,
            "name" => Self::NAME_ID,
            "number_of_cores" => Self::NUMBER_OF_CORES_ID,
            "number_of_logical_processors" => Self::NUMBER_OF_LOGICAL_PROCESSORS_ID,
            "socket_designation" => Self::SOCKET_DESIGNATION_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMemory {
    pub name: String,
    pub bank_label: String,
    pub capacity: String,
    pub description: String,
    pub device_locator: String,
    pub form_factor: String,
    pub interleave_data_depth: String,
    pub interleave_position: String,
    pub manufacturer: String,
    pub memory_type: String,
    pub serial_number: String,
    pub speed: String,
}

#[cfg(target_os = "windows")]
impl WmiMemory {
    const NAME_ID: u64 = 0x00000001;
    const BANK_LABEL_ID: u64 = 0x00000002;
    const CAPACITY_ID: u64 = 0x00000004;
    const DESCRIPTION_ID: u64 = 0x00000008;
    const DEVICE_LOCATOR_ID: u64 = 0x00000010;
    const FORM_FACTOR_ID: u64 = 0x00000020;
    const INTERLEAVE_DATA_DEPTH_ID: u64 = 0x00000040;
    const INTERLEAVE_POSITION_ID: u64 = 0x00000080;
    const MANUFACTURER_ID: u64 = 0x00000100;
    const MEMORY_TYPE_ID: u64 = 0x00000200;
    const SERIAL_NUMBER_ID: u64 = 0x00000400;
    const SPEED_ID: u64 = 0x00000800;
}

#[cfg(target_os = "windows")]
impl Table for WmiMemory {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "bank_label",
        "capacity",
        "description",
        "device_locator",
        "form_factor",
        "interleave_data_depth",
        "interleave_position",
        "manufacturer",
        "memory_type",
        "serial_number",
        "speed", ];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "bank_label" => self.bank_label.clone(),
            "capacity" => self.capacity.clone(),
            "description" => self.description.clone(),
            "device_locator" => self.device_locator.clone(),
            "form_factor" => self.form_factor.clone(),
            "interleave_data_depth" => self.interleave_data_depth.clone(),
            "interleave_position" => self.interleave_position.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "memory_type" => self.memory_type.clone(),
            "serial_number" => self.serial_number.clone(),
            "speed" => self.speed.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::BANK_LABEL_ID => self.bank_label.clone(),
            Self::CAPACITY_ID => self.capacity.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::DEVICE_LOCATOR_ID => self.device_locator.clone(),
            Self::FORM_FACTOR_ID => self.form_factor.clone(),
            Self::INTERLEAVE_DATA_DEPTH_ID => self.interleave_data_depth.clone(),
            Self::INTERLEAVE_POSITION_ID => self.interleave_position.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::MEMORY_TYPE_ID => self.memory_type.clone(),
            Self::SERIAL_NUMBER_ID => self.serial_number.clone(),
            Self::SPEED_ID => self.speed.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "bank_label" => Self::BANK_LABEL_ID,
            "capacity" => Self::CAPACITY_ID,
            "description" => Self::DESCRIPTION_ID,
            "device_locator" => Self::DEVICE_LOCATOR_ID,
            "form_factor" => Self::FORM_FACTOR_ID,
            "interleave_data_depth" => Self::INTERLEAVE_DATA_DEPTH_ID,
            "interleave_position" => Self::INTERLEAVE_POSITION_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "memory_type" => Self::MEMORY_TYPE_ID,
            "serial_number" => Self::SERIAL_NUMBER_ID,
            "speed" => Self::SPEED_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiSound {
    pub name: String,
    pub status: String,
    pub manufacturer: String,
    pub dma_buffer_size: String,
}

#[cfg(target_os = "windows")]
impl WmiSound {
    const NAME_ID: u64 = 0x00000001;
    const STATUS_ID: u64 = 0x00000002;
    const MANUFACTURER_ID: u64 = 0x00000004;
    const DMA_BUFFER_SIZE_ID: u64 = 0x00000008;
}

#[cfg(target_os = "windows")]
impl Table for WmiSound {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "status",
        "manufacturer",
        "dma_buffer_size"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "status" => self.status.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "dma_buffer_size" => self.dma_buffer_size.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::STATUS_ID => self.status.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::DMA_BUFFER_SIZE_ID => self.dma_buffer_size.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "status" => Self::STATUS_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "dma_buffer_size" => Self::DMA_BUFFER_SIZE_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiVideo {
    pub name: String,
    pub adapter_compatibility: String,
    pub adapter_dac_type: String,
    pub adapter_ram: f32,
    pub availability: String,
    pub driver_version: String,
    pub installed_display_driver: Vec<String>,
    pub refresh_rate: String,
    pub screen_info: String,
    pub status: String,
    pub video_architecture: String,
    pub video_memory_type: String,
}

#[cfg(target_os = "windows")]
impl WmiVideo {
    const NAME_ID: u64 = 0x00000001;
    const ADAPTER_COMPATIBILITY_ID: u64 = 0x00000002;
    const ADAPTER_DAC_TYPE_ID: u64 = 0x00000004;
    const ADAPTER_RAM_ID: u64 = 0x00000008;
    const AVAILABILITY_ID: u64 = 0x00000010;
    const DRIVER_VERSION_ID: u64 = 0x00000020;
    const INSTALLED_DISPLAY_DRIVER_ID: u64 = 0x00000040;
    const REFRESH_RATE_ID: u64 = 0x00000080;
    const SCREEN_INFO_ID: u64 = 0x00000100;
    const STATUS_ID: u64 = 0x00000200;
    const VIDEO_ARCHITECTURE_ID: u64 = 0x00000400;
    const VIDEO_MEMORY_TYPE_ID: u64 = 0x00000800;
}

#[cfg(target_os = "windows")]
impl Table for WmiVideo {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "adapter_compatibility",
        "adapter_dac_type",
        "adapter_ram",
        "availability",
        "driver_version",
        "installed_display_driver",
        "refresh_rate",
        "screen_info",
        "status",
        "video_architecture",
        "video_memory_type", ];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "adapter_compatibility" => self.adapter_compatibility.clone(),
            "adapter_dac_type" => self.adapter_dac_type.clone(),
            "adapter_ram" => self.adapter_ram.to_string(),
            "availability" => self.availability.clone(),
            "driver_version" => self.driver_version.clone(),
            "installed_display_driver" => {
                let mut installed_display_driver: String = "".to_owned();
                for driver in self.installed_display_driver.iter() {
                    installed_display_driver.push_str(&driver);
                    installed_display_driver.push_str("\t");
                }
                installed_display_driver
            }
            "refresh_rate" => self.refresh_rate.clone(),
            "screen_info" => self.screen_info.clone(),
            "status" => self.status.clone(),
            "video_architecture" => self.video_architecture.clone(),
            "video_memory_type" => self.video_memory_type.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::ADAPTER_COMPATIBILITY_ID => self.adapter_compatibility.clone(),
            Self::ADAPTER_DAC_TYPE_ID => self.adapter_dac_type.clone(),
            Self::ADAPTER_RAM_ID => self.adapter_ram.to_string(),
            Self::AVAILABILITY_ID => self.availability.clone(),
            Self::DRIVER_VERSION_ID => self.driver_version.clone(),
            Self::INSTALLED_DISPLAY_DRIVER_ID => {
                let mut installed_display_driver: String = "".to_owned();
                for driver in self.installed_display_driver.iter() {
                    installed_display_driver.push_str(&driver);
                    installed_display_driver.push_str("\t");
                }
                installed_display_driver
            }
            Self::REFRESH_RATE_ID => self.refresh_rate.clone(),
            Self::SCREEN_INFO_ID => self.screen_info.clone(),
            Self::STATUS_ID => self.status.clone(),
            Self::VIDEO_ARCHITECTURE_ID => self.video_architecture.clone(),
            Self::VIDEO_MEMORY_TYPE_ID => self.video_memory_type.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "adapter_compatibility" => Self::ADAPTER_COMPATIBILITY_ID,
            "adapter_dac_type" => Self::ADAPTER_DAC_TYPE_ID,
            "adapter_ram" => Self::ADAPTER_RAM_ID,
            "availability" => Self::AVAILABILITY_ID,
            "driver_version" => Self::DRIVER_VERSION_ID,
            "installed_display_driver" => Self::INSTALLED_DISPLAY_DRIVER_ID,
            "refresh_rate" => Self::REFRESH_RATE_ID,
            "screen_info" => Self::SCREEN_INFO_ID,
            "status" => Self::STATUS_ID,
            "video_architecture" => Self::VIDEO_ARCHITECTURE_ID,
            "video_memory_type" => Self::VIDEO_MEMORY_TYPE_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMonitors {
    pub name: String,
    pub availability: String,
    pub bandwidth: u64,
    pub manufacturer: String,
    pub screen_height: u64,
    pub screen_width: u64,
}

#[cfg(target_os = "windows")]
impl WmiMonitors {
    const NAME_ID: u64 = 0x00000001;
    const AVAILABILITY_ID: u64 = 0x00000002;
    const BANDWIDTH_ID: u64 = 0x00000004;
    const MANUFACTURER_ID: u64 = 0x00000008;
    const SCREEN_HEIGHT_ID: u64 = 0x00000010;
    const SCREEN_WIDTH_ID: u64 = 0x00000020;
}

#[cfg(target_os = "windows")]
impl Table for WmiMonitors {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "availability",
        "bandwidth",
        "manufacturer",
        "screen_height",
        "screen_width"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "availability" => self.availability.clone(),
            "bandwidth" => self.bandwidth.to_string(),
            "manufacturer" => self.manufacturer.clone(),
            "screen_height" => self.screen_height.to_string(),
            "screen_width" => self.screen_width.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::AVAILABILITY_ID => self.availability.clone(),
            Self::BANDWIDTH_ID => self.bandwidth.to_string(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::SCREEN_HEIGHT_ID => self.screen_height.to_string(),
            Self::SCREEN_WIDTH_ID => self.screen_width.to_string(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "availability" => Self::AVAILABILITY_ID,
            "bandwidth" => Self::BANDWIDTH_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "screen_height" => Self::SCREEN_HEIGHT_ID,
            "screen_width" => Self::SCREEN_WIDTH_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiKeyboard {
    pub name: String,
    pub description: String,
    pub device_id: String,
    pub status: String,
}

#[cfg(target_os = "windows")]
impl WmiKeyboard {
    const NAME_ID: u64 = 0x00000001;
    const DESCRIPTION_ID: u64 = 0x00000002;
    const DEVICE_ID: u64 = 0x00000004;
    const STATUS_ID: u64 = 0x00000008;
}

#[cfg(target_os = "windows")]
impl Table for WmiKeyboard {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "description",
        "device_id",
        "status"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "description" => self.description.clone(),
            "device_id" => self.device_id.to_string(),
            "status" => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::DESCRIPTION_ID => self.description.clone(),
            Self::DEVICE_ID => self.device_id.to_string(),
            Self::STATUS_ID => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "description" => Self::DESCRIPTION_ID,
            "device_id" => Self::DEVICE_ID,
            "status" => Self::STATUS_ID,
            _ => 0
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiPointingDevice {
    pub name: String,
    pub manufacturer: String,
    pub description: String,
    pub pointing_type: String,
    pub status: String,
}

#[cfg(target_os = "windows")]
impl WmiPointingDevice {
    const NAME_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const DESCRIPTION_ID: u64 = 0x00000004;
    const POINTING_TYPE_ID: u64 = 0x00000008;
    const STATUS_ID: u64 = 0x00000010;
}

#[cfg(target_os = "windows")]
impl Table for WmiPointingDevice {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "manufacturer",
        "description",
        "pointing_type",
        "status"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "name" => self.name.clone(),
            "manufacturer" => self.manufacturer.clone(),
            "description" => self.description.to_string(),
            "pointing_type" => self.pointing_type.clone(),
            "status" => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::NAME_ID => self.name.clone(),
            Self::MANUFACTURER_ID => self.manufacturer.clone(),
            Self::DESCRIPTION_ID => self.description.to_string(),
            Self::POINTING_TYPE_ID => self.pointing_type.clone(),
            Self::STATUS_ID => self.status.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "name" => Self::NAME_ID,
            "manufacturer" => Self::MANUFACTURER_ID,
            "description" => Self::DESCRIPTION_ID,
            "pointing_type" => Self::POINTING_TYPE_ID,
            "status" => Self::STATUS_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessOpenSocketsRow {
    pub pid: i64,
    pub fd: i64,
    pub socket: i64,
    pub family: i32,
    pub protocol: i32,
    pub local_address: String,
    pub remote_address: String,
    pub local_port: i32,
    pub remote_port: i32,
    pub path: String,
    pub state: String,
    pub net_namespace: String,
}

impl ProcessOpenSocketsRow {
    const PID_ID: u64 = 0x00000001;
    const FD_ID: u64 = 0x00000002;
    const SOCKET_ID: u64 = 0x00000004;
    const FAMILY_ID: u64 = 0x00000008;
    const PROTOCOL_ID: u64 = 0x00000010;
    const LOCAL_ADDRESS_ID: u64 = 0x00000020;
    const REMOTE_ADDRESS_ID: u64 = 0x00000040;
    const LOCAL_PORT_ID: u64 = 0x00000080;
    const REMOTE_PORT_ID: u64 = 0x00000100;
    const PATH_ID: u64 = 0x00000200;
    const STATE_ID: u64 = 0x00000400;
    const NET_NAMESPACE_ID: u64 = 0x00000800;
}

impl Table for ProcessOpenSocketsRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "pid",
        "fd",
        "socket",
        "family",
        "protocol",
        "local_address",
        "remote_address",
        "local_port",
        "remote_port",
        "path",
        "state",
        "net_namespace"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "pid" => self.pid.to_string(),
            "fd" => self.fd.to_string(),
            "socket" => self.socket.to_string(),
            "family" => self.family.to_string(),
            "protocol" => self.protocol.to_string(),
            "local_address" => self.local_address.clone(),
            "remote_address" => self.remote_address.clone(),
            "local_port" => self.local_port.to_string(),
            "remote_port" => self.remote_port.to_string(),
            "path" => self.path.clone(),
            "state" => self.state.clone(),
            "net_namespace" => self.net_namespace.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::PID_ID => self.pid.to_string(),
            Self::FD_ID => self.fd.to_string(),
            Self::SOCKET_ID => self.socket.to_string(),
            Self::FAMILY_ID => self.family.to_string(),
            Self::PROTOCOL_ID => self.protocol.to_string(),
            Self::LOCAL_ADDRESS_ID => self.local_address.clone(),
            Self::REMOTE_ADDRESS_ID => self.remote_address.clone(),
            Self::LOCAL_PORT_ID => self.local_port.to_string(),
            Self::REMOTE_PORT_ID => self.remote_port.to_string(),
            Self::PATH_ID => self.path.clone(),
            Self::STATE_ID => self.state.clone(),
            Self::NET_NAMESPACE_ID => self.net_namespace.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "pid" => Self::PID_ID,
            "fd" => Self::FD_ID,
            "socket" => Self::SOCKET_ID,
            "family" => Self::FAMILY_ID,
            "protocol" => Self::PROTOCOL_ID,
            "local_address" => Self::LOCAL_ADDRESS_ID,
            "remote_address" => Self::REMOTE_ADDRESS_ID,
            "local_port" => Self::LOCAL_PORT_ID,
            "remote_port" => Self::REMOTE_PORT_ID,
            "path" => Self::PATH_ID,
            "state" => Self::STATE_ID,
            "net_namespace" => Self::NET_NAMESPACE_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessesRow {
    pub pid: i64,
    pub name: String,
    pub path: String,
    pub cmdline: String,
    pub state: String,
    pub cwd: String,
    pub root: String,
    pub uid: i64,
    pub gid: i64,
    pub euid: i64,
    pub egid: i64,
    pub suid: i64,
    pub sgid: i64,
    pub on_disk: i32,
    pub wired_size: i64,
    pub resident_size: i64,
    pub total_size: i64,
    pub user_time: i64,
    pub system_time: i64,
    pub disk_bytes_read: i64,
    pub disk_bytes_written: i64,
    pub start_time: i64,
    pub parent: i64,
    pub pgroup: i64,
    pub threads: i32,
    pub nice: i32,
    pub is_elevated_token: i32,
    pub cgroup_namespace: String,
    pub ipc_namespace: String,
    pub mnt_namespace: String,
    pub net_namespace: String,
    pub pid_namespace: String,
    pub user_namespace: String,
    pub uts_namespace: String,
}

impl ProcessesRow {
    const PID_ID: u64 = 0x00000001;
    const NAME_ID: u64 = 0x00000002;
    const PATH_ID: u64 = 0x00000004;
    const CMDLINE_ID: u64 = 0x00000008;
    const STATE_ID: u64 = 0x00000010;
    const CWD_ID: u64 = 0x00000020;
    const ROOT_ID: u64 = 0x00000040;
    const UID_ID: u64 = 0x00000080;
    const GID_ID: u64 = 0x00000100;
    const EUID_ID: u64 = 0x00000200;
    const EGID_ID: u64 = 0x00000400;
    const SUID_ID: u64 = 0x00000800;
    const SGID_ID: u64 = 0x00001000;
    const ON_DISK_ID: u64 = 0x00002000;
    const WIRED_SIZE_ID: u64 = 0x00004000;
    const RESIDENT_SIZE_ID: u64 = 0x00008000;
    const TOTAL_SIZE_ID: u64 = 0x00010080;
    const USER_TIME_ID: u64 = 0x00020000;
    const SYSTEM_TIME_ID: u64 = 0x00040000;
    const DISK_BYTES_READ_ID: u64 = 0x00080000;
    const DISK_BYTES_WRITTEN_ID: u64 = 0x00100000;
    const START_TIME_ID: u64 = 0x00200008;
    const PARENT_ID: u64 = 0x00400000;
    const PGROUP_ID: u64 = 0x00800000;
    const THREADS_ID: u64 = 0x01000000;
    const NICE_ID: u64 = 0x02000000;
    const IS_ELEVATED_TOKEN_ID: u64 = 0x04000000;
    const CGROUPE_NAMESPACE_ID: u64 = 0x08000000;
    const IPC_NAMESPACE_ID: u64 = 0x10000000;
    const MNT_NAMESPACE_ID: u64 = 0x20000000;
    const NET_NAMESPACE_ID: u64 = 0x40000000;
    const PID_NAMESPACE_ID: u64 = 0x80000000;
    const USER_NAMESPACE_ID: u64 = 0x100000000;
    const UTS_NAMESPACE_ID: u64 = 0x200000000;
}

impl Table for ProcessesRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "pid",
        "name",
        "path",
        "cmdline",
        "state",
        "cwd",
        "root",
        "uid",
        "gid",
        "euid",
        "egid",
        "suid",
        "sgid",
        "on_disk",
        "wired_size",
        "resident_size",
        "total_size",
        "user_time",
        "system_time",
        "disk_bytes_read",
        "disk_bytes_written",
        "start_time",
        "parent",
        "pgroup",
        "threads",
        "nice",
        "is_elevated_token",
        "cgroup_namespace",
        "ipc_namespace",
        "mnt_namespace",
        "net_namespace",
        "pid_namespace",
        "user_namespace",
        "uts_namespace"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "pid" => self.pid.to_string(),
            "name" => self.name.clone(),
            "path" => self.path.clone(),
            "cmdline" => self.cmdline.clone(),
            "state" => self.state.clone(),
            "cwd" => self.cwd.clone(),
            "root" => self.root.clone(),
            "uid" => self.uid.to_string(),
            "gid" => self.gid.to_string(),
            "euid" => self.euid.to_string(),
            "egid" => self.egid.to_string(),
            "suid" => self.suid.to_string(),
            "sgid" => self.sgid.to_string(),
            "on_disk" => self.on_disk.to_string(),
            "wired_size" => self.wired_size.to_string(),
            "resident_size" => self.resident_size.to_string(),
            "total_size" => self.total_size.to_string(),
            "user_time" => self.user_time.to_string(),
            "system_time" => self.system_time.to_string(),
            "disk_bytes_read" => self.disk_bytes_read.to_string(),
            "disk_bytes_written" => self.disk_bytes_written.to_string(),
            "start_time" => self.start_time.to_string(),
            "parent" => self.parent.to_string(),
            "pgroup" => self.pgroup.to_string(),
            "threads" => self.threads.to_string(),
            "nice" => self.nice.to_string(),
            "is_elevated_token" => self.is_elevated_token.to_string(),
            "cgroup_namespace" => self.cgroup_namespace.clone(),
            "ipc_namespace" => self.ipc_namespace.clone(),
            "mnt_namespace" => self.mnt_namespace.clone(),
            "net_namespace" => self.net_namespace.clone(),
            "pid_namespace" => self.pid_namespace.clone(),
            "user_namespace" => self.user_namespace.clone(),
            "uts_namespace" => self.uts_namespace.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::PID_ID => self.pid.to_string(),
            Self::NAME_ID => self.name.clone(),
            Self::PATH_ID => self.path.clone(),
            Self::CMDLINE_ID => self.cmdline.clone(),
            Self::STATE_ID => self.state.clone(),
            Self::CWD_ID => self.cwd.clone(),
            Self::ROOT_ID => self.root.clone(),
            Self::UID_ID => self.uid.to_string(),
            Self::GID_ID => self.gid.to_string(),
            Self::EUID_ID => self.euid.to_string(),
            Self::EGID_ID => self.egid.to_string(),
            Self::SUID_ID => self.suid.to_string(),
            Self::SGID_ID => self.sgid.to_string(),
            Self::ON_DISK_ID => self.on_disk.to_string(),
            Self::WIRED_SIZE_ID => self.wired_size.to_string(),
            Self::RESIDENT_SIZE_ID => self.resident_size.to_string(),
            Self::TOTAL_SIZE_ID => self.total_size.to_string(),
            Self::USER_TIME_ID => self.user_time.to_string(),
            Self::SYSTEM_TIME_ID => self.system_time.to_string(),
            Self::DISK_BYTES_READ_ID => self.disk_bytes_read.to_string(),
            Self::DISK_BYTES_WRITTEN_ID => self.disk_bytes_written.to_string(),
            Self::START_TIME_ID => self.start_time.to_string(),
            Self::PARENT_ID => self.parent.to_string(),
            Self::PGROUP_ID => self.pgroup.to_string(),
            Self::THREADS_ID => self.threads.to_string(),
            Self::NICE_ID => self.nice.to_string(),
            Self::IS_ELEVATED_TOKEN_ID => self.is_elevated_token.to_string(),
            Self::CGROUPE_NAMESPACE_ID => self.cgroup_namespace.clone(),
            Self::IPC_NAMESPACE_ID => self.ipc_namespace.clone(),
            Self::MNT_NAMESPACE_ID => self.mnt_namespace.clone(),
            Self::NET_NAMESPACE_ID => self.net_namespace.clone(),
            Self::PID_NAMESPACE_ID => self.pid_namespace.clone(),
            Self::USER_NAMESPACE_ID => self.user_namespace.clone(),
            Self::UTS_NAMESPACE_ID => self.uts_namespace.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "pid" => Self::PID_ID,
            "name" => Self::NAME_ID,
            "path" => Self::PATH_ID,
            "cmdline" => Self::CMDLINE_ID,
            "state" => Self::STATE_ID,
            "cwd" => Self::CWD_ID,
            "root" => Self::ROOT_ID,
            "uid" => Self::UID_ID,
            "gid" => Self::GID_ID,
            "euid" => Self::EUID_ID,
            "egid" => Self::EGID_ID,
            "suid" => Self::SUID_ID,
            "sgid" => Self::SGID_ID,
            "on_disk" => Self::ON_DISK_ID,
            "wired_size" => Self::WIRED_SIZE_ID,
            "resident_size" => Self::RESIDENT_SIZE_ID,
            "total_size" => Self::TOTAL_SIZE_ID,
            "user_time" => Self::USER_TIME_ID,
            "system_time" => Self::SYSTEM_TIME_ID,
            "disk_bytes_read" => Self::DISK_BYTES_READ_ID,
            "disk_bytes_written" => Self::DISK_BYTES_WRITTEN_ID,
            "start_time" => Self::START_TIME_ID,
            "parent" => Self::PARENT_ID,
            "pgroup" => Self::PGROUP_ID,
            "threads" => Self::THREADS_ID,
            "nice" => Self::NICE_ID,
            "is_elevated_token" => Self::IS_ELEVATED_TOKEN_ID,
            "cgroup_namespace" => Self::CGROUPE_NAMESPACE_ID,
            "ipc_namespace" => Self::IPC_NAMESPACE_ID,
            "mnt_namespace" => Self::MNT_NAMESPACE_ID,
            "net_namespace" => Self::NET_NAMESPACE_ID,
            "pid_namespace" => Self::PID_NAMESPACE_ID,
            "user_namespace" => Self::USER_NAMESPACE_ID,
            "uts_namespace" => Self::UTS_NAMESPACE_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessMemoryMapRow {
    pub pid: i32,
    pub start: String,
    pub end: String,
    pub permissions: String,
    pub offset: i64,
    pub device: String,
    pub inode: i32,
    pub path: String,
    pub pseudo: i32,
}

impl ProcessMemoryMapRow {
    const PID_ID: u64 = 0x00000001;
    const START_ID: u64 = 0x00000002;
    const END_ID: u64 = 0x00000004;
    const PERMISSION_ID: u64 = 0x00000008;
    const OFFSET_ID: u64 = 0x00000010;
    const DEVICE_ID: u64 = 0x00000020;
    const INODE_ID: u64 = 0x00000040;
    const PATH_ID: u64 = 0x00000080;
    const PSEUDO_ID: u64 = 0x00000100;
}

impl Table for ProcessMemoryMapRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "pid",
        "start",
        "end",
        "permissions",
        "offset",
        "device",
        "inode",
        "path",
        "pseudo"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "pid" => self.pid.to_string(),
            "start" => self.start.clone(),
            "end" => self.end.clone(),
            "permissions" => self.permissions.clone(),
            "offset" => self.offset.to_string(),
            "device" => self.device.clone(),
            "inode" => self.inode.to_string(),
            "path" => self.path.clone(),
            "pseudo" => self.pseudo.to_string(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::PID_ID => self.pid.to_string(),
            Self::START_ID => self.start.clone(),
            Self::END_ID => self.end.clone(),
            Self::PERMISSION_ID => self.permissions.clone(),
            Self::OFFSET_ID => self.offset.to_string(),
            Self::DEVICE_ID => self.device.clone(),
            Self::INODE_ID => self.inode.to_string(),
            Self::PATH_ID => self.path.clone(),
            Self::PSEUDO_ID => self.pseudo.to_string(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "pid" => Self::PID_ID,
            "start" => Self::START_ID,
            "end" => Self::END_ID,
            "permissions" => Self::PERMISSION_ID,
            "offset" => Self::OFFSET_ID,
            "device" => Self::DEVICE_ID,
            "inode" => Self::INODE_ID,
            "path" => Self::PATH_ID,
            "pseudo" => Self::PSEUDO_ID,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessEnvsRow {
    pub pid: i32,
    pub key: String,
    pub value: String,
}

impl ProcessEnvsRow {
    const PID_ID: u64 = 0x00000001;
    const KEY_ID: u64 = 0x00000002;
    const VALUE_ID: u64 = 0x00000004;
}

impl Table for ProcessEnvsRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "pid",
        "key",
        "value"];

    fn get_by_name(&self, _name: &str) -> String {
        match _name {
            "pid" => self.pid.to_string(),
            "key" => self.key.clone(),
            "value" => self.value.clone(),
            _ => "".to_string()
        }
    }

    fn get_by_id(&self, _id: u64) -> String {
        match _id {
            Self::PID_ID => self.pid.to_string(),
            Self::KEY_ID => self.key.clone(),
            Self::VALUE_ID => self.value.clone(),
            _ => "".to_string()
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "pid" => Self::PID_ID,
            "key" => Self::KEY_ID,
            "value" => Self::VALUE_ID,
            _ => 0
        }
    }
}