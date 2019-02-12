use tables::Products;
use winreg::RegKey;
use winreg::enums::*;

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


pub fn get_products_info(ref mut products: &mut Vec<Products>, hkey: RegKey){
    let mut product = Products::new();
    let mut add_program = true;
    for _x in 0..hkey.enum_keys().count() {
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
        let _ = install_date_key.and_then(|install_date_key| hkey.open_subkey_with_flags(install_date_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("InstallDate"))
            .and_then(|install_date: String| {
                product.install_date = install_date;
                Ok(())
            });

        let install_location_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = install_location_key.and_then(|install_location_key| hkey.open_subkey_with_flags(install_location_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("InstallLocation"))
            .and_then(|install_location: String| {
                product.install_location = install_location;
                Ok(())
            });

        let help_link_key = hkey.enum_keys().nth(_x).unwrap();
        let _ = help_link_key.and_then(|help_link_key| hkey.open_subkey_with_flags(help_link_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("HelpLink"))
            .and_then(|help_link: String| {
                product.help_link = help_link;
                Ok(())
            });

        let system_component = hkey.enum_keys().nth(_x).unwrap();
        let mut system_component_value: u64 = 0;
        let _ = system_component.and_then(|system_component_key| hkey.open_subkey_with_flags(system_component_key, KEY_READ))
            .and_then(|program_key| program_key.get_value("SystemComponent"))
            .and_then(|system_component_key: u64| {
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

        if uninstall_string_value.is_empty() {
            //add_program = false;
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
        product = Products::new();
    }
}