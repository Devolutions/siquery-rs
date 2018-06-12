use tables::WmiMemory;
use utils;
use windows::SystemReaderInterface;

impl WmiMemory {
    pub(crate) fn new() -> WmiMemory {
        WmiMemory {
            name: String::new(),
            bank_label: String::new(),
            capacity: String::new(),
            description: String::new(),
            device_locator: String::new(),
            form_factor: String::new(),
            interleave_data_depth: String::new(),
            interleave_position: String::new(),
            manufacturer: String::new(),
            memory_type: String::new(),
            serial_number: String::new(),
            speed: String::new(),
        }
    }

    pub(crate) fn get_physical_memory_info(system_reader: &SystemReaderInterface) -> Vec<WmiMemory> {

        let mut physical_memories: Vec<WmiMemory> = Vec::new();

        if let Some(physical_memory_info) = system_reader.get_wmi_physical_memory() {
            let mut physical_memory = WmiMemory::new();
            let lines = physical_memory_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if physical_memory.name != "" {
                        physical_memories.push(physical_memory);
                    }
                    physical_memory = WmiMemory::new();
                }

                let v: Vec<_> = line.splitn(2, '=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Name" => {
                        physical_memory.name = v;
                    },
                    "BankLabel" => {
                        physical_memory.bank_label = v;
                    },
                    "Capacity" => {
                        physical_memory.capacity = v;
                    },
                    "Description" => {
                        physical_memory.description = v;
                    },
                    "DeviceLocator" => {
                        physical_memory.device_locator = v;
                    },
                    "FormFactor" => {
                        physical_memory.form_factor = v;
                    },
                    "InterleaveDataDepth" => {
                        physical_memory.interleave_data_depth = v;
                    },
                    "InterleavePosition" => {
                        physical_memory.interleave_position = v;
                    },
                    "Manufacturer" => {
                        physical_memory.manufacturer = v;
                    },
                    "MemoryType" => {
                        physical_memory.memory_type = v;
                    },
                    "SerialNumber" => {
                        physical_memory.serial_number = v;
                    },
                    "Speed" => {
                        physical_memory.speed = v;
                    },
                    _ => ()
                }
            }
        }

        physical_memories
    }
}