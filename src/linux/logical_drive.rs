use tables::LogicalDrive;
use linux::SystemReaderInterface;

impl LogicalDrive {
    pub(crate) fn new(_system_reader: &SystemReaderInterface) -> LogicalDrive {
        LogicalDrive {
            device_id: String::new(),
            drive_type: String::new(),
            free_space: 0,
            size: 0,
            file_system: String::new(),
        }
    }
}