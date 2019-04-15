use std::process::Command;
use std::borrow::Borrow;

use tables::{LogicalDrive,LogicalDriveIface};
use utils;

pub struct Reader {}
impl LogicalDriveIface for Reader {
    fn get_wmi_drives_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "get",
                "Description,DriveType,FileSystem,FreeSpace,MaximumComponentLength,Name\
                ,Size,DriveType,SupportsFileBasedCompression,VolumeSerialNumber",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl LogicalDrive {
    pub(crate) fn new_logical_drive() -> LogicalDrive {
        LogicalDrive {
            description: String::new(),
            drive_type: String::new(),
            file_system: String::new(),
            free_space: 0,
            maximum_component_length: 0,
            name : String::new(),
            size: 0,
            supports_file_based_compression: String::new(),
            volume_serial_number: String::new(),
        }
    }

    pub(crate) fn get_specific_ex (reader: &LogicalDriveIface) -> Vec<LogicalDrive> {
        let mut drives: Vec<LogicalDrive> = Vec::new();

        if let Some(drive_info) = reader.get_wmi_drives_info() {
            let mut drive = LogicalDrive::new_logical_drive();
            let lines = drive_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if drive.description != "" && drive.volume_serial_number != "" {
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
                    "Description" => {
                        drive.description = v;
                    },
                    "DriveType" => {
                        drive.drive_type = v;
                    },
                    "FileSystem" => {
                        drive.file_system = v;
                    },
                    "FreeSpace" => {
                        drive.free_space = v.parse::<i64>().unwrap_or(0);
                    },
                    "MaximumComponentLength" => {
                        drive.maximum_component_length = v.parse::<i64>().unwrap_or(0);
                    },
                    "Name" => {
                        drive.name = v;
                    },
                    "Size" => {
                        drive.size = v.parse::<i64>().unwrap_or(0);
                    },
                    "SupportsFileBasedCompression" => {
                        drive.supports_file_based_compression = v.to_lowercase();
                    },
                    "VolumeSerialNumber" => {
                        drive.volume_serial_number = v;
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
        assert_eq!(drive.len(), 3);
        assert_eq!(drive[0].name, "C:");
        assert_eq!(drive[0].file_system, "NTFS");
        assert_eq!(drive[0].size, 496869830656);
        assert_eq!(drive[0].free_space, 55674548224);
        assert_eq!(drive[1].name, "D:");
        assert_eq!(drive[1].file_system, "");
        assert_eq!(drive[1].size, 0);
        assert_eq!(drive[1].free_space, 0);
    }
}