use tables::{SystemInfoData,SystemInfoDataIface};
use linux::SystemReaderInterface;

struct CpuInfo {
    cpu_brand: String,
    cpu_logical_cores: u32
}

impl SystemInfoData {
    pub(crate) fn new() -> SystemInfoData {
        SystemInfoData {
            computer_name: String::new(),
            cpu_brand: String::new(),
            cpu_logical_cores: 0,
            physical_memory: 0
        }
    }

    pub(crate) fn get_specific_ex(system_reader: &SystemReaderInterface) -> Vec<SystemInfoData> {
        let mut output : Vec<SystemInfoData> = Vec::new();
        let mut system_info = SystemInfoData::new();
        system_info.computer_name = system_reader.hostname().unwrap_or_else(|| String::from(""));

        system_info.physical_memory = match system_reader.meminfo() {
            Some(s) => {
                let n = s.split('\n').find(|line| line.starts_with("MemTotal"))
                         .and_then(|line| line.split(':').last())
                         .and_then(|v| v.trim_left_matches(' ').split(' ').next())
                         .and_then(|v| v.parse::<u64>().ok());
                n.unwrap_or(0) * 1024
            }

            None => 0
        };

        if let Some(cpu_info) = system_info.get_cpu_info(system_reader) {
            system_info.cpu_brand = cpu_info.cpu_brand;
            system_info.cpu_logical_cores = cpu_info.cpu_logical_cores;
        };
        output.push(system_info);
        output
    }

    fn get_cpu_info(&mut self, system_reader: &SystemReaderInterface) -> Option<CpuInfo> {
        let s = system_reader.cpuinfo()?;
        let model_name = s.split('\n').find(|line| line.starts_with("model name"))
                          .and_then(|line| line.split(':').last())
                          .and_then(|val| Some(val.trim()));
        let model_name = String::from(model_name.unwrap_or(""));

        Some(CpuInfo {cpu_brand: model_name, cpu_logical_cores: system_reader.cpu_count()})
    }

    pub(crate) fn get_specific() -> Vec<SystemInfoData> {
        let reader: Box<SystemInfoDataIface> = Box::new(Reader{});
        let out = SystemInfoData::get_specific_ex(reader.borrow());
        out
    }
}

