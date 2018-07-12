use tables::;
use linux::SystemReaderInterface;

impl InterfaceAddress {
    pub(crate) fn new(_system_reader: &SystemReaderInterface) -> Vec<InterfaceAddress> {
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

    // TODO interface_address table implementation
}