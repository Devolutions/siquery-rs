#[allow(unused_imports)]
use serde::ser::{Serialize, SerializeStruct, Serializer};
use rusqlite::types::*;

pub trait Table {
    const COLUMN_NAMES: &'static [&'static str];

    fn column_names(&self) -> &'static [&'static str] {
        Self::COLUMN_NAMES
    }

    fn get_by_name(&self, _name: &str) -> Value;
    fn get_by_id(&self, _id: u64) -> Value;
    fn get_id(&self, _name: &str) -> u64;
}

impl<T: Table> Table for Vec<T> {
    const COLUMN_NAMES: &'static [&'static str] = T::COLUMN_NAMES;

    fn get_by_name(&self, _name: &str) -> Value {
        unimplemented!()
    }
    fn get_by_id(&self, _id: u64) -> Value {
        unimplemented!()
    }
    fn get_id(&self, _name: &str) -> u64 {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! table_properties {
    (
    $(#[$attr:meta])*
    pub struct $name:ident {
            $(pub $field_name:ident: $field_type:ty,)*
    }) => {
        $(#[$attr])*
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub fn get_columns_name() -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
            }

            pub fn get_fields_type() -> Vec<&'static str> {
                vec![$(stringify!($field_type)),*]
            }

            pub fn get_columns_type() -> Vec<&'static str> {
                let mut columns_type = Vec::new();
                for t in $name::get_fields_type().iter() {
                    match *t {
                        "String" => columns_type.push("\" TEXT"),
                        "u8" => columns_type.push("\" INTEGER"),
                        "u16" => columns_type.push("\" INTEGER"),
                        "u32" => columns_type.push("\" INTEGER"),
                        "i8" => columns_type.push("\" INTEGER"),
                        "i16" => columns_type.push("\" INTEGER"),
                        "i32" => columns_type.push("\" INTEGER"),
                        "i64" => columns_type.push("\" INTEGER"),
                        "f64" => columns_type.push("\" REAL"),
                        "f32" => columns_type.push("\" TEXT"),
                        "isize" => columns_type.push("\" TEXT"),
                        "usize" => columns_type.push("\" TEXT"),
                        "Vec<String>" => columns_type.push("\" TEXT"),
                        _ => columns_type.push("\" NULL"),
                    }
                }
                    columns_type
            }
        }
    }
}

table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dummy {
    pub a: u32,
    pub b: i32,
}}

impl Dummy {
    const A_ID: u64 = 0x00000001;
    const B_ID: u64 = 0x00000002;
}

impl Table for Dummy {
    const COLUMN_NAMES: &'static [&'static str] = &["a", "b"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "a" => Value::from(self.a),
            "b" => Value::from(self.b),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::A_ID => Value::from(self.a),
            Self::B_ID => Value::from(self.b),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "a" => Self::A_ID,
            "b" => Self::B_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "etc_hosts")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcHosts {
    pub address: String,
    pub hostnames: String,
}}

#[cfg(feature = "etc_hosts")]
pub trait EtcHostsIface {
    fn get_hosts_file(&self) -> Option<String>;
}

#[cfg(feature = "etc_hosts")]
impl EtcHosts {
    const ADDRESS_ID: u64 = 0x00000001;
    const HOSTNAMES_ID: u64 = 0x00000002;
}

#[cfg(feature = "etc_hosts")]
impl Table for EtcHosts {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "address", "hostnames"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "address" => Value::from(self.address.to_owned()),
            "hostnames" => Value::from(self.hostnames.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::ADDRESS_ID => Value::from(self.address.to_owned()),
            Self::HOSTNAMES_ID => Value::from(self.hostnames.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "address" => Self::ADDRESS_ID,
            "hostnames" => Self::HOSTNAMES_ID,
            _ => 0
        }
    }
}
#[cfg(feature = "etc_protocols")]
table_properties! {
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcProtocols {
    pub name: String,
    pub number: u16,
    pub alias: String,
    pub comment: String,
}}

#[cfg(feature = "etc_protocols")]
pub trait EtcProtocolsIface {
    fn get_protocols_file(&self) -> Option<String>;
}

#[cfg(feature = "etc_protocols")]
#[allow(non_upper_case_globals)]
impl EtcProtocols {
    const NAME_ID: u64 = 0x00000001;
    const NUMBER_ID: u64 = 0x00000002;
    const ALIAS_ID: u64 = 0x00000004;
    const COMMENT_ID: u64 = 0x00000008;
}

#[cfg(feature = "etc_protocols")]
impl Table for EtcProtocols {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name", "number", "alias", "comment"];

    fn get_by_name(&self, _name: &str) -> Value {
        let value = match _name {
            "name" => Value::from(self.name.to_owned()),
            "number" => Value::from(self.number),
            "alias" => Value::from(self.alias.to_owned()),
            "comment" => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
        };
        value
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::NUMBER_ID => Value::from(self.number),
            Self::ALIAS_ID => Value::from(self.alias.to_owned()),
            Self::COMMENT_ID => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "etc_services")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EtcServices {
    pub name: String,
    pub port: u16,
    pub protocol: String,
    pub aliases: String,
    pub comment: String,
}}

#[cfg(feature = "etc_services")]
pub trait EtcServicesIface {
    fn get_services_file(&self) -> Option<String>;
}

#[cfg(feature = "etc_services")]
#[allow(non_upper_case_globals)]
impl EtcServices {
    const NAME_ID: u64 = 0x00000001;
    const PORT_ID: u64 = 0x00000002;
    const PROTOCOL_ID: u64 = 0x00000004;
    const ALIASES_ID: u64 = 0x00000008;
    const COMMENT_ID: u64 = 0x00000010;
}

#[cfg(feature = "etc_services")]
impl Table for EtcServices {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name", "port", "protocol", "aliases", "comment"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "port" => Value::from(self.port),
            "protocol" => Value::from(self.protocol.to_owned()),
            "aliases" => Value::from(self.aliases.to_owned()),
            "comment" => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PORT_ID => Value::from(self.port),
            Self::PROTOCOL_ID => Value::from(self.protocol.to_owned()),
            Self::ALIASES_ID => Value::from(self.aliases.to_owned()),
            Self::COMMENT_ID => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_computer_info" , fuzzing))]
table_properties!{
#[derive(Serialize, Debug)]
pub struct WmiComputerInfo {
    pub computer_name: String,
    pub domain: String,
    pub manufacturer: String,
    pub model: String,
    pub number_of_processors: u32,
    pub system_type: String,
}}

#[cfg(feature = "wmi_computer_info")]
pub trait WmiComputerInfoIface {
    fn get_wmi_computer_info(&self) -> Option<String>;
}

#[cfg(any(feature = "wmi_computer_info", fuzzing))]
#[allow(non_upper_case_globals)]
impl WmiComputerInfo {
    const COMPUTER_NAME_ID: u64 = 0x00000001;
    const DOMAIN_ID: u64 = 0x00000002;
    const MANUFACTURER_ID: u64 = 0x00000004;
    const MODEL_ID: u64 = 0x00000008;
    const NUMBER_OF_PROCESSORS_ID: u64 = 0x00000010;
    const SYSTEM_TYPE_ID: u64 = 0x00000020;
}

#[cfg(feature = "wmi_computer_info")]
impl Table for WmiComputerInfo {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "computer_name",
        "domain",
        "manufacturer",
        "model",
        "number_of_processors",
        "system_type"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "computer_name" => Value::from(self.computer_name.to_owned()),
            "domain" => Value::from(self.domain.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "model" => Value::from(self.model.to_owned()),
            "number_of_processors" => Value::from(self.number_of_processors),
            "system_type" => Value::from(self.system_type.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::COMPUTER_NAME_ID => Value::from(self.computer_name.to_owned()),
            Self::DOMAIN_ID => Value::from(self.domain.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::MODEL_ID => Value::from(self.model.to_owned()),
            Self::NUMBER_OF_PROCESSORS_ID => Value::from(self.number_of_processors.to_owned()),
            Self::SYSTEM_TYPE_ID => Value::from(self.system_type.to_owned()),
            _ => Value::from("".to_owned())
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
table_properties!{
#[derive(Serialize)]
pub struct SystemInfoData {
    pub computer_name: String,
    pub cpu_brand: String,
    pub cpu_logical_cores: u32,
    pub physical_memory: i64,
}}

#[cfg(feature = "system_info")]
pub trait SystemInfoDataIface {
    fn get_wmi_cpu_info(&self) -> Option<String>;
    fn get_wmi_system_info(&self) -> Option<String>;
    fn hostname(&self) -> Option<String>;
    fn meminfo(&self) -> Option<String>;
    fn cpuinfo(&self) -> Option<String>;
    fn cpu_count(&self) -> u32;
}

#[cfg(feature = "system_info")]
#[allow(non_upper_case_globals)]
impl SystemInfoData {
    const COMPUTER_NAME_ID: u64 = 0x00000001;
    const CPU_BRAND_ID: u64 = 0x00000002;
    const CPU_LOGICAL_CORES_ID: u64 = 0x00000004;
    const PHYSICAL_MEMORY_ID: u64 = 0x00000008;
}

#[cfg(feature = "system_info")]
impl Table for SystemInfoData {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "computer_name",
        "cpu_brand",
        "cpu_logical_cores",
        "physical_memory"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "computer_name" => Value::from(self.computer_name.to_owned()),
            "cpu_brand" => Value::from(self.cpu_brand.to_owned()),
            "cpu_logical_cores" => Value::from(self.cpu_logical_cores),
            "physical_memory" => Value::from(self.physical_memory),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::COMPUTER_NAME_ID => Value::from(self.computer_name.to_owned()),
            Self::CPU_BRAND_ID => Value::from(self.cpu_brand.to_owned()),
            Self::CPU_LOGICAL_CORES_ID => Value::from(self.cpu_logical_cores),
            Self::PHYSICAL_MEMORY_ID => Value::from(self.physical_memory),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_os_version", fuzzing))]
table_properties!{
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
}}

#[cfg(feature = "wmi_os_version")]
pub trait WmiOsVersionIface {
    fn get_wmi_os_info(&self) -> Option<String>;
}

#[cfg(feature = "wmi_os_version")]
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

#[cfg(feature = "wmi_os_version")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "build_number" => Value::from(self.build_number.to_owned()),
            "csname" => Value::from(self.csname.to_owned()),
            "caption" => Value::from(self.caption.to_owned()),
            "free_physical_mem" => Value::from(self.free_physical_mem.to_owned()),
            "free_virtual_mem" => Value::from(self.free_virtual_mem.to_owned()),
            "platform" => Value::from(self.platform.to_owned()),
            "version" => Value::from(self.version.to_owned()),
            "major" => Value::from(self.major.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "minor" => Value::from(self.minor.to_owned()),
            "name" => Value::from(self.name.to_owned()),
            "service_pack_major" => Value::from(self.service_pack_major.to_owned()),
            "service_pack_minor" => Value::from(self.service_pack_minor.to_owned()),
            "size_stored_in_paging_file" => Value::from(self.size_stored_in_paging_file.to_owned()),
            "total_virtual_mem_size" => Value::from(self.total_virtual_mem_size.to_owned()),
            "total_visible_mem_size" => Value::from(self.total_visible_mem_size.to_owned()),
            "win_directory" => Value::from(self.win_directory.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::BUILDER_NUMBER_ID => Value::from(self.build_number.to_owned()),
            Self::CSNAME_ID => Value::from(self.csname.to_owned()),
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::FREE_PHYSICAL_MEMORY_ID => Value::from(self.free_physical_mem.to_owned()),
            Self::FREE_VIRTUAL_MEMORY_ID => Value::from(self.free_virtual_mem.to_owned()),
            Self::PLATFORM_ID => Value::from(self.platform.to_owned()),
            Self::VERSION_ID => Value::from(self.version.to_owned()),
            Self::MAJOR_ID => Value::from(self.major.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::MINOR_ID => Value::from(self.minor.to_owned()),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::SERVICE_PACK_MAJOR_ID => Value::from(self.service_pack_major.to_owned()),
            Self::SERVICE_PACK_MINOR_ID => Value::from(self.service_pack_minor.to_owned()),
            Self::SIZE_STORED_IN_PAGING_FILE_ID => Value::from(self.size_stored_in_paging_file.to_owned()),
            Self::TOTAL_VIRTUAL_MEM_SIZE_ID => Value::from(self.total_virtual_mem_size.to_owned()),
            Self::TOTAL_VISIBLE_MEM_SIZE_ID => Value::from(self.total_visible_mem_size.to_owned()),
            Self::WIN_DIRECTORY_ID => Value::from(self.win_directory.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "os_version")]
table_properties!{
#[derive(Serialize, Deserialize)]
pub struct OsVersion {
    pub name: String,
    pub platform: String,
    pub version: String,
    pub major: u32,
    pub minor: u32,
}}

#[cfg(feature = "os_version")]
pub trait OsVersionIface {
    fn get_os_info(&self) -> Option<String>;
    fn os_release(&self) -> Option<String>;
    fn os_platform(&self) -> Option<String>;
}

#[cfg(feature = "os_version")]
#[allow(non_upper_case_globals)]
impl OsVersion {
    const NAME_ID: u64 = 0x00000001;
    const PLATFORM_ID: u64 = 0x00000002;
    const VERSION_ID: u64 = 0x00000004;
    const MAJOR_ID: u64 = 0x00000008;
    const MINOR_ID: u64 = 0x00000010;
}

#[cfg(feature = "os_version")]
impl Table for OsVersion {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "platform",
        "version",
        "major",
        "minor"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "platform" => Value::from(self.platform.to_owned()),
            "version" => Value::from(self.version.to_owned()),
            "major" => Value::from(self.major),
            "minor" => Value::from(self.minor),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PLATFORM_ID => Value::from(self.platform.to_owned()),
            Self::VERSION_ID => Value::from(self.version.to_owned()),
            Self::MAJOR_ID => Value::from(self.major),
            Self::MINOR_ID => Value::from(self.minor),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "logical_drives")]
table_properties!{
#[derive(Debug)]
pub struct LogicalDrive {
    pub description: String,
    pub drive_type: String,
    pub file_system: String,
    pub free_space: i64,
    pub maximum_component_length: i64,
    pub name : String,
    pub size: i64,
    pub supports_file_based_compression: String,
    pub volume_serial_number: String,
}}

#[cfg(feature = "logical_drives")]
pub trait LogicalDriveIface {
    fn get_wmi_drives_info(&self) -> Option<String>;
}

#[cfg(feature = "logical_drives")]
#[allow(non_upper_case_globals)]
impl LogicalDrive {
    const DESCRIPTION_ID: u64 = 0x00000001;
    const DRIVE_TYPE_ID: u64 = 0x00000002;
    const FILE_SYSTEM_ID: u64 = 0x00000004;
    const FREE_SPACE_ID: u64 = 0x00000008;
    const MAXIMUM_COMPONENT_LENGTH_ID: u64 = 0x00000010;
    const NAME_ID: u64 = 0x00000020;
    const SIZE_ID: u64 = 0x00000040;
    const SUPPORTS_FILE_BASED_COMPRESSION_ID: u64 = 0x00000080;
    const VOLUME_SERIAL_NUMBER_ID: u64 = 0x00000100;
}

#[cfg(feature = "logical_drives")]
impl Table for LogicalDrive {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "description",
        "drive_type",
        "file_system",
        "free_space",
        "maximum_component_length",
        "name",
        "size",
        "supports_file_based_compression",
        "volume_serial_number",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "description" => Value::from(self.description.to_owned()),
            "drive_type" =>  Value::from(self.drive_type.to_owned()),
            "file_system" => Value::from(self.file_system.to_owned()),
            "free_space" =>  Value::from(self.free_space),
            "maximum_component_length" => Value::from(self.maximum_component_length),
            "name" => Value::from(self.name.to_owned()),
            "size" => Value::from(self.size),
            "supports_file_based_compression" => Value::from(self.supports_file_based_compression.to_owned()),
            "volume_serial_number" => Value::from(self.volume_serial_number.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DRIVE_TYPE_ID => Value::from(self.drive_type.to_owned()),
            Self::FILE_SYSTEM_ID => Value::from(self.file_system.to_owned()),
            Self::FREE_SPACE_ID => Value::from(self.free_space),
            Self::MAXIMUM_COMPONENT_LENGTH_ID => Value::from(self.maximum_component_length),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::SIZE_ID => Value::from(self.size),
            Self::SUPPORTS_FILE_BASED_COMPRESSION_ID => Value::from(self.supports_file_based_compression.to_owned()),
            Self::VOLUME_SERIAL_NUMBER_ID => Value::from(self.volume_serial_number.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, name: &str) -> u64 {
        match name {
            "description" => Self::DESCRIPTION_ID,
            "drive_type" => Self::DRIVE_TYPE_ID,
            "file_system" => Self::FILE_SYSTEM_ID,
            "free_space" => Self::FREE_SPACE_ID,
            "maximum_component_length" => Self::MAXIMUM_COMPONENT_LENGTH_ID,
            "name" => Self::NAME_ID,
            "size" => Self::SIZE_ID,
            "supports_file_based_compression" => Self::SUPPORTS_FILE_BASED_COMPRESSION_ID,
            "volume_serial_number" => Self::VOLUME_SERIAL_NUMBER_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "interface_address")]
table_properties!{
#[derive(Debug)]
pub struct InterfaceAddress {
    pub interface: String,
    pub address: String,
    pub mask: String,
    pub interface_type: String,
    pub friendly_name: String,
    pub broadcast: String,
    pub point_to_point: String,
}}

#[cfg(feature = "interface_address")]
pub trait InterfaceAddressIface {
    fn get_wmi_nicconfig(&self) -> Option<String>;
}

#[cfg(feature = "interface_address")]
#[allow(non_upper_case_globals)]
impl InterfaceAddress {
    const INTERFACE_ID: u64 = 0x00000001;
    const ADDRESS_ID: u64 = 0x00000002;
    const MASK_ID: u64 = 0x00000004;
    const INTERFACE_TYPE_ID: u64 = 0x00000008;
    const FRIENDLY_NAME_ID: u64 = 0x00000010;
    const BROADCAST_ID: u64 = 0x00000020;
    const POINT_TO_POINT_ID: u64 = 0x00000040;
}

#[cfg(feature = "interface_address")]
impl Table for InterfaceAddress {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "interface",
        "address",
        "mask",
        "interface_type",
        "friendly_name",
        "broadcast",
        "point_to_point",];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "interface" => Value::from(self.interface.to_owned()),
            "address" => Value::from(self.address.to_owned()),
            "mask" => Value::from(self.mask.to_owned()),
            "interface_type" => Value::from(self.interface_type.to_owned()),
            "friendly_name" => Value::from(self.friendly_name.to_owned()),
            "broadcast" => Value::from(self.broadcast.to_owned()),
            "point_to_point" => Value::from(self.point_to_point.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::INTERFACE_ID => Value::from(self.interface.to_owned()),
            Self::ADDRESS_ID => Value::from(self.address.to_owned()),
            Self::MASK_ID => Value::from(self.mask.to_owned()),
            Self::INTERFACE_TYPE_ID => Value::from(self.interface_type.to_owned()),
            Self::FRIENDLY_NAME_ID => Value::from(self.friendly_name.to_owned()),
            Self::BROADCAST_ID => Value::from(self.broadcast.to_owned()),
            Self::POINT_TO_POINT_ID => Value::from(self.point_to_point.to_owned()),

            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "interface" => Self::INTERFACE_ID,
            "address" => Self::ADDRESS_ID,
            "mask" => Self::MASK_ID,
            "interface_type" => Self::INTERFACE_TYPE_ID,
            "friendly_name" => Self::FRIENDLY_NAME_ID,
            "broadcast" => Self::BROADCAST_ID,
            "point_to_point" => Self::POINT_TO_POINT_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "interface_address")]
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

#[cfg(feature = "interface_details")]
table_properties!{
#[derive(Debug, Serialize)]
pub struct InterfaceDetails {
    pub interface: String,
    pub mac: String,
    pub type_ : u32,
    pub mtu: u32,
    pub metric: u32,
    pub enabled: u8,
    pub flags: u32,
    pub ipackets: u32,
    pub opackets: u32,
    pub ibytes: u32,
    pub obytes: u32,
    pub ierrors: u32,
    pub oerrors: u32,
    pub idrops: u32,
    pub odrops: u32,
    pub collisions: u32,
    pub last_change: i64,
    pub link_speed : i64,
    pub pci_slot: String,
}}

#[cfg(feature = "interface_details")]
pub trait InterfaceDetailsIface {
    fn get_wmi_nicconfig_details(&self) -> Option<String>;
}

#[cfg(feature = "interface_details")]
#[allow(non_upper_case_globals)]
impl InterfaceDetails {
    const INTERFACE_ID: u64 = 0x00000001;
    const MAC_ID: u64 = 0x00000002;
    const TYPE_ID: u64 = 0x00000004;
    const MTU_ID: u64 = 0x00000008;
    const METRIC_ID: u64 = 0x00000010;
    const ENABLED_ID: u64 = 0x00000020;
    const FLAGS_ID: u64 = 0x00000040;
    const IPACKETS_ID: u64 = 0x00000080;
    const OPACKETS_ID: u64 = 0x00000100;
    const IBYTES_ID: u64 = 0x00000200;
    const OBYTES_ID: u64 = 0x00000400;
    const IERRORS_ID: u64 = 0x00000800;
    const OERRORS_ID: u64 = 0x00001000;
    const IDROPS_ID: u64 = 0x00002000;
    const ODROPS_ID: u64 = 0x00004000;
    const COLLISIONS_ID: u64 = 0x00008000;
    const LAST_CHANGE_ID: u64 = 0x00010000;
    const LINK_SPEED_ID: u64 = 0x00020000;
    const PCI_SLOT_ID: u64 = 0x00040000;
}

#[cfg(feature = "interface_details")]
impl Table for InterfaceDetails {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "interface",
        "mac",
        "type",
        "mtu",
        "metric",
        "enabled",
        "flags",
        "ipackets",
        "opackets",
        "ibytes",
        "obytes",
        "ierrors",
        "oerrors",
        "idrops",
        "odrops",
        "collisions",
        "last_change",
        "link_speed",
        "pci_slot",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "interface" => Value::from(self.interface.to_owned()),
            "mac" => Value::from(self.mac.to_owned()),
            "type" => Value::from(self.type_),
            "mtu" => Value::from(self.mtu),
            "metric" => Value::from(self.metric),
            "enabled" => Value::from(self.enabled),
            "flags" => Value::from(self.flags),
            "ipackets" => Value::from(self.ipackets),
            "opackets" => Value::from(self.opackets),
            "ibytes" => Value::from(self.ibytes),
            "obytes" => Value::from(self.obytes),
            "ierrors" => Value::from(self.ierrors),
            "oerrors" => Value::from(self.oerrors),
            "idrops" => Value::from(self.idrops),
            "odrops" => Value::from(self.odrops),
            "collisions" => Value::from(self.collisions),
            "last_change" => Value::from(self.last_change),
            "link_speed" => Value::from(self.link_speed),
            "pci_slot" => Value::from(self.pci_slot.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::INTERFACE_ID => Value::from(self.interface.to_owned()),
            Self::MAC_ID => Value::from(self.mac.to_owned()),
            Self::TYPE_ID => Value::from(self.type_),
            Self::MTU_ID => Value::from(self.mtu),
            Self::METRIC_ID => Value::from(self.metric),
            Self::ENABLED_ID => Value::from(self.enabled),
            Self::FLAGS_ID => Value::from(self.flags),
            Self::IPACKETS_ID => Value::from(self.ipackets),
            Self::OPACKETS_ID => Value::from(self.opackets),
            Self::IBYTES_ID => Value::from(self.ibytes),
            Self::OBYTES_ID => Value::from(self.obytes),
            Self::IERRORS_ID => Value::from(self.ierrors),
            Self::OERRORS_ID => Value::from(self.oerrors),
            Self::IDROPS_ID => Value::from(self.idrops),
            Self::ODROPS_ID => Value::from(self.odrops),
            Self::COLLISIONS_ID => Value::from(self.collisions),
            Self::LAST_CHANGE_ID => Value::from(self.last_change),
            Self::LINK_SPEED_ID => Value::from(self.link_speed),
            Self::PCI_SLOT_ID => Value::from(self.pci_slot.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "interface" => Self::INTERFACE_ID,
            "mac" => Self::MAC_ID,
            "type" => Self::TYPE_ID,
            "mtu" => Self::MTU_ID,
            "metric" => Self::METRIC_ID,
            "enabled" => Self::ENABLED_ID,
            "flags" => Self::FLAGS_ID,
            "ipackets" => Self::IPACKETS_ID,
            "opackets" => Self::OPACKETS_ID,
            "ibytes" => Self::IBYTES_ID,
            "obytes" => Self::OBYTES_ID,
            "ierrors" => Self::IERRORS_ID,
            "oerrors" => Self::OERRORS_ID,
            "idrops" => Self::IDROPS_ID,
            "odrops" => Self::ODROPS_ID,
            "collisions" => Self::COLLISIONS_ID,
            "last_change" => Self::LAST_CHANGE_ID,
            "link_speed" => Self::LINK_SPEED_ID,
            "pci_slot" => Self::PCI_SLOT_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "uptime")]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct Uptime {
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub total_seconds: i64,
}}

#[cfg(feature = "uptime")]
#[allow(non_upper_case_globals)]
impl Uptime {
    const DAYS_ID: u64 = 0x00000001;
    const HOURS_ID: u64 = 0x00000002;
    const MINUTES_ID: u64 = 0x00000004;
    const SECONDS_ID: u64 = 0x00000008;
    const TOTAL_SECONDS_ID: u64 = 0x00000010;
}

#[cfg(feature = "uptime")]
impl Table for Uptime {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "days",
        "hours",
        "minutes",
        "seconds",
        "total_seconds", ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "days" => Value::from(self.days),
            "hours" => Value::from(self.hours),
            "minutes" => Value::from(self.minutes),
            "seconds" => Value::from(self.seconds),
            "total_seconds" => Value::from(self.total_seconds),
            _ => Value::from(0),
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::DAYS_ID => Value::from(self.days),
            Self::HOURS_ID => Value::from(self.hours),
            Self::MINUTES_ID => Value::from(self.minutes),
            Self::SECONDS_ID => Value::from(self.seconds),
            Self::TOTAL_SECONDS_ID => Value::from(self.total_seconds),
            _ => Value::from(0)
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

#[cfg(any(feature = "wmi_printers", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiPrinters {
    pub attributes: u32,
    pub caption: String,
    pub creation_class_name: String,
    pub device_id: String,
    pub do_complete_first: String,
    pub driver_name: String,
    pub extended_printer_status: u16,
    pub horizontal_resolution: u32,
    pub local: String,
    pub name: String,
    pub port_name: String,
    pub printer_status: u16,
    pub print_job_data_type: String,
    pub print_processor: String,
    pub priority: u32,
    pub status: String,
    pub system_creation_class_name: String,
    pub system_name: String,
    pub vertical_resolution: u32,
}}

#[cfg(feature = "wmi_printers")]
pub trait WmiPrintersIface {
    fn get_wmi_printers_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_printers")]
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

#[cfg(feature = "wmi_printers")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "attributes" => Value::from(self.attributes),
            "caption" => Value::from(self.caption.to_owned()),
            "creation_class_name" => Value::from(self.creation_class_name.to_owned()),
            "device_id" => Value::from(self.device_id.to_owned()),
            "do_complete_first" => Value::from(self.do_complete_first.to_owned()),
            "driver_name" => Value::from(self.driver_name.to_owned()),
            "extended_printer_status" => Value::from(self.extended_printer_status),
            "horizontal_resolution" => Value::from(self.horizontal_resolution),
            "local" => Value::from(self.local.to_owned()),
            "name" => Value::from(self.name.to_owned()),
            "port_name" => Value::from(self.port_name.to_owned()),
            "printer_status" => Value::from(self.printer_status),
            "print_job_data_type" => Value::from(self.print_job_data_type.to_owned()),
            "print_processor" => Value::from(self.print_processor.to_owned()),
            "priority" => Value::from(self.priority),
            "status" => Value::from(self.status.to_owned()),
            "system_creation_class_name" => Value::from(self.system_creation_class_name.to_owned()),
            "system_name" => Value::from(self.system_name.to_owned()),
            "vertical_resolution" => Value::from(self.vertical_resolution),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::ATTRIBUTES_ID => Value::from(self.attributes),
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::CREATION_CLASS_ID => Value::from(self.creation_class_name.to_owned()),
            Self::DEVICE_ID => Value::from(self.device_id.to_owned()),
            Self::DO_COMPLETE_FIRST_ID => Value::from(self.do_complete_first.to_owned()),
            Self::DRIVER_NAME_ID => Value::from(self.driver_name.to_owned()),
            Self::EXTENDED_PRINTER_STATUS_ID => Value::from(self.extended_printer_status),
            Self::HORIZONTAL_RESOLUTION_ID => Value::from(self.horizontal_resolution),
            Self::LOCAL_ID => Value::from(self.local.to_owned()),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PORT_NAME_ID => Value::from(self.port_name.to_owned()),
            Self::PRINTER_STATUS_ID => Value::from(self.printer_status),
            Self::PRINT_JOB_DATA_TYPE_ID => Value::from(self.print_job_data_type.to_owned()),
            Self::PRINT_PROCESSOR_ID => Value::from(self.print_processor.to_owned()),
            Self::PRIORITY_ID => Value::from(self.priority),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            Self::SYSTEM_CREATION_CLASS_NAME_ID => Value::from(self.system_creation_class_name.to_owned()),
            Self::SYSTEM_NAME_ID => Value::from(self.system_name.to_owned()),
            Self::VERTICAL_RESOLUTION_ID => Value::from(self.vertical_resolution),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_services", fuzzing))]
table_properties!{
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
}}

#[cfg(feature = "wmi_services")]
pub trait WmiServicesIface {
    fn get_wmi_services_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_services")]
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

#[cfg(feature = "wmi_services")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "accept_pause" => Value::from(self.accept_pause.to_owned()),
            "accept_stop" => Value::from(self.accept_stop.to_owned()),
            "caption" => Value::from(self.caption.to_owned()),
            "creation_class_name" => Value::from(self.creation_class_name.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "desktop_interact" => Value::from(self.desktop_interact.to_owned()),
            "display_name" => Value::from(self.display_name.to_owned()),
            "error_control" => Value::from(self.error_control.to_owned()),
            "exit_code" => Value::from(self.exit_code),
            "name" => Value::from(self.name.to_owned()),
            "path_name" => Value::from(self.path_name.to_owned()),
            "service_type" => Value::from(self.service_type.to_owned()),
            "started" => Value::from(self.started.to_owned()),
            "start_mode" => Value::from(self.start_mode.to_owned()),
            "start_name" => Value::from(self.start_name.to_owned()),
            "state" => Value::from(self.state.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            "system_creation_class_name" => Value::from(self.system_creation_class_name.to_owned()),
            "system_name" => Value::from(self.system_name.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::ACCEPT_PAUSE_ID => Value::from(self.accept_pause.to_owned()),
            Self::ACCEPT_STOP_ID => Value::from(self.accept_stop.to_owned()),
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::CREATION_CLASS_NAME_ID => Value::from(self.creation_class_name.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DESKTOP_INTERACT_ID => Value::from(self.desktop_interact.to_owned()),
            Self::DISPLAY_NAME_ID => Value::from(self.display_name.to_owned()),
            Self::ERROR_CONTROL_ID => Value::from(self.error_control.to_owned()),
            Self::EXIT_CODE_ID => Value::from(self.exit_code),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PATH_NAME_ID => Value::from(self.path_name.to_owned()),
            Self::SERVICE_TYPE_ID => Value::from(self.service_type.to_owned()),
            Self::STARTED_ID => Value::from(self.started.to_owned()),
            Self::START_MODE_ID => Value::from(self.start_mode.to_owned()),
            Self::START_NAME_ID => Value::from(self.start_name.to_owned()),
            Self::STATE_ID => Value::from(self.state.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            Self::SYSTEM_CREATION_CLASS_NAME_ID => Value::from(self.system_creation_class_name.to_owned()),
            Self::SYSTEM_NAME_ID => Value::from(self.system_name.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_hotfixes", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiHotfixes {
    pub caption: String,
    pub csname: String,
    pub description: String,
    pub hotfix_id: String,
    pub installed_by: String,
    pub installed_on: String,
}}

#[cfg(feature = "wmi_hotfixes")]
pub trait WmiHotfixesIface {
    fn get_wmi_hotfixes_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_hotfixes")]
#[allow(non_upper_case_globals)]
impl WmiHotfixes {
    const CAPTION_ID: u64 = 0x00000001;
    const CSNAME_ID: u64 = 0x00000002;
    const DESCRIPTION_ID: u64 = 0x00000004;
    const HOTFIX_ID: u64 = 0x00000008;
    const INSTALLED_BY_ID: u64 = 0x00000010;
    const INSTALLED_ON_ID: u64 = 0x00000020;
}

#[cfg(feature = "wmi_hotfixes")]
impl Table for WmiHotfixes {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "csname",
        "description",
        "hotfix_id",
        "installed_by",
        "installed_ON"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "caption" => Value::from(self.caption.to_owned()),
            "csname" => Value::from(self.csname.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "hotfix_id" => Value::from(self.hotfix_id.to_owned()),
            "installed_by" => Value::from(self.installed_by.to_owned()),
            "installed_on" => Value::from(self.installed_on.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::CSNAME_ID => Value::from(self.csname.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::HOTFIX_ID => Value::from(self.hotfix_id.to_owned()),
            Self::INSTALLED_BY_ID => Value::from(self.installed_by.to_owned()),
            Self::INSTALLED_ON_ID => Value::from(self.installed_on.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "products", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    pub install_date: String,
    pub install_location: String,
    pub help_link: String,
    pub name: String,
    pub vendor: String,
    pub version: String,
}}

#[cfg(feature = "products")]
#[allow(non_upper_case_globals)]
impl Products {
    const INSTALL_DATE_ID: u64 = 0x00000001;
    const INSTALL_LOCATION_ID: u64 = 0x00000002;
    const HELP_LINK_ID: u64 = 0x00000004;
    const NAME_ID: u64 = 0x00000008;
    const VENDOR_ID: u64 = 0x00000010;
    const VERSION_ID: u64 = 0x00000020;
}

#[cfg(feature = "products")]
impl Table for Products {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "install_date",
        "install_location",
        "help_link",
        "name",
        "vendor",
        "version"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "install_date" => Value::from(self.install_date.to_owned()),
            "install_location" => Value::from(self.install_location.to_owned()),
            "help_link" => Value::from(self.help_link.to_owned()),
            "name" => Value::from(self.name.to_owned()),
            "vendor" => Value::from(self.vendor.to_owned()),
            "version" => Value::from(self.version.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::INSTALL_DATE_ID => Value::from(self.install_date.to_owned()),
            Self::INSTALL_LOCATION_ID => Value::from(self.install_location.to_owned()),
            Self::HELP_LINK_ID => Value::from(self.help_link.to_owned()),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::VENDOR_ID => Value::from(self.vendor.to_owned()),
            Self::VERSION_ID => Value::from(self.version.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_network_adapters", fuzzing))]
table_properties!{
pub struct WmiNetworkAdapters {
    pub description: String,
    pub database_path: String,
    pub dhcp_enabled: String,
    pub ip_address: Vec<String>,
    pub ip_enabled: String,
    pub ip_subnet: Vec<String>,
    pub mac_address: String,
}}

#[cfg(feature = "wmi_network_adapters")]
pub trait WmiNetworkAdaptersIface {
    fn get_wmi_network_adapters_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_network_adapters")]
impl WmiNetworkAdapters {
    const DESCRIPTION_ID: u64 = 0x00000001;
    const DATE_BASE_PATH_ID: u64 = 0x00000002;
    const DHCP_ENABLED_ID: u64 = 0x00000004;
    const IP_ADDRESS_ID: u64 = 0x00000008;
    const IP_ENABLED_ID: u64 = 0x00000010;
    const IP_SUBNET_ID: u64 = 0x00000020;
    const MAC_ADDRESS_ID: u64 = 0x00000040;
}

#[cfg(feature = "wmi_network_adapters")]
impl Table for WmiNetworkAdapters {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "description",
        "database_path",
        "dhcp_enabled",
        "ip_address",
        "ip_enabled",
        "ip_subnet",
        "mac_address"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "description" => Value::from(self.description.to_owned()),
            "database_path" => Value::from(self.database_path.to_owned()),
            "dhcp_enabled" => Value::from(self.dhcp_enabled.to_owned()),
            "ip_address" => {
                let mut ip_address_str: String = "".to_string().to_owned();
                for address in self.ip_address.iter() {
                    ip_address_str.push_str(&address);
                    ip_address_str.push_str("\t");
                }
                Value::from(ip_address_str)
            }
            "ip_enabled" => Value::from(self.ip_enabled.to_owned()),
            "ip_subnet" => {
                let mut ip_subnet_str: String = "".to_string().to_owned();
                for subnet in self.ip_subnet.iter() {
                    ip_subnet_str.push_str(&subnet);
                    ip_subnet_str.push_str("\t");
                }
                Value::from(ip_subnet_str)
            }
            "mac_address" => Value::from(self.mac_address.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DATE_BASE_PATH_ID => Value::from(self.database_path.to_owned()),
            Self::DHCP_ENABLED_ID => Value::from(self.dhcp_enabled.to_owned()),
            Self::IP_ADDRESS_ID => {
                let mut ip_address_str: String = "".to_string().to_owned();
                for address in self.ip_address.iter() {
                    ip_address_str.push_str(&address);
                    ip_address_str.push_str("\t");
                }
                Value::from(ip_address_str)
            }
            Self::IP_ENABLED_ID => Value::from(self.ip_enabled.to_owned()),
            Self::IP_SUBNET_ID => {
                let mut ip_subnet_str: String = "".to_string().to_owned();
                for subnet in self.ip_subnet.iter() {
                    ip_subnet_str.push_str(&subnet);
                    ip_subnet_str.push_str("\t");
                }
                Value::from(ip_subnet_str)
            }
            Self::MAC_ADDRESS_ID => Value::from(self.mac_address.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "description" => Self::DESCRIPTION_ID,
            "database_path" => Self::DATE_BASE_PATH_ID,
            "dhcp_enabled" => Self::DHCP_ENABLED_ID,
            "ip_address" => Self::IP_ADDRESS_ID,
            "ip_enabled"=>  Self::IP_ENABLED_ID,
            "ip_subnet" => Self::IP_SUBNET_ID,
            "mac_address" => Self::MAC_ADDRESS_ID,
            _ => 0
        }
    }
}

#[cfg(any(feature = "wmi_shares", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiShares {
    pub caption: String,
    pub description: String,
    pub name: String,
    pub path: String,
    pub status: String,
    pub _type: String,
    pub allow_maximum: String,
}}

#[cfg(feature = "wmi_shares")]
pub trait WmiSharesIface {
    fn get_wmi_shares_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_shares")]
impl WmiShares {
    const CAPTION_ID: u64 = 0x00000001;
    const DESCRIPTION_ID: u64 = 0x00000002;
    const NAME_ID: u64 = 0x00000004;
    const PATH_ID: u64 = 0x00000008;
    const STATUS_ID: u64 = 0x00000010;
    const TYPE_ID: u64 = 0x00000020;
    const ALLOW_MAXIMUM_ID: u64 = 0x00000040;
}

#[cfg(feature = "wmi_shares")]
impl Table for WmiShares {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "description",
        "name",
        "path",
        "status",
        "type",
        "allow_maximum"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "caption" => Value::from(self.caption.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "name" => Value::from(self.name.to_owned()),
            "path" => Value::from(self.path.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            "type" => Value::from(self._type.to_owned()),
            "allow_maximum" => Value::from(self.allow_maximum.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PATH_ID => Value::from(self.path.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            Self::TYPE_ID => Value::from(self._type.to_owned()),
            Self::ALLOW_MAXIMUM_ID => Value::from(self.allow_maximum.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_local_accounts",fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiLocalAccounts {
    pub account_type: String,
    pub caption: String,
    pub description: String,
    pub _domain: String,
    pub local_account: String,
    pub name: String,
    pub sid: String,
    pub sid_type: u8,
    pub status: String,
}}

#[cfg(feature = "wmi_local_accounts")]
pub trait WmiLocalAccountsIface {
    fn get_wmi_local_accounts_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_local_accounts")]
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

#[cfg(feature = "wmi_local_accounts")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "account_type" => Value::from(self.account_type.to_owned()),
            "caption" => Value::from(self.caption.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "domain" => Value::from(self._domain.to_owned()),
            "local_account" => Value::from(self.local_account.to_owned()),
            "name" => Value::from(self.name.to_owned()),
            "sid" => Value::from(self.sid.to_owned()),
            "sid_type" => Value::from(self.sid_type),
            "status" => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::ACCOUNT_TYPE_ID => Value::from(self.account_type.to_owned()),
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DOMAIN_ID => Value::from(self._domain.to_owned()),
            Self::LOCAL_ACCOUNT_ID => Value::from(self.local_account.to_owned()),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::SID_ID => Value::from(self.sid.to_owned()),
            Self::SID_TYPE_ID => Value::from(self.sid_type),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_bios",fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiBios {
    pub caption: String,
    pub manufacturer: String,
    pub release_date: String,
    pub serial_number: String,
    pub smbios_version: String,
}}

#[cfg(feature = "wmi_bios")]
pub trait WmiBiosIface {
    fn get_wmi_bios_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_bios")]
impl WmiBios {
    const CAPTION_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const RELEASE_DATE_ID: u64 = 0x00000004;
    const SERIAL_NUMBER_ID: u64 = 0x00000008;
    const SMBIOS_VERSION_ID: u64 = 0x00000010;
}

#[cfg(feature = "wmi_bios")]
impl Table for WmiBios {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "caption",
        "manufacturer",
        "release_date",
        "serial_number",
        "smbios_version"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "caption" => Value::from(self.caption.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "release_date" => Value::from(self.release_date.to_owned()),
            "serial_number" => Value::from(self.serial_number.to_owned()),
            "smbios_version" => Value::from(self.smbios_version.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::CAPTION_ID => Value::from(self.caption.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::RELEASE_DATE_ID => Value::from(self.release_date.to_owned()),
            Self::SERIAL_NUMBER_ID => Value::from(self.serial_number.to_owned()),
            Self::SMBIOS_VERSION_ID => Value::from(self.smbios_version.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_motherboard", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMotherboard {
    pub name: String,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
    pub version: String,
}}

#[cfg(feature = "wmi_motherboard")]
pub trait WmiMotherboardIface {
    fn get_wmi_motherboard_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_motherboard")]
impl WmiMotherboard {
    const NAME_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const PRODUCT_ID: u64 = 0x00000004;
    const SERIAL_NUMBER_ID: u64 = 0x00000008;
    const VERSION_ID: u64 = 0x00000010;
}

#[cfg(feature = "wmi_motherboard")]
impl Table for WmiMotherboard {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "manufacturer",
        "product",
        "serial_number",
        "version"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "product" => Value::from(self.product.to_owned()),
            "serial_number" => Value::from(self.serial_number.to_owned()),
            "version" => Value::from(self.version.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::PRODUCT_ID => Value::from(self.product.to_owned()),
            Self::SERIAL_NUMBER_ID => Value::from(self.serial_number.to_owned()),
            Self::VERSION_ID => Value::from(self.version.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_processor", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiProcessor {
    pub address_width: u16,
    pub cpu_satus: String,
    pub current_clock_speed: u32,
    pub current_voltage: u16,
    pub description: String,
    pub external_clock: u32,
    pub hyper_threading_enabled: String,
    pub l2_cache_size: u32,
    pub l2_cache_speed: u32,
    pub l3_cache_size: u32,
    pub l3_cache_speed: u32,
    pub manufacturer: String,
    pub max_clock_speed: u32,
    pub name: String,
    pub number_of_cores: u32,
    pub number_of_logical_processors: u32,
    pub socket_designation: String,
}}

#[cfg(feature = "wmi_processor")]
pub trait WmiProcessorIface {
    fn get_wmi_processor_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_processor")]
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

#[cfg(feature = "wmi_processor")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "address_width" => Value::from(self.address_width),
            "cpu_satus" => Value::from(self.cpu_satus.to_owned()),
            "current_clock_speed" => Value::from(self.current_clock_speed),
            "current_voltage" => Value::from(self.current_voltage),
            "description" => Value::from(self.description.to_owned()),
            "external_clock" => Value::from(self.external_clock),
            "hyper_threading_enabled" => Value::from(self.hyper_threading_enabled.to_owned()),
            "l2_cache_size" => Value::from(self.l2_cache_size),
            "l2_cache_speed" => Value::from(self.l2_cache_speed),
            "l3_cache_size" => Value::from(self.l3_cache_size),
            "l3_cache_speed" => Value::from(self.l3_cache_speed),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "max_clock_speed" => Value::from(self.max_clock_speed),
            "name" => Value::from(self.name.to_owned()),
            "number_of_cores" => Value::from(self.number_of_cores),
            "number_of_logical_processors" => Value::from(self.number_of_logical_processors),
            "socket_designation" => Value::from(self.socket_designation.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::ADDRESS_WIDTH_ID => Value::from(self.address_width),
            Self::CPU_STATUS_ID => Value::from(self.cpu_satus.to_owned()),
            Self::CURRENT_CLOCK_SPEED_ID => Value::from(self.current_clock_speed),
            Self::CURRENT_VOLTAGE_ID => Value::from(self.current_voltage),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::EXTERNAL_CLOCK_ID => Value::from(self.external_clock),
            Self::HYPER_THREADING_ENABLED_ID => Value::from(self.hyper_threading_enabled.to_owned()),
            Self::L2_CACHE_SIZE_ID => Value::from(self.l2_cache_size),
            Self::L2_CACHE_SPEED_ID => Value::from(self.l2_cache_speed),
            Self::L3_CACHE_SIZE_ID => Value::from(self.l3_cache_size),
            Self::L3_CACHE_SPEED_ID => Value::from(self.l3_cache_speed),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::MAX_CLOCK_SPEED_ID => Value::from(self.max_clock_speed),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::NUMBER_OF_CORES_ID => Value::from(self.number_of_cores),
            Self::NUMBER_OF_LOGICAL_PROCESSORS_ID => Value::from(self.number_of_logical_processors),
            Self::SOCKET_DESIGNATION_ID => Value::from(self.socket_designation.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_physical_memory",fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMemory {
    pub name: String,
    pub bank_label: String,
    pub capacity: String,
    pub description: String,
    pub device_locator: String,
    pub form_factor: u16,
    pub interleave_data_depth: u16,
    pub interleave_position: u32,
    pub manufacturer: String,
    pub memory_type: u16,
    pub serial_number: String,
    pub speed: u32,
}}

#[cfg(feature = "wmi_physical_memory")]
pub trait WmiMemoryIface {
    fn get_wmi_physical_memory(&self)-> Option<String>;
}

#[cfg(feature = "wmi_physical_memory")]
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

#[cfg(feature = "wmi_physical_memory")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "bank_label" => Value::from(self.bank_label.to_owned()),
            "capacity" => Value::from(self.capacity.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "device_locator" => Value::from(self.device_locator.to_owned()),
            "form_factor" => Value::from(self.form_factor),
            "interleave_data_depth" => Value::from(self.interleave_data_depth),
            "interleave_position" => Value::from(self.interleave_position),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "memory_type" => Value::from(self.memory_type),
            "serial_number" => Value::from(self.serial_number.to_owned()),
            "speed" => Value::from(self.speed),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::BANK_LABEL_ID => Value::from(self.bank_label.to_owned()),
            Self::CAPACITY_ID => Value::from(self.capacity.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DEVICE_LOCATOR_ID => Value::from(self.device_locator.to_owned()),
            Self::FORM_FACTOR_ID => Value::from(self.form_factor),
            Self::INTERLEAVE_DATA_DEPTH_ID => Value::from(self.interleave_data_depth),
            Self::INTERLEAVE_POSITION_ID => Value::from(self.interleave_position),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::MEMORY_TYPE_ID => Value::from(self.memory_type),
            Self::SERIAL_NUMBER_ID => Value::from(self.serial_number.to_owned()),
            Self::SPEED_ID => Value::from(self.speed),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_sound", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiSound {
    pub name: String,
    pub status: String,
    pub manufacturer: String,
    pub dma_buffer_size: u16,
}}

#[cfg(feature = "wmi_sound")]
pub trait WmiSoundIface {
    fn get_wmi_sound_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_sound")]
impl WmiSound {
    const NAME_ID: u64 = 0x00000001;
    const STATUS_ID: u64 = 0x00000002;
    const MANUFACTURER_ID: u64 = 0x00000004;
    const DMA_BUFFER_SIZE_ID: u64 = 0x00000008;
}

#[cfg(feature = "wmi_sound")]
impl Table for WmiSound {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "status",
        "manufacturer",
        "dma_buffer_size"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "dma_buffer_size" => Value::from(self.dma_buffer_size),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::DMA_BUFFER_SIZE_ID => Value::from(self.dma_buffer_size),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_video",fuzzing))]
table_properties! {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct WmiVideo {
        pub name: String,
        pub adapter_compatibility: String,
        pub adapter_dac_type: String,
        pub adapter_ram: u32,
        pub availability: String,
        pub driver_version: String,
        pub installed_display_driver: Vec<String>,
        pub refresh_rate: String,
        pub screen_info: String,
        pub status: String,
        pub video_architecture: String,
        pub video_memory_type: String,
    }
}

#[cfg(feature = "wmi_video")]
pub trait WmiVideoIface {
    fn get_wmi_video_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_video")]
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

#[cfg(feature = "wmi_video")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "adapter_compatibility" => Value::from(self.adapter_compatibility.to_owned()),
            "adapter_dac_type" => Value::from(self.adapter_dac_type.to_owned()),
            "adapter_ram" => Value::from(self.adapter_ram),
            "availability" => Value::from(self.availability.to_owned()),
            "driver_version" => Value::from(self.driver_version.to_owned()),
            "installed_display_driver" => {
                let mut installed_display_driver: String = "".to_owned();
                for driver in self.installed_display_driver.iter() {
                    installed_display_driver.push_str(&driver);
                    installed_display_driver.push_str("\t");
                }
                Value::from(installed_display_driver)
            }
            "refresh_rate" => Value::from(self.refresh_rate.to_owned()),
            "screen_info" => Value::from(self.screen_info.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            "video_architecture" => Value::from(self.video_architecture.to_owned()),
            "video_memory_type" => Value::from(self.video_memory_type.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::ADAPTER_COMPATIBILITY_ID => Value::from(self.adapter_compatibility.to_owned()),
            Self::ADAPTER_DAC_TYPE_ID => Value::from(self.adapter_dac_type.to_owned()),
            Self::ADAPTER_RAM_ID => Value::from(self.adapter_ram),
            Self::AVAILABILITY_ID => Value::from(self.availability.to_owned()),
            Self::DRIVER_VERSION_ID => Value::from(self.driver_version.to_owned()),
            Self::INSTALLED_DISPLAY_DRIVER_ID => {
                let mut installed_display_driver: String = "".to_owned();
                for driver in self.installed_display_driver.iter() {
                    installed_display_driver.push_str(&driver);
                    installed_display_driver.push_str("\t");
                }
                Value::from(installed_display_driver)
            }
            Self::REFRESH_RATE_ID => Value::from(self.refresh_rate.to_owned()),
            Self::SCREEN_INFO_ID => Value::from(self.screen_info.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            Self::VIDEO_ARCHITECTURE_ID => Value::from(self.video_architecture.to_owned()),
            Self::VIDEO_MEMORY_TYPE_ID => Value::from(self.video_memory_type.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_monitors",fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiMonitors {
    pub name: String,
    pub availability: String,
    pub bandwidth: u32,
    pub manufacturer: String,
    pub screen_height: u32,
    pub screen_width: u32,
}}

#[cfg(feature = "wmi_monitors")]
pub trait WmiMonitorsIface {
    fn get_wmi_monitor_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_monitors")]
impl WmiMonitors {
    const NAME_ID: u64 = 0x00000001;
    const AVAILABILITY_ID: u64 = 0x00000002;
    const BANDWIDTH_ID: u64 = 0x00000004;
    const MANUFACTURER_ID: u64 = 0x00000008;
    const SCREEN_HEIGHT_ID: u64 = 0x00000010;
    const SCREEN_WIDTH_ID: u64 = 0x00000020;
}

#[cfg(feature = "wmi_monitors")]
impl Table for WmiMonitors {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "availability",
        "bandwidth",
        "manufacturer",
        "screen_height",
        "screen_width"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "availability" => Value::from(self.availability.to_owned()),
            "bandwidth" => Value::from(self.bandwidth),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "screen_height" => Value::from(self.screen_height),
            "screen_width" => Value::from(self.screen_width),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::AVAILABILITY_ID => Value::from(self.availability.to_owned()),
            Self::BANDWIDTH_ID => Value::from(self.bandwidth),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::SCREEN_HEIGHT_ID => Value::from(self.screen_height),
            Self::SCREEN_WIDTH_ID => Value::from(self.screen_width),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_keyboard", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiKeyboard {
    pub name: String,
    pub description: String,
    pub device_id: String,
    pub status: String,
}}

#[cfg(feature = "wmi_keyboard")]
pub trait WmiKeyboardIface {
    fn get_wmi_keyboard_info(&self)-> Option<String>;
}

#[cfg(feature = "wmi_keyboard")]
impl WmiKeyboard {
    const NAME_ID: u64 = 0x00000001;
    const DESCRIPTION_ID: u64 = 0x00000002;
    const DEVICE_ID: u64 = 0x00000004;
    const STATUS_ID: u64 = 0x00000008;
}

#[cfg(feature = "wmi_keyboard")]
impl Table for WmiKeyboard {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "description",
        "device_id",
        "status"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "device_id" => Value::from(self.device_id.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::DEVICE_ID => Value::from(self.device_id.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(any(feature = "wmi_pointing_device", fuzzing))]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct WmiPointingDevice {
    pub name: String,
    pub manufacturer: String,
    pub description: String,
    pub pointing_type: String,
    pub status: String,
}}

#[cfg(feature = "wmi_pointing_device")]
pub trait WmiPointingDeviceIface {
    fn get_wmi_pointing_device(&self)-> Option<String>;
}

#[cfg(feature = "wmi_pointing_device")]
impl WmiPointingDevice {
    const NAME_ID: u64 = 0x00000001;
    const MANUFACTURER_ID: u64 = 0x00000002;
    const DESCRIPTION_ID: u64 = 0x00000004;
    const POINTING_TYPE_ID: u64 = 0x00000008;
    const STATUS_ID: u64 = 0x00000010;
}

#[cfg(feature = "wmi_pointing_device")]
impl Table for WmiPointingDevice {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "name",
        "manufacturer",
        "description",
        "pointing_type",
        "status"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "name" => Value::from(self.name.to_owned()),
            "manufacturer" => Value::from(self.manufacturer.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "pointing_type" => Value::from(self.pointing_type.to_owned()),
            "status" => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::MANUFACTURER_ID => Value::from(self.manufacturer.to_owned()),
            Self::DESCRIPTION_ID => Value::from(self.description.to_owned()),
            Self::POINTING_TYPE_ID => Value::from(self.pointing_type.to_owned()),
            Self::STATUS_ID => Value::from(self.status.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "process_open_sockets")]
table_properties!{
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
}}

#[cfg(feature = "process_open_sockets")]
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

#[cfg(feature = "process_open_sockets")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "pid" => Value::from(self.pid),
            "fd" => Value::from(self.fd),
            "socket" => Value::from(self.socket),
            "family" => Value::from(self.family),
            "protocol" => Value::from(self.protocol),
            "local_address" => Value::from(self.local_address.to_owned()),
            "remote_address" => Value::from(self.remote_address.to_owned()),
            "local_port" => Value::from(self.local_port),
            "remote_port" => Value::from(self.remote_port),
            "path" => Value::from(self.path.to_owned()),
            "state" => Value::from(self.state.to_owned()),
            "net_namespace" => Value::from(self.net_namespace.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::PID_ID => Value::from(self.pid.to_owned()),
            Self::FD_ID => Value::from(self.fd.to_owned()),
            Self::SOCKET_ID => Value::from(self.socket.to_owned()),
            Self::FAMILY_ID => Value::from(self.family.to_owned()),
            Self::PROTOCOL_ID => Value::from(self.protocol.to_owned()),
            Self::LOCAL_ADDRESS_ID => Value::from(self.local_address.to_owned()),
            Self::REMOTE_ADDRESS_ID => Value::from(self.remote_address.to_owned()),
            Self::LOCAL_PORT_ID => Value::from(self.local_port.to_owned()),
            Self::REMOTE_PORT_ID => Value::from(self.remote_port.to_owned()),
            Self::PATH_ID => Value::from(self.path.to_owned()),
            Self::STATE_ID => Value::from(self.state.to_owned()),
            Self::NET_NAMESPACE_ID => Value::from(self.net_namespace.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "processes")]
table_properties!{
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
}}

#[cfg(feature = "processes")]
pub trait ProcessesIface {
    fn get_wmi_process_info(&self) -> Option<String>;
}

#[cfg(feature = "processes")]
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

#[cfg(feature = "processes")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "pid" => Value::from(self.pid),
            "name" => Value::from(self.name.to_owned()),
            "path" => Value::from(self.path.to_owned()),
            "cmdline" => Value::from(self.cmdline.to_owned()),
            "state" => Value::from(self.state.to_owned()),
            "cwd" => Value::from(self.cwd.to_owned()),
            "root" => Value::from(self.root.to_owned()),
            "uid" => Value::from(self.uid),
            "gid" => Value::from(self.gid),
            "euid" => Value::from(self.euid),
            "egid" => Value::from(self.egid),
            "suid" => Value::from(self.suid),
            "sgid" => Value::from(self.sgid),
            "on_disk" => Value::from(self.on_disk),
            "wired_size" => Value::from(self.wired_size),
            "resident_size" => Value::from(self.resident_size),
            "total_size" => Value::from(self.total_size),
            "user_time" => Value::from(self.user_time),
            "system_time" => Value::from(self.system_time),
            "disk_bytes_read" => Value::from(self.disk_bytes_read),
            "disk_bytes_written" => Value::from(self.disk_bytes_written),
            "start_time" => Value::from(self.start_time),
            "parent" => Value::from(self.parent),
            "pgroup" => Value::from(self.pgroup),
            "threads" => Value::from(self.threads),
            "nice" => Value::from(self.nice),
            "is_elevated_token" => Value::from(self.is_elevated_token),
            "cgroup_namespace" => Value::from(self.cgroup_namespace.to_owned()),
            "ipc_namespace" => Value::from(self.ipc_namespace.to_owned()),
            "mnt_namespace" => Value::from(self.mnt_namespace.to_owned()),
            "net_namespace" => Value::from(self.net_namespace.to_owned()),
            "pid_namespace" => Value::from(self.pid_namespace.to_owned()),
            "user_namespace" => Value::from(self.user_namespace.to_owned()),
            "uts_namespace" => Value::from(self.uts_namespace.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::PID_ID => Value::from(self.pid),
            Self::NAME_ID => Value::from(self.name.to_owned()),
            Self::PATH_ID => Value::from(self.path.to_owned()),
            Self::CMDLINE_ID => Value::from(self.cmdline.to_owned()),
            Self::STATE_ID => Value::from(self.state.to_owned()),
            Self::CWD_ID => Value::from(self.cwd.to_owned()),
            Self::ROOT_ID => Value::from(self.root.to_owned()),
            Self::UID_ID => Value::from(self.uid),
            Self::GID_ID => Value::from(self.gid),
            Self::EUID_ID => Value::from(self.euid),
            Self::EGID_ID => Value::from(self.egid),
            Self::SUID_ID => Value::from(self.suid),
            Self::SGID_ID => Value::from(self.sgid),
            Self::ON_DISK_ID => Value::from(self.on_disk),
            Self::WIRED_SIZE_ID => Value::from(self.wired_size),
            Self::RESIDENT_SIZE_ID => Value::from(self.resident_size),
            Self::TOTAL_SIZE_ID => Value::from(self.total_size),
            Self::USER_TIME_ID => Value::from(self.user_time),
            Self::SYSTEM_TIME_ID => Value::from(self.system_time),
            Self::DISK_BYTES_READ_ID => Value::from(self.disk_bytes_read),
            Self::DISK_BYTES_WRITTEN_ID => Value::from(self.disk_bytes_written),
            Self::START_TIME_ID => Value::from(self.start_time),
            Self::PARENT_ID => Value::from(self.parent),
            Self::PGROUP_ID => Value::from(self.pgroup),
            Self::THREADS_ID => Value::from(self.threads),
            Self::NICE_ID => Value::from(self.nice),
            Self::IS_ELEVATED_TOKEN_ID => Value::from(self.is_elevated_token),
            Self::CGROUPE_NAMESPACE_ID => Value::from(self.cgroup_namespace.to_owned()),
            Self::IPC_NAMESPACE_ID => Value::from(self.ipc_namespace.to_owned()),
            Self::MNT_NAMESPACE_ID => Value::from(self.mnt_namespace.to_owned()),
            Self::NET_NAMESPACE_ID => Value::from(self.net_namespace.to_owned()),
            Self::PID_NAMESPACE_ID => Value::from(self.pid_namespace.to_owned()),
            Self::USER_NAMESPACE_ID => Value::from(self.user_namespace.to_owned()),
            Self::UTS_NAMESPACE_ID => Value::from(self.uts_namespace.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "process_memory_map")]
table_properties!{
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
}}

#[cfg(feature = "process_memory_map")]
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

#[cfg(feature = "process_memory_map")]
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

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "pid" => Value::from(self.pid),
            "start" => Value::from(self.start.to_owned()),
            "end" => Value::from(self.end.to_owned()),
            "permissions" => Value::from(self.permissions.to_owned()),
            "offset" => Value::from(self.offset),
            "device" => Value::from(self.device.to_owned()),
            "inode" => Value::from(self.inode),
            "path" => Value::from(self.path.to_owned()),
            "pseudo" => Value::from(self.pseudo),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::PID_ID => Value::from(self.pid),
            Self::START_ID => Value::from(self.start.to_owned()),
            Self::END_ID => Value::from(self.end.to_owned()),
            Self::PERMISSION_ID => Value::from(self.permissions.to_owned()),
            Self::OFFSET_ID => Value::from(self.offset),
            Self::DEVICE_ID => Value::from(self.device.to_owned()),
            Self::INODE_ID => Value::from(self.inode),
            Self::PATH_ID => Value::from(self.path.to_owned()),
            Self::PSEUDO_ID => Value::from(self.pseudo),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "process_envs")]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessEnvsRow {
    pub pid: i32,
    pub key: String,
    pub value: String,
}}

#[cfg(feature = "process_envs")]
impl ProcessEnvsRow {
    const PID_ID: u64 = 0x00000001;
    const KEY_ID: u64 = 0x00000002;
    const VALUE_ID: u64 = 0x00000004;
}

#[cfg(feature = "process_envs")]
impl Table for ProcessEnvsRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "pid",
        "key",
        "value"];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "pid" => Value::from(self.pid),
            "key" => Value::from(self.key.to_owned()),
            "value" => Value::from(self.value.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::PID_ID => Value::from(self.pid),
            Self::KEY_ID => Value::from(self.key.to_owned()),
            Self::VALUE_ID => Value::from(self.value.to_owned()),
            _ => Value::from("".to_owned())
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

#[cfg(feature = "mounts")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MountsRow {
    pub device: String,
    pub device_alias: String,
    pub path: String,
    pub device_type: String,
    pub blocks_size: i64,
    pub blocks: i64,
    pub blocks_free: i64,
    pub blocks_available: i64,
    pub inodes: i64,
    pub inodes_free: i64,
    pub flags: String,
}}

#[cfg(feature = "mounts")]
impl MountsRow {
    const DEVICE_ID: u64 = 0x00000001;
    const DEVICE_ALIAS_ID: u64 = 0x00000002;
    const PATH_ID: u64 = 0x00000004;
    const TYPE_ID: u64 = 0x00000008;
    const BLOCKS_SIZE_ID: u64 = 0x00000010;
    const BLOCKS_ID: u64 = 0x00000020;
    const BLOCKS_FREE_ID: u64 = 0x00000040;
    const BLOCKS_AVAILABLE_ID: u64 = 0x00000080;
    const INODES_ID: u64 = 0x00000100;
    const INODES_FREE_ID: u64 = 0x00000200;
    const FLAGS_ID: u64 = 0x00000400;
}

#[cfg(feature = "mounts")]
impl Table for MountsRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "device",
        "device_alias",
        "path",
        "type",
        "blocks_size",
        "blocks",
        "blocks_free",
        "blocks_available",
        "inodes",
        "inodes_free",
        "flags"
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "device" => Value::from(self.device.to_owned()),
            "device_alias" => Value::from(self.device_alias.to_owned()),
            "path" => Value::from(self.path.to_owned()),
            "type" => Value::from(self.device_type.to_owned()),
            "blocks_size" => Value::from(self.blocks_size.to_owned()),
            "blocks" => Value::from(self.blocks.to_owned()),
            "blocks_free" => Value::from(self.blocks_free.to_owned()),
            "blocks_available" => Value::from(self.blocks_available.to_owned()),
            "inodes" => Value::from(self.inodes.to_owned()),
            "inodes_free" => Value::from(self.inodes_free.to_owned()),
            "flags" => Value::from(self.flags.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::DEVICE_ID => Value::from(self.device.to_owned()),
            Self::DEVICE_ALIAS_ID => Value::from(self.device_alias.to_owned()),
            Self::PATH_ID => Value::from(self.path.to_owned()),
            Self::TYPE_ID => Value::from(self.device_type.to_owned()),
            Self::BLOCKS_SIZE_ID => Value::from(self.blocks_size.to_owned()),
            Self::BLOCKS_ID => Value::from(self.blocks.to_owned()),
            Self::BLOCKS_FREE_ID => Value::from(self.blocks_free.to_owned()),
            Self::BLOCKS_AVAILABLE_ID => Value::from(self.blocks_available.to_owned()),
            Self::INODES_ID => Value::from(self.inodes.to_owned()),
            Self::INODES_FREE_ID => Value::from(self.inodes_free.to_owned()),
            Self::FLAGS_ID => Value::from(self.flags.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "device" => Self::DEVICE_ID,
            "device_alias" => Self::DEVICE_ALIAS_ID,
            "path" => Self::PATH_ID,
            "type" => Self::TYPE_ID,
            "blocks_size" => Self::BLOCKS_SIZE_ID,
            "blocks" => Self::BLOCKS_ID,
            "blocks_free" => Self::BLOCKS_FREE_ID,
            "blocks_available" => Self::BLOCKS_AVAILABLE_ID,
            "inodes" => Self::INODES_ID,
            "inodes_free" => Self::INODES_FREE_ID,
            "flags" => Self::FLAGS_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "users")]
table_properties!{
#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub uid : i64,
    pub gid: i64,
    pub uid_signed: i64,
    pub gid_signed: i64,
    pub username: String,
    pub description: String,
    pub directory: String,
    pub shell: String,
    pub uuid: String,
    pub type_: String,
}}

#[cfg(feature = "users")]
impl Users {
    const UID_ID: u64 = 0x00000001;
    const GID_ID: u64 = 0x00000002;
    const UID_SIGNED_ID: u64 = 0x00000004;
    const GID_SIGNED_ID: u64 = 0x00000008;
    const USERNAME_ID: u64 = 0x00000010;
    const DESCRIPTION_SIGNED_ID: u64 = 0x00000020;
    const DIRECTORY_ID: u64 = 0x00000040;
    const SHELL_ID: u64 = 0x00000080;
    const UUID_SIGNED_ID: u64 = 0x00000100;
    const TYPE_ID: u64 = 0x00000200;
}

#[cfg(feature = "users")]
impl Table for Users {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "uid",
        "gid",
        "uid_signed",
        "gid_signed",
        "username",
        "description",
        "directory",
        "shell",
        "uuid",
        "type_", ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "uid" => Value::from(self.uid),
            "gid" => Value::from(self.gid),
            "uid_signed" => Value::from(self.uid_signed),
            "gid_signed" => Value::from(self.gid_signed),
            "username" => Value::from(self.username.to_owned()),
            "description" => Value::from(self.description.to_owned()),
            "directory" => Value::from(self.directory.to_owned()),
            "shell" => Value::from(self.shell.to_owned()),
            "uuid" => Value::from(self.uuid.to_owned()),
            "type_" => Value::from(self.type_.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::UID_ID => Value::from(self.uid),
            Self::GID_ID => Value::from(self.gid),
            Self::UID_SIGNED_ID => Value::from(self.uid_signed),
            Self::GID_SIGNED_ID => Value::from(self.gid_signed),
            Self::USERNAME_ID => Value::from(self.username.to_owned()),
            Self::DESCRIPTION_SIGNED_ID => Value::from(self.description.to_owned()),
            Self::DIRECTORY_ID => Value::from(self.directory.to_owned()),
            Self::SHELL_ID => Value::from(self.shell.to_owned()),
            Self::UUID_SIGNED_ID => Value::from(self.uuid.to_owned()),
            Self::TYPE_ID => Value::from(self.type_.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "uid" => Self::UID_ID,
            "gid" => Self::GID_ID,
            "uid_signed" => Self::UID_SIGNED_ID,
            "gid_signed" => Self::GID_SIGNED_ID,
            "username" => Self::USERNAME_ID,
            "description" => Self::DESCRIPTION_SIGNED_ID,
            "directory" => Self::DIRECTORY_ID,
            "shell" => Self::SHELL_ID,
            "uuid" => Self::UUID_SIGNED_ID,
            "type_" => Self::TYPE_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "logged_in_users")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggedInUsers {
    pub type_: String,
    pub user: String,
    pub tty: String,
    pub host: String,
    pub time: i64,
    pub pid: i64,
}}

#[cfg(feature = "logged_in_users")]
impl LoggedInUsers {
    const TYPE_ID: u64 = 0x00000001;
    const USER_ID: u64 = 0x00000002;
    const TTY_ID:  u64 = 0x00000004;
    const HOST_ID: u64 = 0x00000008;
    const TIME_ID: u64 = 0x00000010;
    const PID_ID: u64 = 0x00000020;
}

#[cfg(feature = "logged_in_users")]
impl Table for LoggedInUsers {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "type_",
        "user",
        "tty",
        "host",
        "time",
        "pid",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "type_" => Value::from(self.type_.to_owned()),
            "user" => Value::from(self.user.to_owned()),
            "tty" => Value::from(self.tty.to_owned()),
            "host" => Value::from(self.host.to_owned()),
            "time" => Value::from(self.time),
            "pid" => Value::from(self.pid),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::TYPE_ID => Value::from(self.type_.to_owned()),
            Self::USER_ID => Value::from(self.user.to_owned()),
            Self::TTY_ID => Value::from(self.tty.to_owned()),
            Self::HOST_ID => Value::from(self.host.to_owned()),
            Self::TIME_ID => Value::from(self.time),
            Self::PID_ID => Value::from(self.pid),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "type_" => Self::TYPE_ID,
            "user" => Self::USER_ID,
            "tty" => Self::TTY_ID,
            "host" => Self::HOST_ID,
            "time" => Self::TIME_ID,
            "pid" => Self::PID_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "groups")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupsRow {
    pub gid: i64,
    pub gid_signed: i64,
    pub groupname: String,
    pub group_sid: String,
    pub comment: String,
}}

#[cfg(feature = "groups")]
impl GroupsRow {
    const GID_ID: u64 = 0x00000001;
    const GID_SIGNED_ID: u64 = 0x00000002;
    const GROUPNAME_ID: u64 = 0x00000004;
    const GROUP_SID_ID: u64 = 0x00000008;
    const COMMENT_ID: u64 = 0x00000010;
}

#[cfg(feature = "groups")]
impl Table for GroupsRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "gid",
        "gid_signed",
        "groupname",
        "group_sid",
        "comment"
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "gid" => Value::from(self.gid),
            "gid_signed" => Value::from(self.gid_signed),
            "groupname" => Value::from(self.groupname.to_owned()),
            "group_sid" => Value::from(self.group_sid.to_owned()),
            "comment" => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::GID_ID => Value::from(self.gid),
            Self::GID_SIGNED_ID => Value::from(self.gid_signed),
            Self::GROUPNAME_ID => Value::from(self.groupname.to_owned()),
            Self::GROUP_SID_ID => Value::from(self.group_sid.to_owned()),
            Self::COMMENT_ID => Value::from(self.comment.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "gid" => Self::GID_ID,
            "gid_signed" => Self::GID_SIGNED_ID,
            "groupname" => Self::GROUPNAME_ID,
            "group_sid" => Self::GROUP_SID_ID,
            "comment" => Self::COMMENT_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "proxies")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProxiesRow {
    pub url: String,
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub interface: String,
    pub exceptions: String,
}}

#[cfg(feature = "proxies")]
impl ProxiesRow {
    const URL_ID: u64 = 0x00000001;
    const PROTOCOL_ID: u64 = 0x00000002;
    const HOST_ID: u64 = 0x00000004;
    const PORT_ID: u64 = 0x00000008;
    const INTERFACE_ID: u64 = 0x00000010;
    const EXCEPTIONS_ID: u64 = 0x00000020;
}

#[cfg(feature = "proxies")]
impl Table for ProxiesRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "url",
        "protocol",
        "host",
        "port",
        "interface",
        "exceptions",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "url" => Value::from(self.url.to_owned()),
            "protocol" => Value::from(self.protocol.to_owned()),
            "host" => Value::from(self.host.to_owned()),
            "port" => Value::from(self.port),
            "interface" => Value::from(self.interface.to_owned()),
            "exceptions" => Value::from(self.exceptions.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::URL_ID => Value::from(self.url.to_owned()),
            Self::PROTOCOL_ID => Value::from(self.protocol.to_owned()),
            Self::HOST_ID => Value::from(self.host.to_owned()),
            Self::PORT_ID => Value::from(self.port),
            Self::INTERFACE_ID => Value::from(self.interface.to_owned()),
            Self::EXCEPTIONS_ID => Value::from(self.exceptions.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "url" => Self::URL_ID,
            "protocol" => Self::PROTOCOL_ID,
            "host" => Self::HOST_ID,
            "port" => Self::PORT_ID,
            "interface" => Self::INTERFACE_ID,
            "exceptions" => Self::EXCEPTIONS_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "logon_sessions")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogonSessions {
    pub logon_id: i32,
    pub user: String,
    pub logon_domain: String,
    pub authentication_package: String,
    pub logon_type: String,
    pub session_id: i32,
    pub logon_sid: String,
    pub logon_time: i64,
    pub logon_server: String,
    pub dns_domain_name: String,
    pub upn: String,
    pub logon_script: String,
    pub profile_path: String,
    pub home_directory: String,
    pub home_directory_drive: String,
}}

#[cfg(feature = "logon_sessions")]
impl LogonSessions {
    const LOGON_ID_ID: u64 = 0x00000001;
    const USER_ID: u64 = 0x00000002;
    const LOGON_DOMAIN_ID: u64 = 0x00000004;
    const AUTHENTICATION_PACKAGE_ID: u64 = 0x00000008;
    const LOGON_TYPE_ID: u64 = 0x00000010;
    const SESSION_ID_ID: u64 = 0x00000020;
    const LOGON_SID_ID: u64 = 0x00000040;
    const LOGON_TIME_ID: u64 = 0x00000080;
    const LOGON_SERVER_ID: u64 = 0x00000100;
    const DNS_DOMAIN_NAME_ID: u64 = 0x00000200;
    const UPN_ID: u64 = 0x00000400;
    const LOGON_SCRIPT_ID: u64 = 0x00000800;
    const PROFILE_PATH_ID: u64 = 0x00001000;
    const HOME_DIRECTORY_ID: u64 = 0x00002000;
    const HOME_DIRECTORY_DRIVE_ID: u64 = 0x00004000;
}

#[cfg(feature = "logon_sessions")]
impl Table for LogonSessions {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "logon_id",
        "user",
        "logon_domain",
        "authentication_package",
        "logon_type",
        "session_id",
        "logon_sid",
        "logon_time",
        "logon_server",
        "dns_domain_name",
        "upn",
        "logon_script",
        "profile_path",
        "home_directory",
        "home_directory_drive",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "logon_id" => Value::from(self.logon_id),
            "user" => Value::from(self.user.to_owned()),
            "logon_domain" => Value::from(self.logon_domain.to_owned()),
            "authentication_package" => Value::from(self.authentication_package.to_owned()),
            "logon_type" => Value::from(self.logon_type.to_owned()),
            "session_id" => Value::from(self.session_id),
            "logon_sid" => Value::from(self.logon_sid.to_owned()),
            "logon_time" => Value::from(self.logon_time),
            "logon_server" => Value::from(self.logon_server.to_owned()),
            "dns_domain_name" => Value::from(self.dns_domain_name.to_owned()),
            "upn" => Value::from(self.upn.to_owned()),
            "logon_script" => Value::from(self.logon_script.to_owned()),
            "profile_path" => Value::from(self.profile_path.to_owned()),
            "home_directory" => Value::from(self.home_directory.to_owned()),
            "home_directory_drive" => Value::from(self.home_directory_drive.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::LOGON_ID_ID => Value::from(self.logon_id),
            Self::USER_ID => Value::from(self.user.to_owned()),
            Self::LOGON_DOMAIN_ID => Value::from(self.logon_domain.to_owned()),
            Self::AUTHENTICATION_PACKAGE_ID => Value::from(self.authentication_package.to_owned()),
            Self::LOGON_TYPE_ID => Value::from(self.logon_type.to_owned()),
            Self::SESSION_ID_ID => Value::from(self.session_id),
            Self::LOGON_SID_ID => Value::from(self.logon_sid.to_owned()),
            Self::LOGON_TIME_ID => Value::from(self.logon_time),
            Self::LOGON_SERVER_ID => Value::from(self.logon_server.to_owned()),
            Self::DNS_DOMAIN_NAME_ID => Value::from(self.dns_domain_name.to_owned()),
            Self::UPN_ID => Value::from(self.upn.to_owned()),
            Self::LOGON_SCRIPT_ID => Value::from(self.logon_script.to_owned()),
            Self::PROFILE_PATH_ID => Value::from(self.profile_path.to_owned()),
            Self::HOME_DIRECTORY_ID => Value::from(self.home_directory.to_owned()),
            Self::HOME_DIRECTORY_DRIVE_ID => Value::from(self.home_directory_drive.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "logon_id" => Self::LOGON_ID_ID,
            "user" => Self::USER_ID,
            "logon_domain" => Self::LOGON_DOMAIN_ID,
            "authentication_package" => Self::AUTHENTICATION_PACKAGE_ID,
            "logon_type" => Self::LOGON_TYPE_ID,
            "session_id" => Self::SESSION_ID_ID,
            "logon_sid" => Self::LOGON_SID_ID,
            "logon_time" => Self::LOGON_TIME_ID,
            "logon_server" => Self::LOGON_SERVER_ID,
            "dns_domain_name" => Self::DNS_DOMAIN_NAME_ID,
            "upn" => Self::UPN_ID,
            "logon_script" => Self::LOGON_SCRIPT_ID,
            "profile_path" => Self::PROFILE_PATH_ID,
            "home_directory" => Self::HOME_DIRECTORY_ID,
            "home_directory_drive" => Self::HOME_DIRECTORY_DRIVE_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "launchd")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchdRow {
    pub path: String,
    pub name: String,
    pub label: String,
    pub program: String,
    pub run_at_load: String,
    pub keep_alive: String,
    pub on_demand: String,
    pub disabled: String,
    pub username: String,
    pub groupname: String,
    pub stdout_path: String,
    pub stderr_path: String,
    pub start_interval: String,
    pub program_arguments: String,
    pub watch_paths: String,
    pub queue_directories: String,
    pub inetd_compatibility: String,
    pub start_on_mount: String,
    pub root_directory: String,
    pub working_directory: String,
    pub process_type: String,

}}

#[cfg(feature = "launchd")]
impl LaunchdRow {
    const PATH_ID: u64 = 0x00000001;
    const NAME_ID: u64 = 0x00000002;
    const LABEL_ID: u64 = 0x00000004;
    const PROGRAM_ID: u64 = 0x00000008;
    const RUN_AT_LOAD_ID: u64 = 0x00000010;
    const KEEP_ALIVE_ID: u64 = 0x00000020;
    const ON_DEMAND_ID: u64 = 0x00000040;
    const DISABLED_ID: u64 = 0x00000080;
    const USERNAME_ID: u64 = 0x00000100;
    const GROUPNAME_ID: u64 = 0x00000200;
    const STDOUT_PATH_ID: u64 = 0x00000400;
    const STDERR_PATH_ID: u64 = 0x00000800;
    const START_INTERVAL_ID: u64 = 0x00001000;
    const PROGRAM_ARGUMENTS_ID: u64 = 0x00002000;
    const WATCH_PATHS_ID: u64 = 0x00004000;
    const QUEUE_DIRECTORIES_ID: u64 = 0x00008000;
    const INETD_COMPATIBILITY_ID: u64 = 0x00010000;
    const START_ON_MOUNT_ID: u64 = 0x00020000;
    const ROOT_DIRECTORY_ID: u64 = 0x00040000;
    const WORK_DIRECTORY_ID: u64 = 0x00080000;
    const PROCESS_TYPE_ID: u64 = 0x00100000;
}

#[cfg(feature = "launchd")]
impl Table for LaunchdRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "path",
        "name",
        "label",
        "program",
        "run_at_load",
        "keep_alive",
        "on_demand",
        "disabled",
        "username",
        "groupname",
        "stdout_path",
        "stderr_path",
        "start_interval",
        "program_arguments",
        "watch_paths",
        "queue_directories",
        "inetd_compatibility",
        "start_on_mount",
        "root_directory",
        "working_directory",
        "process_type",
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "path"=> Value::from(self.path.to_owned()),
            "name"=> Value::from(self.name.to_owned()),
            "label"=> Value::from(self.label.to_owned()),
            "program"=> Value::from(self.program.to_owned()),
            "run_at_load"=> Value::from(self.run_at_load.to_owned()),
            "keep_alive"=> Value::from(self.keep_alive.to_owned()),
            "on_demand"=> Value::from(self.on_demand.to_owned()),
            "disabled"=> Value::from(self.disabled.to_owned()),
            "username"=> Value::from(self.username.to_owned()),
            "groupname"=> Value::from(self.groupname.to_owned()),
            "stdout_path"=> Value::from(self.stdout_path.to_owned()),
            "stderr_path"=> Value::from(self.stderr_path.to_owned()),
            "start_interval" => Value::from(self.start_interval.to_owned()),
            "program_arguments"=> Value::from(self.program_arguments.to_owned()),
            "watch_paths"=> Value::from(self.watch_paths.to_owned()),
            "queue_directories"=> Value::from(self.queue_directories.to_owned()),
            "inetd_compatibility"=> Value::from(self.inetd_compatibility.to_owned()),
            "start_on_mount"=> Value::from(self.start_on_mount.to_owned()),
            "root_directory"=> Value::from(self.root_directory.to_owned()),
            "working_directory"=> Value::from(self.working_directory.to_owned()),
            "process_type"=> Value::from(self.process_type.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::PATH_ID=> Value::from(self.path.to_owned()),
            Self::NAME_ID=> Value::from(self.name.to_owned()),
            Self::LABEL_ID=> Value::from(self.label.to_owned()),
            Self::PROGRAM_ID=> Value::from(self.program.to_owned()),
            Self::RUN_AT_LOAD_ID=> Value::from(self.run_at_load.to_owned()),
            Self::KEEP_ALIVE_ID=> Value::from(self.keep_alive.to_owned()),
            Self::ON_DEMAND_ID=> Value::from(self.on_demand.to_owned()),
            Self::DISABLED_ID=> Value::from(self.disabled.to_owned()),
            Self::USERNAME_ID=> Value::from(self.username.to_owned()),
            Self::GROUPNAME_ID=> Value::from(self.groupname.to_owned()),
            Self::STDOUT_PATH_ID=> Value::from(self.stdout_path.to_owned()),
            Self::STDERR_PATH_ID=> Value::from(self.stderr_path.to_owned()),
            Self::START_INTERVAL_ID=> Value::from(self.start_interval.to_owned()),
            Self::PROGRAM_ARGUMENTS_ID=> Value::from(self.program_arguments.to_owned()),
            Self::WATCH_PATHS_ID=> Value::from(self.watch_paths.to_owned()),
            Self::QUEUE_DIRECTORIES_ID=> Value::from(self.queue_directories.to_owned()),
            Self::INETD_COMPATIBILITY_ID=> Value::from(self.inetd_compatibility.to_owned()),
            Self::START_ON_MOUNT_ID=> Value::from(self.start_on_mount.to_owned()),
            Self::ROOT_DIRECTORY_ID=> Value::from(self.root_directory.to_owned()),
            Self::WORK_DIRECTORY_ID=> Value::from(self.working_directory.to_owned()),
            Self::PROCESS_TYPE_ID=> Value::from(self.process_type.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "path" => Self::PATH_ID,
            "name" => Self::NAME_ID,
            "label"=> Self::LABEL_ID,
            "program"=> Self::PROGRAM_ID,
            "run_at_load"=> Self::RUN_AT_LOAD_ID,
            "keep_alive"=> Self::KEEP_ALIVE_ID,
            "on_demand"=> Self::ON_DEMAND_ID,
            "disabled"=> Self::DISABLED_ID,
            "username"=> Self::USERNAME_ID,
            "groupname"=> Self::GROUPNAME_ID,
            "stdout_path"=> Self::STDOUT_PATH_ID,
            "stderr_path"=> Self::STDERR_PATH_ID,
            "start_interval"=> Self::START_INTERVAL_ID,
            "program_arguments"=> Self::PROGRAM_ARGUMENTS_ID,
            "watch_paths"=> Self::WATCH_PATHS_ID,
            "queue_directories"=> Self::QUEUE_DIRECTORIES_ID,
            "inetd_compatibility"=> Self::INETD_COMPATIBILITY_ID,
            "start_on_mount"=> Self::START_ON_MOUNT_ID,
            "root_directory"=> Self::ROOT_DIRECTORY_ID,
            "working_directory"=> Self::WORK_DIRECTORY_ID,
            "process_type"=> Self::PROCESS_TYPE_ID,
            _ => 0
        }
    }
}

#[cfg(feature = "launchd_overrides")]
table_properties!{
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchdOverridesRow {
    pub label   : String,
    pub key     : String,
    pub value   : String,
    pub uid     : i64,
    pub path    : String,
}}

#[cfg(feature = "launchd_overrides")]
impl LaunchdOverridesRow {
    const LABEL_ID  : u64 = 0x00000001;
    const KEY_ID    : u64 = 0x00000002;
    const VALUE_ID  : u64 = 0x00000004;
    const UID_ID    : u64 = 0x00000008;
    const PATH_ID   : u64 = 0x00000010;
}

#[cfg(feature = "launchd_overrides")]
impl Table for LaunchdOverridesRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "label" ,
        "key"   ,
        "value" ,
        "uid"   ,
        "path"  ,
    ];

    fn get_by_name(&self, _name: &str) -> Value {
        match _name {
            "label" => Value::from(self.label.to_owned()),
            "key"   => Value::from(self.key.to_owned()),
            "value" => Value::from(self.value.to_owned()),
            "uid"   => Value::from(self.uid.to_owned()),
            "path"  => Value::from(self.path.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_by_id(&self, _id: u64) -> Value {
        match _id {
            Self::LABEL_ID => Value::from(self.label.to_owned()),
            Self::KEY_ID   => Value::from(self.key.to_owned()),
            Self::VALUE_ID => Value::from(self.value.to_owned()),
            Self::UID_ID   => Value::from(self.uid.to_owned()),
            Self::PATH_ID  => Value::from(self.path.to_owned()),
            _ => Value::from("".to_owned())
        }
    }

    fn get_id(&self, _name: &str) -> u64 {
        match _name {
            "label" =>  Self::LABEL_ID ,
            "key"   =>  Self::KEY_ID   ,
            "value" =>  Self::VALUE_ID ,
            "uid"   =>  Self::UID_ID   ,
            "path"  =>  Self::PATH_ID  ,
            _ => 0
        }
    }
}

pub fn get_table_list() -> Vec<String> {
    vec![
        #[cfg(feature = "etc_hosts")]
            "etc_hosts".to_string(),
        #[cfg(feature = "etc_protocols")]
            "etc_protocols".to_string(),
        #[cfg(feature = "etc_services")]
            "etc_services".to_string(),
        #[cfg(feature = "system_info")]
            "system_info".to_string(),
        #[cfg(feature = "os_version")]
            "os_version".to_string(),
        #[cfg(feature = "logical_drives")]
            "logical_drives".to_string(),
        #[cfg(feature = "uptime")]
            "uptime".to_string(),
        #[cfg(feature = "processes")]
            "processes".to_string(),
        #[cfg(feature = "interface_address")]
            "interface_address".to_string(),
        #[cfg(feature = "interface_details")]
            "interface_details".to_string(),
        #[cfg(feature = "process_open_sockets")]
            "process_open_sockets".to_string(),
        #[cfg(feature = "process_memory_map")]
            "process_memory_map".to_string(),
        #[cfg(feature = "products")]
            "products".to_string(),
        #[cfg(feature = "users")]
            "users".to_string(),
        #[cfg(feature = "logged_in_users")]
            "logged_in_users".to_string(),
        #[cfg(feature = "logon_sessions")]
            "logon_sessions".to_string(),
        #[cfg(feature = "groups")]
            "groups".to_string(),
        #[cfg(feature = "wmi_computer_info")]
            "wmi_computer_info".to_string(),
        #[cfg(feature = "wmi_os_version")]
            "wmi_os_version".to_string(),
        #[cfg(feature = "wmi_printers")]
            "wmi_printers".to_string(),
        #[cfg(feature = "wmi_services")]
            "wmi_services".to_string(),
        #[cfg(feature = "wmi_hotfixes")]
            "wmi_hotfixes".to_string(),
        #[cfg(feature = "wmi_shares")]
            "wmi_shares".to_string(),
        #[cfg(feature = "wmi_network_adapters")]
            "wmi_network_adapters".to_string(),
        #[cfg(feature = "wmi_local_accounts")]
            "wmi_local_accounts".to_string(),
        #[cfg(feature = "wmi_bios")]
            "wmi_bios".to_string(),
        #[cfg(feature = "wmi_motherboard")]
            "wmi_motherboard".to_string(),
        #[cfg(feature = "wmi_processor")]
            "wmi_processor".to_string(),
        #[cfg(feature = "wmi_physical_memory")]
            "wmi_physical_memory".to_string(),
        #[cfg(feature = "wmi_sound")]
            "wmi_sound".to_string(),
        #[cfg(feature = "wmi_video")]
            "wmi_video".to_string(),
        #[cfg(feature = "wmi_monitors")]
            "wmi_monitors".to_string(),
        #[cfg(feature = "wmi_keyboard")]
            "wmi_keyboard".to_string(),
        #[cfg(feature = "wmi_pointing_device")]
            "wmi_pointing_device".to_string(),
        #[cfg(feature = "process_envs")]
            "process_envs".to_string(),
        #[cfg(feature = "mounts")]
            "mounts".to_string(),
        #[cfg(feature = "proxies")]
            "proxies".to_string(),
        #[cfg(feature = "launchd")]
            "launchd".to_string(),
        #[cfg(feature = "launchd_overrides")]
            "launchd_overrides".to_string(),
        #[cfg(test)]
            "Dummy".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_by_name(){
        let table = Dummy {
            a: 25,
            b: 30,
        };
        assert_eq!(table.get_by_name("a"), Value::from(25));
        assert_eq!(table.get_by_name("b"), Value::from(30));
        assert_ne!(table.get_by_name("b"), Value::from(35));
        assert_eq!(table.get_by_name("c"), Value::from("".to_owned()));
    }
    #[test]
    fn test_get_by_id(){
        let table = Dummy {
            a: 25,
            b: 30,
        };
        assert_eq!(table.get_by_id(1), Value::from(25));
        assert_eq!(table.get_by_id(2), Value::from(30));
        assert_ne!(table.get_by_id(2), Value::from(35));
        assert_eq!(table.get_by_id(0), Value::from("".to_owned()));
    }
    #[test]
    fn test_get_id(){
        let table = Dummy {
            a: 25,
            b: 30,
        };
        assert_eq!(table.get_id("a"), 1);
        assert_eq!(table.get_id("b"), 2);
        assert_ne!(table.get_id("c"), 2);
        assert_eq!(table.get_id("c"), 0);
    }
    #[test]
    fn test_table_properties(){
        assert_eq!(Dummy::get_columns_name(), vec!["a", "b"]);
        assert_eq!(Dummy::get_fields_type(), vec!["u32", "i32"]);
        assert_eq!(Dummy::get_columns_type(), vec!["\" INTEGER", "\" INTEGER"]);
    }
}