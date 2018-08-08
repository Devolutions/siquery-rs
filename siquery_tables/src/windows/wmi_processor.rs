use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiProcessor,WmiProcessorIface};
use utils;

pub struct Reader {}
impl WmiProcessorIface for Reader {
    fn get_wmi_processor_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["cpu", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiProcessor {
    pub fn get_specific_ex(reader: &WmiProcessorIface) -> Vec<WmiProcessor> {
        let mut output : Vec<WmiProcessor> = Vec::new();
        let mut processor = WmiProcessor {
            address_width: 0,
            cpu_satus: String::new(),
            current_clock_speed: 0,
            current_voltage: 0,
            description: String::new(),
            external_clock: 0,
            hyper_threading_enabled: "FALSE".to_string(),
            l2_cache_size: 0,
            l2_cache_speed: 0,
            l3_cache_size: 0,
            l3_cache_speed: 0,
            manufacturer: String::new(),
            max_clock_speed: 0,
            name: String::new(),
            number_of_cores: 0,
            number_of_logical_processors: 0,
            socket_designation: String::new(),
        };

        let mut nbr_cores = 0;
        let mut nbr_logical_cpu = 0;

        if let Some(processor_info) = reader.get_wmi_processor_info() {
            let lines = processor_info.split('\n');

            for line in lines {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "AddressWidth" => {
                        processor.address_width = v.parse::<u16>().unwrap_or(0);
                    },
                    "CpuStatus" => {
                        //https://msdn.microsoft.com/en-us/library/aa394373(v=vs.85).aspx
                        match v.as_str(){
                            "0"=> {
                                processor.cpu_satus = "Unknown".to_string();
                            },
                            "1"=> {
                                processor.cpu_satus = "CPU Enabled".to_string();
                            },
                            "2"=> {
                                processor.cpu_satus = "CPU Disabled by User via BIOS Setup".to_string();
                            },
                            "3"=> {
                                processor.cpu_satus = "CPU Disabled By BIOS (POST Error)".to_string();
                            },
                            "4"=> {
                                processor.cpu_satus = "CPU is Idle".to_string();
                            },
                            "5"=> {
                                processor.cpu_satus = "Reserved".to_string();
                            },
                            "6"=> {
                                processor.cpu_satus = "Reserved".to_string();
                            },
                            "7"=> {
                                processor.cpu_satus = "Other".to_string();
                            },
                            _=>()
                        }
                    },
                    "CurrentClockSpeed" => {
                        processor.current_clock_speed = v.parse::<u32>().unwrap_or(0);
                    },
                    "CurrentVoltage" => {
                        processor.current_voltage = v.parse::<u16>().unwrap_or(0);
                    },
                    "Description" => {
                        processor.description = v;
                    },
                    "ExtClock" => {
                        processor.external_clock = v.parse::<u32>().unwrap_or(0);
                    },
                    "L2CacheSize" => {
                        processor.l2_cache_size = v.parse::<u32>().unwrap_or(0);
                    },
                    "L2CacheSpeed" => {
                        processor.l2_cache_speed = v.parse::<u32>().unwrap_or(0);
                    },
                    "L3CacheSize" => {
                        processor.l3_cache_size = v.parse::<u32>().unwrap_or(0);
                    },
                    "L3CacheSpeed" => {
                        processor.l3_cache_speed = v.parse::<u32>().unwrap_or(0);
                    },
                    "Manufacturer" => {
                        processor.manufacturer = v;
                    },
                    "MaxClockSpeed" => {
                        processor.max_clock_speed = v.parse::<u32>().unwrap_or(0);
                    },
                    "Name" => {
                        processor.name = v;
                    }
                    "NumberOfCores" => {
                        nbr_cores = v.parse::<u32>().unwrap_or(0);
                        processor.number_of_cores = nbr_cores;
                    },
                    "NumberOfLogicalProcessors" => {
                        nbr_logical_cpu = v.parse::<u32>().unwrap_or(0);
                        processor.number_of_logical_processors = nbr_logical_cpu;
                    },
                    "SocketDesignation" => {
                        processor.socket_designation = v;
                    },
                    _ => ()
                }
            }
        }

        if nbr_logical_cpu > nbr_cores {
            processor.hyper_threading_enabled = "TRUE".to_string();
        }

        output.push(processor);
        output
    }

    pub(crate) fn get_specific() -> Vec<WmiProcessor> {
        let reader: Box<WmiProcessorIface> = Box::new(Reader{});
        let out = WmiProcessor::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiProcessorIface for Test {
        fn get_wmi_processor_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-processor.txt")))
        }
    }
    #[test]
    fn test_wmi_processor () {
        let reader: Box<WmiProcessorIface> = Box::new(Test{});
        let processor_info = &WmiProcessor::get_specific_ex(reader.borrow())[0];
        assert_eq!(processor_info.name, "Fabrikam Core(TM) i7-7500U CPU @ 2.70GHz");
        assert_eq!(processor_info.address_width, 64);
        assert_eq!(processor_info.cpu_satus, "CPU Enabled");
        assert_eq!(processor_info.current_clock_speed, 1600);
        assert_eq!(processor_info.current_voltage, 11);
        assert_eq!(processor_info.description, "Fabrikam Family 6 Model 142 Stepping 9");
        assert_eq!(processor_info.external_clock, 100);
        assert_eq!(processor_info.hyper_threading_enabled, "FALSE");
        assert_eq!(processor_info.l2_cache_size, 512);
        assert_eq!(processor_info.l2_cache_speed, 0);
        assert_eq!(processor_info.l3_cache_size, 4096);
        assert_eq!(processor_info.l3_cache_speed, 0);
        assert_eq!(processor_info.manufacturer, "Fabrikam, Inc.");
        assert_eq!(processor_info.max_clock_speed, 2901);
        assert_eq!(processor_info.number_of_cores, 2);
        assert_eq!(processor_info.number_of_logical_processors, 2);
        assert_eq!(processor_info.socket_designation, "U4E2");
    }
}