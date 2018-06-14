use tables::WmiKeyboard;
use utils;
use windows::SystemReaderInterface;

impl WmiKeyboard {
    pub(crate) fn new() -> WmiKeyboard {
        WmiKeyboard {
            name: String::new(),
            description: String::new(),
            device_id: String::new(),
            status: String::new(),
        }
    }

    pub(crate) fn get_keyboard_info(system_reader: &SystemReaderInterface) -> Vec<WmiKeyboard> {

        let mut keyboards: Vec<WmiKeyboard> = Vec::new();

        if let Some(keyboard_info) = system_reader.get_wmi_keyboard_info() {
            let mut keyboard = WmiKeyboard::new();
            let lines = keyboard_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if keyboard.name != "" {
                        keyboards.push(keyboard);
                    }
                    keyboard = WmiKeyboard::new();
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
                        keyboard.name = v;
                    },
                    "Description" => {
                        keyboard.description = v;
                    },
                    "DeviceID" => {
                        keyboard.device_id = v;
                    },
                    "Status" => {
                        keyboard.status = v;
                    },
                    _ => ()
                }
            }
        }

        keyboards
    }
}