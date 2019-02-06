use treexml::{Element,Document,XmlVersion::Version10};
//use heck::CamelCase;
//use rusqlite::{Rows, Row as RusqliteRow, types::{Value, Type}};

use tables::*;

pub fn execute_inventory_query() {
    let mut root = Element::new("InventorySystemInformation");

    //get_local_accounts_inv(&mut root);
    get_sysem_info_inv(&mut root);

    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    println!("{}",doc.to_string());
}

fn get_local_accounts_inv(ref mut root: &mut Element) {

    let wmi_local_accounts = WmiLocalAccounts::get_specific();

    let mut local_accounts = Element::new("LocalAccounts");

    for local_account in wmi_local_accounts {
        let mut remote_account = Element::new("RemoteAccount");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Description");
        let mut child_3 = Element::new("Domain");
        let mut child_4 = Element::new("LocalAccount");
        let mut child_5 = Element::new("Name");
        let mut child_6 = Element::new("SID");
        let mut child_7 = Element::new("Status");

        child_1.text = Some(local_account.caption);
        child_2.text = Some(local_account.description);
        child_3.text = Some(local_account._domain);
        child_4.text = Some(local_account.local_account.to_lowercase());
        child_5.text = Some(local_account.name);
        child_6.text = Some(local_account.sid);
        child_7.text = Some(local_account.status);

        remote_account.children.push(child_1);
        remote_account.children.push(child_2);
        remote_account.children.push(child_3);
        remote_account.children.push(child_4);
        remote_account.children.push(child_5);
        remote_account.children.push(child_6);
        remote_account.children.push(child_7);

        local_accounts.children.push(remote_account);
    }

    root.children.push(local_accounts);
}

fn get_sysem_info_inv(ref mut root: &mut Element) {
    for entry in WmiComputerInfo::get_specific(){
        let mut parent = Element::new("System");

        let mut child_1 = Element::new("Domain");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("Model");
        let mut child_4 = Element::new("NumberOfProcessors");
        let mut child_5 = Element::new("SystemType");

        child_1.text = Some(entry.domain);
        child_2.text = Some(entry.manufacturer);
        child_3.text = Some(entry.model);
        child_4.text = Some(entry.number_of_processors.to_string());
        child_5.text = Some(entry.system_type);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        root.children.push(parent);
    }

    for entry in WmiBios::get_specific() {
        let mut parent = Element::new("Bios");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("ReleaseDate");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("SMBIOSBIOSVersion");

        child_1.text = Some(entry.caption);
        child_2.text = Some(entry.manufacturer);
        //fixme child_3.text = Some(entry.release_date);
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.smbios_version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        //parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        root.children.push(parent);
    }

    for entry in WmiOsVersion::get_specific(){
        let mut parent = Element::new("OperatingSystem");

        let mut child_1  = Element::new("BuildNumber");
        let mut child_2  = Element::new("CSName");
        let mut child_3  = Element::new("FreePhysicalMemory");
        let mut child_4  = Element::new("FreeVirtualMemory");
        let mut child_5  = Element::new("InstallDate");
        let mut child_6  = Element::new("LastBootUpTime");
        let mut child_7  = Element::new("Locale");
        let mut child_8  = Element::new("Manufacturer");
        let mut child_9  = Element::new("Name");
        let mut child_10 = Element::new("OSType");
        let mut child_11 = Element::new("SizeStoredInPagingFiles");
        let mut child_12 = Element::new("TotalVirtualMemorySize");
        let mut child_13 = Element::new("TotalVisibleMemorySize");
        let mut child_14 = Element::new("Version");
        let mut child_15 = Element::new("WindowsDirectory");

        child_1.text  = Some(entry.build_number);
        child_2.text  = Some(entry.csname);
        child_3.text  = Some(entry.free_physical_mem);
        child_4.text  = Some(entry.free_virtual_mem);
        //fixme child_5.text  = Some(entry.install_date);
        //fixme child_6.text  = Some(entry.last_boot_up_time);
        //fixme child_7.text  = Some(entry.locale);
        child_8.text  = Some(entry.manufacturer);
        child_9.text  = Some(entry.name);
        //fixme child_10.text = Some(entry.os_type);
        child_11.text = Some(entry.size_stored_in_paging_file);
        child_12.text = Some(entry.total_virtual_mem_size);
        child_13.text = Some(entry.total_visible_mem_size);
        child_14.text = Some(entry.version);
        child_15.text = Some(entry.win_directory);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        //parent.children.push(child_5);
        //parent.children.push(child_6);
        //parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);
        parent.children.push(child_13);
        parent.children.push(child_14);
        parent.children.push(child_15);

        root.children.push(parent);
    }

    let mut parent_motherboards = Element::new("Motherboards");
    for entry in WmiMotherboard::get_specific() {
        let mut parent = Element::new("RemoteMotherboard");

        let mut child_1 = Element::new("Manufacturer");
        let mut child_2 = Element::new("Name");
        let mut child_3 = Element::new("Product");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("Version");

        child_1.text = Some(entry.manufacturer);
        child_2.text = Some(entry.name);
        child_3.text = Some(entry.product);
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);

        parent_motherboards.children.push(parent);
    }
    root.children.push(parent_motherboards);

    let mut parent_processors = Element::new("Processors");
    for entry in WmiProcessor::get_specific() {
        let mut parent = Element::new("RemoteProcessor");

        let mut child_1  = Element::new("AddressWidth");
        let mut child_2  = Element::new("Architecture");
        let mut child_3  = Element::new("CpuStatus");
        let mut child_4  = Element::new("CurrentClockSpeed");
        let mut child_5  = Element::new("CurrentVoltage");
        let mut child_6  = Element::new("Description");
        let mut child_7  = Element::new("ExtClock");
        let mut child_8  = Element::new("L2CacheSize");
        let mut child_9  = Element::new("L3CacheSize");
        let mut child_10 = Element::new("Manufacturer");
        let mut child_11 = Element::new("MaxClockSpeed");
        let mut child_12 = Element::new("Name");
        let mut child_13 = Element::new("NumberOfCores");
        let mut child_14 = Element::new("NumberOfLogicalProcessors");
        let mut child_15 = Element::new("SocketDesignation");

        child_1.text  = Some(entry.address_width.to_string());
        //fixme child_2.text  = Some(entry.architecture);
        //fixme child_3.text  = Some(entry.cpu_satus);
        child_4.text  = Some(entry.current_clock_speed.to_string());
        child_5.text  = Some(entry.current_voltage.to_string());
        child_6.text  = Some(entry.description);
        child_7.text  = Some(entry.external_clock.to_string());
        child_8.text  = Some(entry.l2_cache_size.to_string());
        child_9.text  = Some(entry.l3_cache_size.to_string());
        child_10.text = Some(entry.manufacturer);
        child_11.text = Some(entry.max_clock_speed.to_string());
        child_12.text = Some(entry.name);
        child_13.text = Some(entry.number_of_cores.to_string());
        child_14.text = Some(entry.number_of_logical_processors.to_string());
        child_15.text = Some(entry.socket_designation);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);
        parent.children.push(child_13);
        parent.children.push(child_14);
        parent.children.push(child_15);

        parent_processors.children.push(parent);
    }
    root.children.push(parent_processors);

    let mut parent_memory = Element::new("Memory");
    for entry in WmiMemory::get_specific() {
        let mut parent = Element::new("RemoteMemory");

        let mut child_1  = Element::new("Capacity");
        let mut child_2  = Element::new("Description");
        let mut child_3  = Element::new("DeviceLocator");
        let mut child_4  = Element::new("FormFactor");
        let mut child_5  = Element::new("InterleaveDataDepth");
        let mut child_6  = Element::new("InterleavePosition");
        let mut child_7  = Element::new("Manufacturer");
        let mut child_8  = Element::new("MemoryType");
        let mut child_9  = Element::new("Name");
        let mut child_10 = Element::new("SerialNumber");
        let mut child_11 = Element::new("Speed");

        child_1.text  = Some(entry.capacity                             );
        child_2.text  = Some(entry.description                          );
        child_3.text  = Some(entry.device_locator                       );
        child_4.text  = Some(entry.form_factor.to_string()              );
        child_5.text  = Some(entry.interleave_data_depth.to_string()    );
        child_6.text  = Some(entry.interleave_position.to_string()      );
        child_7.text  = Some(entry.manufacturer                         );
        child_8.text  = Some(entry.memory_type.to_string()              );
        child_9.text  = Some(entry.name                                 );
        child_10.text = Some(entry.serial_number                        );
        child_11.text = Some(entry.speed.to_string()                    );

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);

        parent_memory.children.push(parent);
    }
    root.children.push(parent_memory);

    let mut parent_sound = Element::new("SoundDevice");
    for entry in WmiSound::get_specific() {
        let mut parent = Element::new("RemoteSound");

        let mut child_1  = Element::new("Manufacturer");
        let mut child_2  = Element::new("Name");

        child_1.text  = Some(entry.manufacturer   );
        child_2.text  = Some(entry.name           );

        parent.children.push(child_1);
        parent.children.push(child_2);

        parent_sound.children.push(parent);
    }
    root.children.push(parent_sound);

    let mut parent_video = Element::new("Videos");
    for entry in WmiVideo::get_specific() {
        let mut parent = Element::new("RemoteVideo");

        let mut child_1 = Element::new("AdapterCompatibility");
        let mut child_2 = Element::new("AdapterDACType");
        let mut child_3 = Element::new("AdapterRam");
        let mut child_4 = Element::new("Availability");
        let mut child_5 = Element::new("DriverVersion");
        let mut child_6 = Element::new("InstalledDisplayDrivers");
        let mut child_7 = Element::new("Name");
        let mut child_8 = Element::new("RefreshRate");
        let mut child_9 = Element::new("ScreenInfo");
        let mut child_10 = Element::new("Status");
        let mut child_11 = Element::new("VideoArchitecture");
        let mut child_12 = Element::new("VideoMemoryType");

        child_1.text = Some(entry.adapter_compatibility);
        child_2.text = Some(entry.adapter_dac_type);
        child_3.text = Some(entry.adapter_ram.to_string());
        //fixme child_4.text = Some(entry.availability);
        child_5.text = Some(entry.driver_version);
        for driver in entry.installed_display_driver {
            let mut string = Element::new("string");
            string.text = Some(driver);
            child_6.children.push(string);
        }
        child_7.text = Some(entry.name);
        child_8.text = Some(entry.refresh_rate);
        child_9.text = Some(entry.screen_info);
        child_10.text = Some(entry.status);
        //fixme child_11.text = Some(entry.video_architecture);
        //fixme child_12.text = Some(entry.video_memory_type);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
        parent.children.push(child_6);
        parent.children.push(child_7);
        parent.children.push(child_8);
        parent.children.push(child_9);
        parent.children.push(child_10);
        parent.children.push(child_11);
        parent.children.push(child_12);

        parent_video.children.push(parent);
    }
    root.children.push(parent_video);

    let mut parent_devices = Element::new("Devices");
    let mut parent_keyboards = Element::new("Keyboards");
    for entry in WmiKeyboard::get_specific() {
        let mut parent = Element::new("RemoteKeyboard");

        let mut child_1  = Element::new("Description");
        let mut child_2  = Element::new("Name");

        child_1.text  = Some(entry.description);
        child_2.text  = Some(entry.name);

        parent.children.push(child_1);
        parent.children.push(child_2);

        parent_keyboards.children.push(parent);
    }
    parent_devices.children.push(parent_keyboards);

    let mut parent_pointing_dev = Element::new("PointingDevices");
    for entry in WmiPointingDevice::get_specific() {
        let mut parent = Element::new("RemotePointingDevice");

        let mut child_1  = Element::new("Description");
        let mut child_2  = Element::new("Manufacturer");
        let mut child_3  = Element::new("Name");

        child_1.text  = Some(entry.description);
        child_2.text  = Some(entry.manufacturer);
        child_2.text  = Some(entry.name);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);

        parent_pointing_dev.children.push(parent);
    }
    parent_devices.children.push(parent_pointing_dev);
    root.children.push(parent_devices);
}

