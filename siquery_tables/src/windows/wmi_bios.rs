use tables::WmiBios;
use utils;
use windows::SystemReaderInterface;

impl WmiBios {
    pub(crate) fn get_bios_info(system_reader: &SystemReaderInterface) -> WmiBios {

        let mut bios = WmiBios {
            caption : String::new(),
            manufacturer: String::new(),
            release_date: String::new(),
            serial_number: String::new(),
            smbios_version: String::new(),
        };

        if let Some(bios_info) = system_reader.get_wmi_bios_info() {
            let lines = bios_info.split('\n');

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
                    "Caption" => {
                        bios.caption = v;
                    },
                    "Manufacturer" => {
                        bios.manufacturer = v;
                    },
                    "ReleaseDate" => {
                        v.truncate(8);
                        bios.release_date = v;
                    },
                    "SerialNumber" => {
                        bios.serial_number = v;
                    },
                    "SMBIOSBIOSVersion" => {
                        bios.smbios_version = v;
                    },
                    _ => ()
                }
            }
        }

        bios
    }
}