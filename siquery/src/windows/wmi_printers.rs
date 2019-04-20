use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiPrinters,WmiPrintersIface};
use crate::utils;

pub struct Reader {}
impl WmiPrintersIface for Reader {
    fn get_wmi_printers_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["printer",
                "get",
                "Attributes,Caption,CreationClassName,DeviceID,DoCompleteFirst,DriverName,\
                ExtendedPrinterStatus,HorizontalResolution,Local,Name,PortName,PrinterStatus,\
                PrintJobDataType,PrintProcessor,Priority,Status,SystemCreationClassName,\
                SystemName,VerticalResolution",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiPrinters {
    pub(crate) fn new() -> WmiPrinters {
        WmiPrinters {
            attributes: 99999,
            caption: String::new(),
            creation_class_name: String::new(),
            device_id: String::new(),
            do_complete_first: String::new(),
            driver_name: String::new(),
            extended_printer_status: 0,
            horizontal_resolution: 0,
            local: String::new(),
            name: String::new(),
            port_name: String::new(),
            printer_status: 0,
            print_job_data_type: String::new(),
            print_processor: String::new(),
            priority: 0,
            status: String::new(),
            system_creation_class_name: String::new(),
            system_name: String::new(),
            vertical_resolution: 0,
        }
    }

    pub fn get_specific_ex(reader: &WmiPrintersIface) -> Vec<WmiPrinters> {

        let mut output: Vec<WmiPrinters> = Vec::new();

        if let Some(printer_info) = reader.get_wmi_printers_info() {
            let mut printer = WmiPrinters::new();
            let lines = printer_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if printer.attributes != 99999 {
                        output.push(printer);
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
                        printer.attributes = v.parse::<u32>().unwrap_or(0);
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
                        printer.do_complete_first = v.to_lowercase();
                    },
                    "DriverName" => {
                        printer.driver_name = v;
                    },
                    "ExtendedPrinterStatus" => {
                        printer.extended_printer_status = v.parse::<u16>().unwrap_or(0);
                    },
                    "HorizontalResolution" => {
                        printer.horizontal_resolution = v.parse::<u32>().unwrap_or(0);
                    },
                    "Local" => {
                        printer.local = v.to_lowercase();
                    },
                    "Name" => {
                        printer.name = v;
                    },
                    "PortName" => {
                        printer.port_name = v;
                    },
                    "PrinterStatus" => {
                        printer.printer_status = v.parse::<u16>().unwrap_or(0);
                    },
                    "PrintJobDataType" => {
                        printer.print_job_data_type = v;
                    },
                    "PrintProcessor" => {
                        printer.print_processor = v;
                    },
                    "Priority" => {
                        printer.priority = v.parse::<u32>().unwrap_or(0);
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
                        printer.vertical_resolution = v.parse::<u32>().unwrap_or(0);
                    },
                    _ => ()
                }
            }
        }

        output
    }

    pub(crate) fn get_specific() -> Vec<WmiPrinters> {
        let reader: Box<WmiPrintersIface> = Box::new(Reader{});
        let out = WmiPrinters::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiPrintersIface for Test {
        fn get_wmi_printers_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-printers.txt")))
        }
    }
    #[test]
    fn test_wmi_printers () {
        let reader: Box<WmiPrintersIface> = Box::new(Test{});
        let test_printers = &WmiPrinters::get_specific_ex(reader.borrow())[0];
        assert_eq!(test_printers.caption, "Snagit 2018");
        assert_eq!(test_printers.creation_class_name, "Win32_Printer");
        assert_eq!(test_printers.device_id, "Snagit 2018");
        assert_eq!(test_printers.do_complete_first, "false");
        assert_eq!(test_printers.driver_name, "Snagit 18 Printer");
        assert_eq!(test_printers.extended_printer_status, 2);
        assert_eq!(test_printers.horizontal_resolution, 200);
        assert_eq!(test_printers.local, "true");
        assert_eq!(test_printers.name, "Snagit 2018");
        assert_eq!(test_printers.port_name, "C:\\ProgramData\\TechSmith\\Snagit18\\PrinterPortFile");
        assert_eq!(test_printers.printer_status, 3);
        assert_eq!(test_printers.print_job_data_type, "RAW");
        assert_eq!(test_printers.print_processor, "winprint");
        assert_eq!(test_printers.priority, 1);
        assert_eq!(test_printers.status, "Unknown");
        assert_eq!(test_printers.system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(test_printers.system_name, "ekyaw");
        assert_eq!(test_printers.vertical_resolution, 200);
    }
}