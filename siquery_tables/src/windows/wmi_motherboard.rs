use tables::WmiMotherboard;
use utils;
use windows::SystemReaderInterface;

impl WmiMotherboard {
    pub(crate) fn get_motherboard_info(system_reader: &SystemReaderInterface) -> WmiMotherboard {

        let mut motherboard = WmiMotherboard {
            name: String::new(),
            manufacturer: String::new(),
            product: String::new(),
            serial_number: String::new(),
            version: String::new(),
        };
        if let Some(motherboard_info) = system_reader.get_wmi_motherboard_info() {
            let lines = motherboard_info.split('\n');

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
                    "Name" => {
                        motherboard.name = v;
                    },
                    "Manufacturer" => {
                        motherboard.manufacturer = v;
                    },
                    "Product" => {
                        motherboard.product = v;
                    },
                    "SerialNumber" => {
                        motherboard.serial_number = v;
                    },
                    "Version" => {
                        motherboard.version = v;
                    },
                    _ => ()
                }
            }
        }

        motherboard
    }
}