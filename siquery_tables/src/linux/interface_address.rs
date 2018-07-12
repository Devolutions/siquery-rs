use tables::InterfaceAddress;
use linux::SystemReaderInterface;

impl InterfaceAddress {
    pub fn new(_system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
        let interface_address = Vec::new();

        interface_address.push( InterfaceAddress {
            interface: String::new(),
            address: String::new(),
            mask: String::new(),
            interface_type: String::new(),
            friendly_name: String::new(),
        });

        interface_address
    }


    pub fn get_interfaces(system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
        let mut interfaces: Vec<InterfaceAddress> = Vec::new();
        // TODO interface_address table implementation
        interfaces
    }
}