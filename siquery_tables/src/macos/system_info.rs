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

    pub(crate) fn update(&mut self, system_reader: &SystemReaderInterface) {
        self.computer_name = system_reader.hostname().unwrap_or(String::from(""));
    }
}