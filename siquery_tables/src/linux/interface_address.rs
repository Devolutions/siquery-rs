use tables::InterfaceAddress;
use linux::SystemReaderInterface;

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

    pub(crate) fn get_specific(_system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
        // TODO: implement logical_drive table
        let mut output: Vec<InterfaceAddress> = Vec::new();
        output.push(
            InterfaceAddress::new()
        );
        output
    }
}