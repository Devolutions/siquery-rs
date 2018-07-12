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

    pub(crate) fn get_interfaces(_system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
        let mut interfaces: Vec<InterfaceAddress> = Vec::new();
        let address = InterfaceAddress::new();

        interfaces.push(address);

        // TODO interface_address table implementation
        interfaces
    }
}