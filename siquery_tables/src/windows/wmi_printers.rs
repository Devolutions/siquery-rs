use tables::WmiPrinters;
use utils;
use windows::SystemReaderInterface;

impl WmiPrinters {
    pub(crate) fn new() -> WmiPrinters {
        WmiPrinters {
            attributes: String::new(),
            caption: String::new(),
            creation_class_name: String::new(),
            device_id: String::new(),
            do_complete_first: String::new(),
            driver_name: String::new(),
            extended_printer_status: String::new(),
            horizontal_resolution: String::new(),
            local: String::new(),
            name: String::new(),
            port_name: String::new(),
            printer_status: String::new(),
            print_job_data_type: String::new(),
            print_processor: String::new(),
            priority: String::new(),
            status: String::new(),
            system_creation_class_name: String::new(),
            system_name: String::new(),
            vertical_resolution: String::new(),
        }
    }

    pub(crate) fn get_printers_info(system_reader: &SystemReaderInterface) -> Vec<WmiPrinters> {

        let mut printers: Vec<WmiPrinters> = Vec::new();

        if let Some(printer_info) = system_reader.get_wmi_printers_info() {
            let mut printer = WmiPrinters::new();
            let lines = printer_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if printer.attributes != "" {
                        printers.push(printer);
                    }
                    printer = WmiPrinters::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Attributes" => {
                        printer.attributes = v;
                    },
                    "Caption" => {
                        printer.caption = v;
                    },
                    "CreationClassName" => {
                        printer.creation_class_name = v;
                    },
                    "DeviceID" => {
                        printer.device_id = v;
                    },
                    "DoCompleteFirst" => {
                        printer.do_complete_first = v;
                    },
                    "DriverName" => {
                        printer.driver_name = v;
                    },
                    "ExtendedPrinterStatus" => {
                        printer.extended_printer_status = v;
                    },
                    "HorizontalResolution" => {
                        printer.horizontal_resolution = v;
                    },
                    "Local" => {
                        printer.local = v;
                    },
                    "Name" => {
                        printer.name = v;
                    },
                    "PortName" => {
                        printer.port_name = v;
                    },
                    "PrinterStatus" => {
                        printer.printer_status = v;
                    },
                    "PrintJobDataType" => {
                        printer.print_job_data_type = v;
                    },
                    "PrintProcessor" => {
                        printer.print_processor = v;
                    },
                    "Priority" => {
                        printer.priority = v;
                    },
                    "Status" => {
                        printer.status = v;
                    },
                    "SystemCreationClassName" => {
                        printer.system_creation_class_name = v;
                    },
                    "SystemName" => {
                        printer.system_name = v;
                    },
                    "VerticalResolution" => {
                        printer.vertical_resolution = v;
                    },
                    _ => ()
                }
            }
        }

        printers
    }
}