use tables::InterfaceDetails;
use linux::SystemReaderInterface;

impl InterfaceDetails {
    pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            mtu: 1500,
            enabled: 1,
        }
    }

    pub fn get_specific(_system_reader: &SystemReaderInterface) -> Vec<InterfaceDetails> {
        // TODO: implement interface_details table
        let mut interfaces: Vec<InterfaceDetails> = Vec::new();
        interfaces.push(
            InterfaceDetails::new()
        );
        interfaces
    }
}