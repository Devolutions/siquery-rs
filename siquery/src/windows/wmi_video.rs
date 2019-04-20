use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiVideo,WmiVideoIface};
use crate::utils;

pub struct Reader {}
impl WmiVideoIface for Reader {
    fn get_wmi_video_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}


impl WmiVideo {
    pub(crate) fn new() -> WmiVideo {
        WmiVideo {
            name: String::new(),
            adapter_compatibility: String::new(),
            adapter_dac_type: String::new(),
            adapter_ram: 0,
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

    pub fn get_specific_ex(reader: &WmiVideoIface) -> Vec<WmiVideo> {

        let mut videos: Vec<WmiVideo> = Vec::new();

        if let Some(video_info) = reader.get_wmi_video_info() {
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
                        let mut ram = v.parse::<u32>().unwrap_or(0) / 1073741824;
                        video.adapter_ram = ram;
                    },
                    "Availability" => {
                        video.availability = v;
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
                        video.video_memory_type = v;
                    }
                    "VideoArchitecture" => {
                        //https://msdn.microsoft.com/en-us/library/aa394512(v=vs.85).aspx
                        video.video_architecture = v;
                    }
                    _ => ()
                }
            }
        }

        videos
    }

    pub(crate) fn get_specific() -> Vec<WmiVideo> {
        let reader: Box<WmiVideoIface> = Box::new(Reader{});
        let out = WmiVideo::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiVideoIface for Test {
        fn get_wmi_video_info(&self)-> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-video.txt")))
        }
    }
    #[test]
    fn test_wmi_video () {
        let reader: Box<WmiVideoIface> = Box::new(Test{});
        let video_info = &WmiVideo::get_specific_ex(reader.borrow())[0];
        assert_eq!(WmiVideo::get_specific_ex(reader.borrow()).len(), 3);
        assert_eq!(video_info.name, "Graphic Design Institute 940MX");
        assert_eq!(video_info.adapter_compatibility, "Graphic Design Institute");
        assert_eq!(video_info.adapter_dac_type, "Integrated RAMDAC");
        assert_eq!(video_info.adapter_ram, 2);
        assert_eq!(video_info.availability, "16");
        assert_eq!(video_info.driver_version, "23.21.13.9065");
        assert_eq!(video_info.installed_display_driver.len(), 2);
        assert_eq!(video_info.refresh_rate, "60");
        assert_eq!(video_info.screen_info, "1920 x 1080 x 4294967296 colors");
        assert_eq!(video_info.status, "OK");
        assert_eq!(video_info.video_architecture, "7");
        assert_eq!(video_info.video_memory_type, "6");
    }
}