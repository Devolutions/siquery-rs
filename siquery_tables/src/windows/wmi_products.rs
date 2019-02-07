use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiProducts, WmiProductsIface};
use utils;

pub struct Reader {}
impl WmiProductsIface for Reader {
    fn get_wmi_products_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["Product",
                "get",
                "InstallDate,Name,Vendor,Version",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiProducts {
    pub(crate) fn new() -> WmiProducts {
        WmiProducts {
            help_link: String::new(),
            install_date: String::new(),
            install_location: String::new(),
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
        }
    }

    pub fn get_specific_ex(reader: &WmiProductsIface) -> Vec<WmiProducts> {

        let mut products: Vec<WmiProducts> = Vec::new();
        if let Some(product_info) = reader.get_wmi_products_info() {
            let mut product = WmiProducts::new();
            let lines = product_info.split('\n');

            for line in lines {
                let mut element_counter = 0;
                if line.len() <= 2 {
                    if element_counter == 6 {
                        //products.push(product);
                    }
                    product = WmiProducts::new();
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
                    "HelpLink" => {
                        element_counter = element_counter + 1;
                        product.help_link = v;
                    },
                    "InstallDate" => {
                        element_counter = element_counter + 1;
                        product.install_date = v;
                    },
                    "InstallLocation" => {
                        element_counter = element_counter + 1;
                        product.install_location = v;
                    },
                    "Name" => {
                        element_counter = element_counter + 1;
                        product.name = v;
                    },
                    "Vendor" => {
                        element_counter = element_counter + 1;
                        product.vendor = v;
                    },
                    "Version" => {
                        element_counter = element_counter + 1;
                        product.version = v;
                    },
                    _ => ()
                }
            }
        }

        products
    }

    pub(crate) fn get_specific() -> Vec<WmiProducts> {
        let reader: Box<WmiProductsIface> = Box::new(Reader{});
        let out = WmiProducts::get_specific_ex(reader.borrow());
        out
    }
}

// todo test table