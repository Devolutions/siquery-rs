use tables::SystemInfoData;
use macos::SystemReaderInterface;

impl SystemInfoData {
    pub(crate) fn new() -> SystemInfoData {
        SystemInfoData {
            computer_name: String::new(),
            cpu_brand: String::new(),
            cpu_logical_cores: 0,
            physical_memory: 0
        }
    }

    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<SystemInfoData> {
        let mut output : Vec<SystemInfoData> = Vec::new();
        let mut system_info = SystemInfoData::new();
        system_info.computer_name = system_reader.hostname().unwrap_or(String::from(""));
        output.push(system_info);
        output
    }
}