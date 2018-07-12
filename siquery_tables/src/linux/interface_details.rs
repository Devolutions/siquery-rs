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

    pub fn get_interface_details(_system_reader: &SystemReaderInterface) -> Vec<InterfaceDetails> {
        let mut interfaces: Vec<InterfaceDetails> = Vec::new();
        let interface = InterfaceDetails::new();
        interfaces.push(interface);

        // TODO interface_details table implementation
        interfaces
    }

}