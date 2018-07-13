use utils;
use tables::WmiComputerInfo;
use windows::SystemReaderInterface;

impl WmiComputerInfo {
    pub fn new() -> WmiComputerInfo {
        WmiComputerInfo {
            computer_name: String::new(),
            domain: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            number_of_processors: String::new(),
            system_type: String::new(),
        }
    }

    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<WmiComputerInfo> {
        let mut output : Vec<WmiComputerInfo> = Vec::new();
        let mut computer = WmiComputerInfo::new();

        if let Some(computer_info) = system_reader.get_wmi_computer_info() {

            let lines = computer_info.split('\n');

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
                        computer.computer_name = v;
                    },
                    "Domain" => {
                        computer.domain = v;
                    },
                    "Manufacturer" => {
                        computer.manufacturer = v;
                    },
                    "Model" => {
                        computer.model = v;
                    },
                    "NumberOfProcessors" => {
                        computer.number_of_processors = v;
                    },
                    "SystemType" => {
                        computer.system_type = v;
                    },
                    _ => {}
                }
            }
        }
        output.push(computer);
        output
    }
}
