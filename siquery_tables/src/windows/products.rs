extern crate winreg;

use tables::Products;
use windows::products::winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

impl Products {
    pub(crate) fn new() -> Products {
        Products {
            install_date: String::new(),
            install_location: String::new(),
            help_link: String::new(),
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
        }
    }

    pub(crate) fn get_products_info() -> Vec<Products> {
        let mut products: Vec<Products> = Vec::new();
        let mut product = Products::new();

        let hklm = &winreg::RegKey::predef(HKEY_LOCAL_MACHINE);

        let subkey = hklm.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Uninstall"#, KEY_READ)
            .expect("Failed to open subkey");

        for _x in 0..subkey.enum_keys().count() {

            let display_name_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = display_name_key.and_then(|display_name_key| subkey.open_subkey_with_flags(display_name_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("DisplayName"))
                .and_then(|name: String| { product.name = name;
                    Ok(())
                });

            let display_version_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = &display_version_key.and_then(|display_version_key| subkey.open_subkey_with_flags(display_version_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("DisplayVersion"))
                .and_then(|version: String| {
                    product.version = version;
                    Ok(())
                });

            let publisher_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = publisher_key.and_then(|publisher_key| subkey.open_subkey_with_flags(publisher_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("Publisher"))
                .and_then(|vendor: String| {
                    product.vendor = vendor;
                    Ok(())
                });

            let install_date_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = install_date_key.and_then(|install_date_key| subkey.open_subkey_with_flags(install_date_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("InstallDate"))
                .and_then(|install_date: String| {
                    product.install_date = install_date;
                    Ok(())
                });

            let install_location_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = install_location_key.and_then(|install_location_key| subkey.open_subkey_with_flags(install_location_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("InstallLocation"))
                .and_then(|install_location: String| {
                    product.install_location = install_location;
                    Ok(())
                });

            let help_link_key = subkey.enum_keys().nth(_x).unwrap();
            let _ = help_link_key.and_then(|help_link_key| subkey.open_subkey_with_flags(help_link_key, KEY_READ))
                .and_then(|program_key| program_key.get_value("HelpLink"))
                .and_then(|help_link: String| {
                    product.help_link = help_link;
                    Ok(())
                });

            if product.name != ""{
                if product.install_date != ""{
                    product.install_date.insert_str(4, "/");
                    product.install_date.insert_str(7, "/");
                }
                products.push(product);
            }

            product = Products::new();
        }

        products
    }
}




