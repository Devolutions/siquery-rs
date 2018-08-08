use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiSound,WmiSoundIface};
use utils;

pub struct Reader {}
impl WmiSoundIface for Reader {
    fn get_wmi_sound_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["sounddev", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiSound {
    pub(crate) fn new() -> WmiSound {
        WmiSound {
            name: String::new(),
            manufacturer: String::new(),
            status: String::new(),
            dma_buffer_size: 0,
        }
    }

    pub fn get_specific_ex(reader: &WmiSoundIface) -> Vec<WmiSound> {

        let mut sounds: Vec<WmiSound> = Vec::new();

        if let Some(sound_info) = reader.get_wmi_sound_info() {
            let mut sound = WmiSound::new();
            let lines = sound_info.split('\n');
            for line in lines {
                if line.len() <= 2 {
                    if sound.name != "" {
                        sounds.push(sound);
                    }
                    sound = WmiSound::new();
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
                        sound.name = v;
                    },
                    "Manufacturer" => {
                        sound.manufacturer = v;
                    },
                    "Status" => {
                        sound.status = v;
                    },
                    "DMABufferSize" => {
                        sound.dma_buffer_size = v.parse::<u16>().unwrap_or(0);
                    },
                    _ => ()
                }
            }
        }
        sounds
    }

    pub(crate) fn get_specific() -> Vec<WmiSound> {
        let reader: Box<WmiSoundIface> = Box::new(Reader{});
        let out = WmiSound::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiSoundIface for Test {
        fn get_wmi_sound_info(&self)-> Option<String>{
            Some(String::from(include_str!("../../test_data/wmi-sound.txt")))
        }
    }
    #[test]
    fn test_wmi_sound () {
        let reader: Box<WmiSoundIface> = Box::new(Test{});
        let sound_info = &WmiSound::get_specific_ex(reader.borrow())[0];
        assert_eq!(sound_info.name, "Fabrikam Audio");
        assert_eq!(sound_info.manufacturer, "Fabrikam, Inc.");
        assert_eq!(sound_info.status, "OK");
        assert_eq!(sound_info.dma_buffer_size, 256);
    }
}