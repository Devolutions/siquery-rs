use tables::Products;
use winreg::RegKey;
use winreg::enums::*;
use chrono::{NaiveDateTime, NaiveDate, DateTime, Duration, Utc, TimeZone};
use std::fs;
use filetime::FileTime;


impl Products {
    pub(crate) fn new() -> Products {
        Products {
            install_date: String::new(),
            install_location: String::new(),
            help_link: String::new(),
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
            size : 0,
        }
    }

    pub(crate) fn get_specific() -> Vec<Products> {
        let mut products: Vec<Products> = Vec::new();

        let _hklm_local_microsoft = &RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_local_microsoft = _hklm_local_microsoft.open_subkey_with_flags(
            r#"Software\Microsoft\Windows\CurrentVersion\Uninstall"#, KEY_READ)
            .expect("Failed to open subkey");
        get_products_info(&mut products, subkey_local_microsoft);

        let _hklm_local_wow6432node = &RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_local_wow6432node = _hklm_local_wow6432node.open_subkey_with_flags(
            r#"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"#, KEY_READ)
            .expect("Failed to open subkey");
        get_products_info(&mut products, subkey_local_wow6432node);

        let _hklm_local_classes = &RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_local_classes = _hklm_local_classes.open_subkey_with_flags(
            r#"Software\Classes\Installer\Products"#, KEY_READ)
            .expect("Failed to open subkey");
        get_products_info(&mut products, subkey_local_classes);


        let _hklm_current_user_current_version = &RegKey::predef(HKEY_CURRENT_USER);
        let subkey_current_user_current_version = _hklm_current_user_current_version.open_subkey_with_flags(
            r#"Software\Microsoft\Windows\CurrentVersion"#, KEY_READ)
            .expect("Failed to open subkey");
        get_products_info(&mut products, subkey_current_user_current_version);

        let _hklm_current_user_products = &RegKey::predef(HKEY_CURRENT_USER);
        let subkey_current_user_products = _hklm_current_user_products.open_subkey_with_flags(
            r#"Software\Microsoft\Installer\Products"#, KEY_READ)
            .expect("Failed to open subkey");
        get_products_info(&mut products, subkey_current_user_products);

        products
    }
}


pub fn get_products_info(ref mut products: &mut Vec<Products>, hkey: RegKey) {
    for _x in 0..hkey.enum_keys().count() {
        let mut add_program = true;
        let mut product = Products::new();
        let display_name_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = display_name_key.and_then(|display_name_key| hkey.open_subkey_with_flags(display_name_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("DisplayName"))
            .and_then(|name: String| {
                product.name = name;
                Ok(())
            });

        let display_version_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = &display_version_key.and_then(|display_version_key| hkey.open_subkey_with_flags(display_version_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("DisplayVersion"))
            .and_then(|version: String| {
                product.version = version;
                Ok(())
            });

        let publisher_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = publisher_key.and_then(|publisher_key| hkey.open_subkey_with_flags(publisher_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("Publisher"))
            .and_then(|vendor: String| {
                product.vendor = vendor;
                Ok(())
            });

        let install_date_key = hkey.enum_keys().nth(_x).unwrap();
        let mut date = "".to_string();
        let _ = install_date_key.and_then(|install_date_key| hkey.open_subkey_with_flags(install_date_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("InstallDate"))
            .and_then(|install_date: String| {
                date = install_date;
                Ok(())
            });
        if date != "" {
            let mut install_date = date.clone();
            if install_date.len() >= 8 {
                install_date.truncate(8);
                if let Ok(formated_date) = NaiveDate::parse_from_str(&install_date, "%Y%m%d") {
                    product.install_date = formated_date.format("%Y-%m-%d").to_string();
                }
            }
        }

        let install_location_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = install_location_key.and_then(|install_location_key| hkey.open_subkey_with_flags(install_location_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("InstallLocation"))
            .and_then(|install_location: String| {
                product.install_location = install_location;
                Ok(())
            });

        if date == "" {
            let attr = fs::symlink_metadata(product.install_location.clone()).ok();
            if let Ok(att) = fs::symlink_metadata(product.install_location.clone()) {
                let mut time_ = FileTime::from_creation_time(&att).unwrap();

                let utc = Utc;
                let d1 = Utc::now();
                let d2 = utc.datetime_from_str(&"Jan 1 00:00:00 1601", "%b %d %H:%M:%S %Y").unwrap();
                let d3 = utc.datetime_from_str(&"Jan 1 00:00:00 1970", "%b %d %H:%M:%S %Y").unwrap();

                let duration_1601 = d1.signed_duration_since(d2);
                let seconds_since_d2 = duration_1601.num_days() * 24 * 60 * 60;

                let duration_1970 = d1.signed_duration_since(d3);
                let seconds_since_d3 = duration_1970.num_days() * 24 * 60 * 60;
                let dt = NaiveDateTime::from_timestamp(time_.seconds() - seconds_since_d2 + seconds_since_d3, time_.nanoseconds());

                product.install_date = dt.format("%Y-%m-%d").to_string();
            }
        }

        let help_link_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = help_link_key.and_then(|help_link_key| hkey.open_subkey_with_flags(help_link_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("HelpLink"))
            .and_then(|help_link: String| {
                product.help_link = help_link;
                Ok(())
            });

        let system_component = hkey.enum_keys().nth(_x).unwrap();
        let mut system_component_value: u32 = 9999;
        let _ = system_component.and_then(|system_component_key| hkey.open_subkey_with_flags(system_component_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("SystemComponent"))
            .and_then(|system_component_key: u32| {
                system_component_value = system_component_key;
                Ok(())
            });
        if system_component_value == 1 {
            add_program = false;
        }

        let uninstall_string = hkey.enum_keys().nth(_x).unwrap();
        let mut uninstall_string_value = "".to_string();
        let _ = uninstall_string.and_then(|uninstall_string_key| hkey.open_subkey_with_flags(uninstall_string_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("UninstallString"))
            .and_then(|uninstall_string_key: String| {
                uninstall_string_value = uninstall_string_key;
                Ok(())
            });

        let mut size: i64 = 0;
        let estimated_size_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = estimated_size_key.and_then(|estimated_s_key| hkey.open_subkey_with_flags(estimated_s_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("EstimatedSize"))
            .and_then(|size_value: u32| {
                size = size_value as i64;
                Ok(())
            });
        if uninstall_string_value == "" && size == 0 {
            add_program = false;
        } else {
            product.size = size * 1024;
        }

        let parent_key_name = hkey.enum_keys().nth(_x).unwrap();
        let mut parent_key_value = "".to_string();
        let _ = parent_key_name.and_then(|parent_key| hkey.open_subkey_with_flags(parent_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("ParentKeyName"))
            .and_then(|parent_key: String| {
                parent_key_value = parent_key;
                Ok(())
            });

        if parent_key_value != "" {
            add_program = false;
        }

        if product.name != "" && add_program {
            products.push(product);
        }
    }
}