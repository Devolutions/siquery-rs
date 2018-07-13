use tables::WmiPointingDevice;
use utils;
use windows::SystemReaderInterface;

impl WmiPointingDevice {
    pub(crate) fn new() -> WmiPointingDevice {
        WmiPointingDevice {
            name: String::new(),
            manufacturer: String::new(),
            description: String::new(),
            pointing_type: String::new(),
            status: String::new(),
        }
    }

    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<WmiPointingDevice> {

        let mut pointing_devices: Vec<WmiPointingDevice> = Vec::new();

        if let Some(pointing_device_info) = system_reader.get_wmi_pointing_device() {
            let mut pointing_device = WmiPointingDevice::new();
            let lines = pointing_device_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if pointing_device.name != "" {
                        pointing_devices.push(pointing_device);
                    }
                    pointing_device = WmiPointingDevice::new();
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
                        pointing_device.name = v;
                    },
                    "Manufacturer" => {
                        pointing_device.manufacturer = v;
                    },
                    "Description" => {
                        pointing_device.description = v;
                    },
                    "PointingType" => {
                        match v.as_str() {
                            "1" => {
                                pointing_device.pointing_type = "Other".to_string();
                            },
                            "2" => {
                                pointing_device.pointing_type = "Unknown".to_string();
                            },
                            "3" => {
                                pointing_device.pointing_type = "Mouse".to_string();
                            },
                            "4" => {
                                pointing_device.pointing_type = "Track Ball".to_string();
                            },
                            "5" => {
                                pointing_device.pointing_type = "Track Point".to_string();
                            },
                            "6" => {
                                pointing_device.pointing_type = "Glide Point".to_string();
                            },
                            "7" => {
                                pointing_device.pointing_type = "Touch Pad".to_string();
                            },
                            "8" => {
                                pointing_device.pointing_type = "Touch Screen".to_string();
                            },
                            "9" => {
                                pointing_device.pointing_type = "Mouse - Optical Sensor".to_string();
                            },
                            _ => ()
                        }
                    },
                    "Status" => {
                        pointing_device.status = v;
                    }
                    _ => ()
                }
            }
        }
        pointing_devices
    }
}