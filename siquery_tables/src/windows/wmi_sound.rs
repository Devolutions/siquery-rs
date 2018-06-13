use tables::WmiSound;
use utils;
use windows::SystemReaderInterface;

impl WmiSound {
    pub(crate) fn new() -> WmiSound {
        WmiSound {
            name: String::new(),
            manufacturer: String::new(),
            status: String::new(),
            dma_buffer_size: String::new(),
        }
    }

    pub(crate) fn get_sound_info(system_reader: &SystemReaderInterface) -> Vec<WmiSound> {

        let mut sounds: Vec<WmiSound> = Vec::new();

        if let Some(sound_info) = system_reader.get_wmi_sound_info() {
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
                        sound.dma_buffer_size = v;
                    },
                    _ => ()
                }
            }
        }
        sounds
    }
}