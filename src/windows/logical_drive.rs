use tables::LogicalDrive;
use utils;
use windows::SystemReaderInterface;

impl LogicalDrive {
    pub(crate) fn new() -> LogicalDrive {
        LogicalDrive {
            device_id: String::new(),
            drive_type: String::new(),
            free_space: 0,
            size: 0,
            file_system: String::new(),
        }
    }

    pub(crate) fn get_drives(system_reader: &SystemReaderInterface) -> Vec<LogicalDrive> {
        let mut drives: Vec<LogicalDrive> = Vec::new();

        if let Some(drive_info) = system_reader.get_wmi_drives_info() {
            let mut drive = LogicalDrive::new();
            let lines = drive_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if drive.device_id != "" && drive.drive_type == "Disk drive" {
                        drives.push(drive);
                    }
                    drive = LogicalDrive::new();
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
                    "DeviceID" => {
                        drive.device_id = v;
                    },
                    "FileSystem" => {
                        drive.file_system = v;
                    },
                    "Size" => {
                        drive.size = v.parse::<u64>().unwrap_or(0);
                    },
                    "FreeSpace" => {                        
                        drive.free_space = v.parse::<u64>().unwrap_or(0);
                    },
                    "DriveType" => {
                        if v == "3" {
                            drive.drive_type = String::from("Disk drive");
                        }
                    },
                    _ => ()
                }
            }
        }

        drives
    } 
}