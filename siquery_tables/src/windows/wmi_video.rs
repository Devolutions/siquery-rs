use tables::WmiVideo;
use utils;
use windows::SystemReaderInterface;

impl WmiVideo {
    pub(crate) fn new() -> WmiVideo {
        WmiVideo {
            name: String::new(),
            adapter_compatibility: String::new(),
            adapter_dac_type: String::new(),
            adapter_ram: 0.0,
            availability: String::new(),
            driver_version: String::new(),
            installed_display_driver: Vec::new(),
            refresh_rate: String::new(),
            screen_info: String::new(),
            status: String::new(),
            video_architecture: String::new(),
            video_memory_type: String::new(),
        }
    }

    pub(crate) fn get_video_info(system_reader: &SystemReaderInterface) -> Vec<WmiVideo> {

        let mut videos: Vec<WmiVideo> = Vec::new();

        if let Some(video_info) = system_reader.get_wmi_video_info() {
            let mut video = WmiVideo::new();
            let lines = video_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if video.name != "" {
                        videos.push(video);
                    }
                    video = WmiVideo::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "AdapterCompatibility" => {
                        video.adapter_compatibility = v;
                    },
                    "AdapterDACType" => {
                        video.adapter_dac_type = v;
                    },
                    "AdapterRAM" => {
                        // convert bytes to GB
                        let mut ram = v.parse::<f32>().unwrap_or(0.0) / 1073741824.0;
                        video.adapter_ram = ram;
                    },
                    "Availability" => {
                        match v.as_str() {
                            "1" => {
                                video.availability = "Other".to_string();
                            },
                            "2" => {
                                video.availability = "Unknown".to_string();
                            },
                            "3" => {
                                video.availability = "Running or Full Power".to_string();
                            },
                            "4" => {
                                video.availability = "Warning".to_string();
                            },
                            "5" => {
                                video.availability = "In Test".to_string();
                            },
                            "6" => {
                                video.availability = "Not Applicable".to_string();
                            },
                            "7" => {
                                video.availability = "Power Off".to_string();
                            },
                            "8" => {
                                video.availability = "Off Line".to_string();
                            },
                            "9" => {
                                video.availability = "Off Duty".to_string();
                            },
                            "10" => {
                                video.availability = "Degraded".to_string();
                            },
                            "11" => {
                                video.availability = "Not Installed".to_string();
                            },
                            "12" => {
                                video.availability = "Install Error".to_string();
                            },
                            "13" => {
                                video.availability = "Power Save - Unknown".to_string();
                            },
                            "14" => {
                                video.availability = "Power Save - Low Power Mode".to_string();
                            },
                            "15" => {
                                video.availability = "Power Save - Standby".to_string();
                            },
                            "16" => {
                                video.availability = "Power Cycle".to_string();
                            },
                            "17" => {
                                video.availability = "Power Save - Warning".to_string();
                            },
                            "18" => {
                                video.availability = "Paused ".to_string();
                            },
                            "19" => {
                                video.availability = "Not Ready".to_string();
                            },

                            "20" => {
                                video.availability = "Not Configured".to_string();
                            },
                            "21" => {
                                video.availability = "Quiesced".to_string();
                            },
                            _ => {}
                        }
                    },
                    "DriverVersion" => {
                        video.driver_version = v;
                    },
                    "InstalledDisplayDrivers" => {
                        let d: Vec<_> = v.split(',').collect();
                        for s in d {
                            video.installed_display_driver.push(s.to_string());
                        }
                    },
                    "MaxRefreshRate" => {
                        video.refresh_rate = v;
                    },
                    "Name" => {
                        video.name = v;
                    },
                    "VideoModeDescription" => {
                        video.screen_info = v;
                    },
                    "Status" => {
                        video.status = v;
                    },
                    "VideoMemoryType" => {
                        //https://msdn.microsoft.com/en-us/library/aa394512(v=vs.85).aspx
                        match v.as_str() {
                            "1" => {
                                video.video_memory_type = "Other".to_string();
                            },
                            "2" => {
                                video.video_memory_type = "Unknown".to_string();
                            },
                            "3" => {
                                video.video_memory_type = "VRAM".to_string();
                            },
                            "4" => {
                                video.video_memory_type = "DRAM".to_string();
                            },
                            "5" => {
                                video.video_memory_type = "SRAM".to_string();
                            },
                            "6" => {
                                video.video_memory_type = "WRAM".to_string();
                            },
                            "7" => {
                                video.video_memory_type = "EDO RAM".to_string();
                            },
                            "8" => {
                                video.video_memory_type = "Burst Synchronous DRAM".to_string();
                            },
                            "9" => {
                                video.video_memory_type = "Pipelined Burst SRAM".to_string();
                            },
                            "10" => {
                                video.video_memory_type = "CDRAM".to_string();
                            },
                            "11" => {
                                video.video_memory_type = "3DRAM".to_string();
                            },
                            "12" => {
                                video.video_memory_type = "SDRAM".to_string();
                            },
                            "160" => {
                                video.video_memory_type = "SGRAM".to_string();
                            },
                            _ => ()
                        }
                    }
                    "VideoArchitecture" => {
                        //https://msdn.microsoft.com/en-us/library/aa394512(v=vs.85).aspx
                        match v.as_str() {
                            "1" => {
                                video.video_architecture = "Other".to_string();
                            },
                            "2" => {
                                video.video_architecture = "Unknown".to_string();
                            },
                            "3" => {
                                video.video_architecture = "CGA".to_string();
                            },
                            "4" => {
                                video.video_architecture = "EGA".to_string();
                            },
                            "5" => {
                                video.video_architecture = "VGA".to_string();
                            },
                            "6" => {
                                video.video_architecture = "SVGA".to_string();
                            },
                            "7" => {
                                video.video_architecture = "MDA".to_string();
                            },
                            "8" => {
                                video.video_architecture = "HGC".to_string();
                            },
                            "9" => {
                                video.video_architecture = "MCGA".to_string();
                            },
                            "10" => {
                                video.video_architecture = "8514A".to_string();
                            },
                            "11" => {
                                video.video_architecture = "XGA".to_string();
                            },
                            "12" => {
                                video.video_architecture = "Linear Frame Buffer".to_string();
                            },
                            "13" => {
                                video.video_architecture = "PC-98".to_string();
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
        }

        videos
    }
}