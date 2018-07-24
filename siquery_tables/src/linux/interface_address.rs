use tables::InterfaceAddress;

#[cfg(not(fuzzing))]    // Since not yet implemented for linux, disable this implementation when fuzzing to allow fuzzing interface_address for windows
impl InterfaceAddress {
    pub(crate) fn new() -> InterfaceAddress {
        InterfaceAddress {
            interface: String::new(),
            address: String::new(),
            mask: String::new(),
            interface_type: String::new(),
            friendly_name: String::new(),
        }
    }

    pub(crate) fn get_specific() -> Vec<InterfaceAddress> {
        // TODO: implement interface_address table
        let mut output: Vec<InterfaceAddress> = Vec::new();
        output.push(
            InterfaceAddress::new()
        );
        output
    }
}