use tables::WmiProcessor;
use utils;
use windows::SystemReaderInterface;

impl WmiProcessor {
    pub(crate) fn get_processor_info(system_reader: &SystemReaderInterface) -> WmiProcessor {

        let mut processor = WmiProcessor {
            address_width: String::new(),
            cpu_satus: String::new(),
            current_clock_speed: String::new(),
            current_voltage: String::new(),
            description: String::new(),
            external_clock: String::new(),
            hyper_threading_enabled: "FALSE".to_string(),
            l2_cache_size: String::new(),
            l2_cache_speed: String::new(),
            l3_cache_size: String::new(),
            l3_cache_speed: String::new(),
            manufacturer: String::new(),
            max_clock_speed: String::new(),
            name: String::new(),
            number_of_cores: String::new(),
            number_of_logical_processors: String::new(),
            socket_designation: String::new(),
        };

        let mut nbr_cores = 0;
        let mut nbr_logical_cpu = 0;

        if let Some(processor_info) = system_reader.get_wmi_processor_info() {
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
                        processor.address_width = v;
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
                        processor.current_clock_speed = v;
                    },
                    "CurrentVoltage" => {
                        processor.current_voltage = v;
                    },
                    "Description" => {
                        processor.description = v;
                    },
                    "ExtClock" => {
                        processor.external_clock = v;
                    },
                    "L2CacheSize" => {
                        processor.l2_cache_size = v;
                    },
                    "L2CacheSpeed" => {
                        processor.l2_cache_speed = v;
                    },
                    "L3CacheSize" => {
                        processor.l3_cache_size = v;
                    },
                    "L3CacheSpeed" => {
                        processor.l3_cache_speed = v;
                    },
                    "Manufacturer" => {
                        processor.manufacturer = v;
                    },
                    "MaxClockSpeed" => {
                        processor.max_clock_speed = v;
                    },
                    "Name" => {
                        processor.name = v;
                    }
                    "NumberOfCores" => {
                        nbr_cores = v.parse::<u64>().unwrap_or(0);
                        processor.number_of_cores = v;
                    },
                    "NumberOfLogicalProcessors" => {
                        nbr_logical_cpu = v.parse::<u64>().unwrap_or(0);
                        processor.number_of_logical_processors = v;
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

        processor
    }
}