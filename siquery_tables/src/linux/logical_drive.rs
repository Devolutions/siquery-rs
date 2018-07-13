use tables::LogicalDrive;
use linux::SystemReaderInterface;

impl LogicalDrive {
    pub(crate) fn get_specific(_system_reader: &SystemReaderInterface) -> Vec<LogicalDrive> {
        // TODO: implement logical_drive table

        let mut output = Vec::new();

        output.push(
            LogicalDrive {
                device_id: String::new(),
                drive_type: String::new(),
                free_space: 0,
                size: 0,
                file_system: String::new(),
            }
        );
        output
    }
}