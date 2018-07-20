use std::process::Command;
use std::borrow::Borrow;

use tables::{LogicalDrive,LogicalDriveIface};
use utils;

pub struct Reader {}
impl LogicalDriveIface for Reader {
    fn get_wmi_drives_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "get", "DeviceID,FileSystem,Size,FreeSpace,DriveType",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl LogicalDrive {
    pub(crate) fn new_logical_drive() -> LogicalDrive {
        LogicalDrive {
            device_id: String::new(),
            drive_type: String::new(),
            free_space: 0,
            size: 0,
            file_system: String::new(),
        }
    }

    pub(crate) fn get_specific_ex (reader: &LogicalDriveIface) -> Vec<LogicalDrive> {
        let mut drives: Vec<LogicalDrive> = Vec::new();

        if let Some(drive_info) = reader.get_wmi_drives_info() {
            let mut drive = LogicalDrive::new_logical_drive();
            let lines = drive_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if drive.device_id != "" && drive.drive_type == "Disk drive" {
                        drives.push(drive);
                    }
                    drive = LogicalDrive::new_logical_drive();
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

    pub(crate) fn get_specific() -> Vec<LogicalDrive> {
        let reader: Box<LogicalDriveIface> = Box::new(Reader{});
        let out = LogicalDrive::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl LogicalDriveIface for Test {
        fn get_wmi_drives_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-driveinfo.txt")))
        }
    }
    #[test]
    fn test_logical_drives () {
        let reader: Box<LogicalDriveIface> = Box::new(Test {});
        let drive = LogicalDrive::get_specific_ex(reader.borrow());
        assert_eq!(drive.len(), 2);
        assert_eq!(drive[0].device_id, "C:");
        assert_eq!(drive[0].file_system, "NTFS");
        assert_eq!(drive[0].size, 496869830656);
        assert_eq!(drive[0].free_space, 55674548224);
        assert_eq!(drive[1].device_id, "E:");
        assert_eq!(drive[1].file_system, "NTFS");
        assert_eq!(drive[1].size, 501215232);
        assert_eq!(drive[1].free_space, 469622784);
    }
}
