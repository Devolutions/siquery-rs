use tables::WmiMonitors;
use utils;
use windows::SystemReaderInterface;

impl WmiMonitors {
    pub(crate) fn new() -> WmiMonitors {
        WmiMonitors {
            name: String::new(),
            availability: String::new(),
            bandwidth: 0,
            manufacturer: String::new(),
            screen_height: 0,
            screen_width: 0,
        }
    }

    pub(crate) fn get_monitors_info(system_reader: &SystemReaderInterface) -> Vec<WmiMonitors> {

        let mut monitors: Vec<WmiMonitors> = Vec::new();

        if let Some(monitor_info) = system_reader.get_wmi_monitor_info() {
            let mut monitor = WmiMonitors::new();
            let lines = monitor_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if monitor.name != "" {
                        monitors.push(monitor);
                    }
                    monitor = WmiMonitors::new();
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
                        monitor.name = v;
                    },
                    "Availability" => {
                        match v.as_str() {
                            "1" => {
                                monitor.availability = "Other".to_string();
                            },
                            "2" => {
                                monitor.availability = "Unknown".to_string();
                            },
                            "3" => {
                                monitor.availability = "Running or Full Power".to_string();
                            },
                            "4" => {
                                monitor.availability = "Warning".to_string();
                            },
                            "5" => {
                                monitor.availability = "In Test".to_string();
                            },
                            "6" => {
                                monitor.availability = "Not Applicable".to_string();
                            },
                            "7" => {
                                monitor.availability = "Power Off".to_string();
                            },
                            "8" => {
                                monitor.availability = "Off Line".to_string();
                            },
                            "9" => {
                                monitor.availability = "Off Duty".to_string();
                            },
                            "10" => {
                                monitor.availability = "Degraded".to_string();
                            },
                            "11" => {
                                monitor.availability = "Not Installed".to_string();
                            },
                            "12" => {
                                monitor.availability = "Install Error".to_string();
                            },
                            "13" => {
                                monitor.availability = "Power Save - Unknown".to_string();
                            },
                            "14" => {
                                monitor.availability = "Power Save - Low Power Mode".to_string();
                            },
                            "15" => {
                                monitor.availability = "Power Save - Standby".to_string();
                            },
                            "16" => {
                                monitor.availability = "Power Cycle".to_string();
                            },
                            "17" => {
                                monitor.availability = "Power Save - Warning".to_string();
                            },
                            "18" => {
                                monitor.availability = "Paused ".to_string();
                            },
                            "19" => {
                                monitor.availability = "Not Ready".to_string();
                            },
                            "20" => {
                                monitor.availability = "Not Configured".to_string();
                            },
                            "21" => {
                                monitor.availability = "Quiesced".to_string();
                            },
                            _ => {}
                        }
                    },
                    "Bandwidth" => {
                        monitor.bandwidth = v.parse::<u64>().unwrap_or(0);
                    },
                    "ScreenHeight" => {
                        monitor.screen_height = v.parse::<u64>().unwrap_or(0);
                    },
                    "ScreenWidth" => {
                        monitor.screen_width = v.parse::<u64>().unwrap_or(0);
                    },
                    "MonitorManufacturer" => {
                        monitor.manufacturer = v;
                    },
                    _ => ()
                }
            }
        }

        monitors
    }
}