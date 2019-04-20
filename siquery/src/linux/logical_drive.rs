use crate::tables::LogicalDrive;

#[cfg(not(fuzzing))]    // Since not yet implemented for linux, disable this implementation when fuzzing to allow fuzzing interface_address for windows
impl LogicalDrive {
    pub(crate) fn get_specific() -> Vec<LogicalDrive> {
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