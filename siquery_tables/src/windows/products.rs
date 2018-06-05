extern crate winreg;

use tables::WmiProducts;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

impl Products {

    pub(crate) fn new() -> WmiProducts {
        WmiProducts {
            install_date: String::new(),
            install_location: String::new(),
            help_link: String::new(),
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
        }
    }

    pub(crate) fn get_products_info()-> Vec<Products>{
        let mut products: Vec<WmiProducts> = Vec::new();



        products
    }

}

