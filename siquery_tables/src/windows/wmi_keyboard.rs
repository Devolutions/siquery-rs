use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiKeyboard,WmiKeyboardIface};
use utils;

pub struct Reader {}
impl WmiKeyboardIface for Reader {
    fn get_wmi_keyboard_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "Win32_Keyboard", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiKeyboard {
    pub(crate) fn new() -> WmiKeyboard {
        WmiKeyboard {
            name: String::new(),
            description: String::new(),
            device_id: String::new(),
            status: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiKeyboardIface) -> Vec<WmiKeyboard> {

        let mut keyboards: Vec<WmiKeyboard> = Vec::new();

        if let Some(keyboard_info) = reader.get_wmi_keyboard_info() {
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

    pub(crate) fn get_specific() -> Vec<WmiKeyboard> {
        let reader: Box<WmiKeyboardIface> = Box::new(Reader{});
        let out = WmiKeyboard::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiKeyboardIface for Test {
        fn get_wmi_keyboard_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-keyboard.txt")))
        }
    }
    #[test]
    fn test_wmi_keyboard () {
        let reader: Box<WmiKeyboardIface> = Box::new(Test {});
        let keyboard_info = &WmiKeyboard::get_specific_ex(reader.borrow())[0];
        assert_eq!(WmiKeyboard::get_specific_ex(reader.borrow()).len(), 2);
        assert_eq!(keyboard_info.name, "Enhanced (101- or 102-key)");
        assert_eq!(keyboard_info.description, "USB Input Device");
        assert_eq!(keyboard_info.device_id, "USB\\VID_046D&amp;0&amp;0000");
        assert_eq!(keyboard_info.status, "OK");
    }
}