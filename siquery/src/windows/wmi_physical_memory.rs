use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiMemory,WmiMemoryIface};
use crate::utils;

pub struct Reader {}
impl WmiMemoryIface for Reader {
    fn get_wmi_physical_memory(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["memorychip", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiMemory {
    pub(crate) fn new() -> WmiMemory {
        WmiMemory {
            name: String::new(),
            bank_label: String::new(),
            capacity: String::new(),
            description: String::new(),
            device_locator: String::new(),
            form_factor: 0,
            interleave_data_depth: 0,
            interleave_position: 0,
            manufacturer: String::new(),
            memory_type: 0,
            serial_number: String::new(),
            speed: 0,
        }
    }

    pub fn get_specific_ex(reader: &WmiMemoryIface) -> Vec<WmiMemory> {

        let mut physical_memories: Vec<WmiMemory> = Vec::new();

        if let Some(physical_memory_info) = reader.get_wmi_physical_memory() {
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
                        physical_memory.form_factor = v.parse::<u16>().unwrap_or(0);
                    },
                    "InterleaveDataDepth" => {
                        physical_memory.interleave_data_depth = v.parse::<u16>().unwrap_or(0);
                    },
                    "InterleavePosition" => {
                        physical_memory.interleave_position = v.parse::<u32>().unwrap_or(0);
                    },
                    "Manufacturer" => {
                        physical_memory.manufacturer = v;
                    },
                    "MemoryType" => {
                        physical_memory.memory_type = v.parse::<u16>().unwrap_or(0);
                    },
                    "SerialNumber" => {
                        physical_memory.serial_number = v;
                    },
                    "Speed" => {
                        physical_memory.speed = v.parse::<u32>().unwrap_or(0);
                    },
                    _ => ()
                }
            }
        }

        physical_memories
    }

    pub(crate) fn get_specific() -> Vec<WmiMemory> {
        let reader: Box<WmiMemoryIface> = Box::new(Reader{});
        let out = WmiMemory::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiMemoryIface for Test {
        fn get_wmi_physical_memory(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-physical-memory.txt")))
        }
    }
    #[test]
    fn test_wmi_physical_memory () {
        let reader: Box<WmiMemoryIface> = Box::new(Test{});
        let physical_memory = &WmiMemory::get_specific_ex(reader.borrow())[0];
        assert_eq!(physical_memory.name, "Physical Memory");
        assert_eq!(physical_memory.bank_label, "BANK 0");
        assert_eq!(physical_memory.capacity, "17179869184 bytes");
        assert_eq!(physical_memory.description, "Physical Memory");
        assert_eq!(physical_memory.device_locator, "DIMM A");
        assert_eq!(physical_memory.form_factor, 12);
        assert_eq!(physical_memory.interleave_data_depth, 0);
        assert_eq!(physical_memory.interleave_position, 0);
        assert_eq!(physical_memory.manufacturer, "Fabrikam, Inc.");
        assert_eq!(physical_memory.memory_type, 0);
        assert_eq!(physical_memory.serial_number, "91A92B93C");
        assert_eq!(physical_memory.speed, 2400);
    }
}
