use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use rusqlite::{Rows, Row as RusqliteRow, types::{Value, Type}};

use tables::*;

pub fn print_xml (col_names: &Vec<String>, rows: &mut Rows) {
    // RDM Structures.
    let mut root = Element::new("InventorySystemInformation");

    let mut bios = bios();
    let mut cd_roms = cd_roms();
    let mut devices = devices();
    let mut ip_address = ip_address();
    let mut local_accounts = local_accounts();
    let mut logical_drives = logical_drives();
    let mut memory = memory();
    let mut monitors = monitors();
    let mut motherobaords = motherboards();
    let mut network_adapters = network_adapters();








    let mut operating_system = Element::new("OperatingSystem");

    let mut printers = Element::new("Printers");
    let mut remote_printer = Element::new("RemotePrinter");

    let mut processors = Element::new("Processors");
    let mut remote_processors = Element::new("RemoteProcessors");

    let mut products = Element::new("Products");
    let mut remote_program = Element::new("RemoteProgram");

    let mut quick_fix_engineerings = Element::new("QuickFixEngineerings");
    let mut remote_quick_fix_engineering = Element::new("RemoteQuickFixEngineering");

    let mut services = Element::new("Services");
    let mut remote_service = Element::new("RemoteService");

    let mut shares = Element::new("Shares");
    let mut remote_share = Element::new("RemoteShare");

    let mut sound_devices = Element::new("SoundDevice");
    let mut remote_sound = Element::new("RemoteSound");

    let mut start_ups = Element::new("StartUps");
    let mut remote_start_up = Element::new("RemoteStartUp");

    let mut system = Element::new("System");

    let mut time_zone = Element::new("TimeZone");
    let mut description = Element::new("Description");

    let mut videos = Element::new("Videos");
    let mut remote_video = Element::new("RemoteVideos");


    loop {
        if let Some(row) = rows.next() {
            if let Some(row_data) = row.ok() {
                let mut remote_account = Element::new("RemoteAccount");
                for i in 0..(row_data.column_count()-1) {
                    let mut col = Element::new(col_names.get(i).unwrap().to_camel_case());
                    match Value::data_type(&row_data.get_checked(i).unwrap()) {
                        Type::Text => {
                            col.text = Some(row_data.get_checked::<usize, String>(i).unwrap());
                            remote_account.children.push(col);
                        },
                        Type::Real | Type::Integer => {
                            col.text = Some(row_data.get_checked::<usize, i64>(i).unwrap().to_string());
                            remote_account.children.push(col);
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                local_accounts.children.push(remote_account);
            }
        } else {
            break
        }
    }
    root.children.push(local_accounts);
    //}

    root.children.push(bios);
    root.children.push(cd_roms);
    root.children.push(devices);
    root.children.push(ip_address);
    //root.children.push(local_accounts);
    root.children.push(logical_drives);
    root.children.push(memory);
    root.children.push(monitors);
    root.children.push(motherobaords);
    root.children.push(network_adapters);

    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    println!("{}",doc.to_string());
}

fn bios() -> Element {
    let bios = WmiBios::get_specific();

    let mut parent = Element::new("Bios");

    for entry in bios {
        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Manufacturer");
        let mut child_3 = Element::new("ReleaseDate");
        let mut child_4 = Element::new("SerialNumber");
        let mut child_5 = Element::new("SMBIOSBIOSVersion");

        child_1.text = Some(entry.caption);
        child_2.text = Some(entry.manufacturer);
        child_3.text = Some(entry.release_date);
        child_4.text = Some(entry.serial_number);
        child_5.text = Some(entry.smbios_version);

        parent.children.push(child_1);
        parent.children.push(child_2);
        parent.children.push(child_3);
        parent.children.push(child_4);
        parent.children.push(child_5);
    }

    parent
}

fn cd_roms() -> Element {
    let mut cd_roms = Element::new("CDRoms");

    cd_roms
}

fn devices() -> Element {
    let mut devices = Element::new("Devices");

    let mut keyboards = Element::new("Keyboards");
    let mut remote_keyboard = Element::new("RemoteKeyboard");
    let mut remote_keyboard_description = Element::new("Description");
    let mut remote_keyboard_name = Element::new("Name");

    remote_keyboard.children.push(remote_keyboard_description);
    remote_keyboard.children.push(remote_keyboard_name);
    keyboards.children.push(remote_keyboard);
    devices.children.push(keyboards);

    let mut pointing_devices = Element::new("PointingDevices");
    let mut remote_pointing_device = Element::new("RemotePointingDevice");
    let mut remote_pointing_device_description = Element::new("Description");
    let mut remote_pointing_device_manufacturer = Element::new("Manufacturer");
    let mut remote_pointing_device_name = Element::new("Name");

    remote_pointing_device.children.push(remote_pointing_device_description);
    remote_pointing_device.children.push(remote_pointing_device_manufacturer);
    remote_pointing_device.children.push(remote_pointing_device_name);
    pointing_devices.children.push(remote_pointing_device);
    devices.children.push(pointing_devices);

    devices
}

fn ip_address() -> Element {
    let mut ip_address = Element::new("IPAddress");

    ip_address
}

fn local_accounts() -> Element {
    let wmi_local_accounts = WmiLocalAccounts::get_specific();

    let mut local_accounts = Element::new("LocalAccounts");

    let mut remote_account = Element::new("RemoteAccount");
    let mut child_1 = Element::new("Caption");
    let mut child_2 = Element::new("Domain");
    let mut child_3 = Element::new("LocalAccount");
    let mut child_4 = Element::new("Name");
    let mut child_6 = Element::new("Status");

    for local_account in wmi_local_accounts {
        let mut child_5 = Element::new("SID");
        child_5.text = Some(local_account.sid);
        remote_account.children.push(child_5);
    }

    /*loop {
        if let Some(row) = users.next() {
            if let Some(row_data) = row.ok() {
                let mut remote_account = Element::new("RemoteAccount");
                for i in 0..(row_data.column_count()-1) {
                    let mut col = Element::new(col_names.get(i).unwrap().to_camel_case());
                    match Value::data_type(&row_data.get_checked(i).unwrap()) {
                        Type::Text => {
                            col.text = Some(row_data.get_checked::<usize, String>(i).unwrap());
                            remote_account.children.push(col);
                        },
                        Type::Real | Type::Integer => {
                            col.text = Some(row_data.get_checked::<usize, i64>(i).unwrap().to_string());
                            remote_account.children.push(col);
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                local_accounts.children.push(remote_account);
            }
        } else {
            break
        }
    }*/




    remote_account.children.push(child_1);
    remote_account.children.push(child_2);
    remote_account.children.push(child_3);
    remote_account.children.push(child_4);
    remote_account.children.push(child_6);
    local_accounts.children.push(remote_account);

    local_accounts
}

fn logical_drives() -> Element {
    let mut logical_drives = Element::new("LogicalDrives");

    let mut remote_logical_disk = Element::new("RemoteLogicalDisk");
    let mut child_1 = Element::new("Description");
    let mut child_2 = Element::new("DriveType");
    let mut child_3 = Element::new("FileSystem");
    let mut child_4 = Element::new("FreeSpace");
    let mut child_5 = Element::new("MaximumComponentLength");
    let mut child_6 = Element::new("Name");
    let mut child_7 = Element::new("Size");
    let mut child_8 = Element::new("SupportsFileBasedCompression");
    let mut child_9 = Element::new("VolumeSerialNumber");

    remote_logical_disk.children.push(child_1);
    remote_logical_disk.children.push(child_2);
    remote_logical_disk.children.push(child_3);
    remote_logical_disk.children.push(child_4);
    remote_logical_disk.children.push(child_5);
    remote_logical_disk.children.push(child_6);
    remote_logical_disk.children.push(child_7);
    remote_logical_disk.children.push(child_8);
    remote_logical_disk.children.push(child_9);

    logical_drives.children.push(remote_logical_disk);

    logical_drives
}

fn memory() -> Element {
    let mut memory = Element::new("Memory");

    let mut remote_memory = Element::new("RemoteMemory");
    let mut child_1 = Element::new("Capacity");
    let mut child_2 = Element::new("Description");
    let mut child_3 = Element::new("DeviceLocator");
    let mut child_4 = Element::new("FormFactor");
    let mut child_5 = Element::new("Manufacturer");
    let mut child_6 = Element::new("MemoryType");
    let mut child_7 = Element::new("Name");
    let mut child_8 = Element::new("SerialNumber");
    let mut child_9 = Element::new("VolumeSerialNumber");

    remote_memory.children.push(child_1);
    remote_memory.children.push(child_2);
    remote_memory.children.push(child_3);
    remote_memory.children.push(child_4);
    remote_memory.children.push(child_5);
    remote_memory.children.push(child_6);
    remote_memory.children.push(child_7);
    remote_memory.children.push(child_8);
    remote_memory.children.push(child_9);

    memory.children.push(remote_memory);

    memory
}

fn monitors() -> Element {
    let mut monitors = Element::new("Monitors");

    let mut remote_monitor = Element::new("RemoteMonitor");
    let mut child_1 = Element::new("Availability");
    let mut child_2 = Element::new("Manufacturer");
    let mut child_3 = Element::new("Name");
    let mut child_4 = Element::new("ScreenHeight");
    let mut child_5 = Element::new("ScreenWidth");

    remote_monitor.children.push(child_1);
    remote_monitor.children.push(child_2);
    remote_monitor.children.push(child_3);
    remote_monitor.children.push(child_4);
    remote_monitor.children.push(child_5);

    monitors.children.push(remote_monitor);

    monitors
}

fn motherboards() -> Element {
    let mut motherboards = Element::new("Motherboards");

    let mut remote_motherboards = Element::new("RemoteMotherboard");
    let mut child_1 = Element::new("Manufacturer");
    let mut child_2 = Element::new("Name");
    let mut child_3 = Element::new("Product");
    let mut child_4 = Element::new("SerialNumber");
    let mut child_5 = Element::new("Version");

    remote_motherboards.children.push(child_1);
    remote_motherboards.children.push(child_2);
    remote_motherboards.children.push(child_3);
    remote_motherboards.children.push(child_4);
    remote_motherboards.children.push(child_5);

    motherboards.children.push(remote_motherboards);

    motherboards
}

fn network_adapters() -> Element {
    let mut network_adapters = Element::new("NetworkAdapters");

    let mut remote_network_adapters = Element::new("RemoteNetworkAdapter");
    let mut child_1 = Element::new("Ports");
    let mut child_2 = Element::new("DatabasePath");
    let mut child_3 = Element::new("Description");
    let mut child_4 = Element::new("DHCPEnabled");
    let mut child_5 = Element::new("IPAddress");
    let mut child_6 = Element::new("IPAddressMac");
    let mut child_7 = Element::new("IPEnabled");
    let mut child_8 = Element::new("IPSubnet");
    let mut child_9 = Element::new("MACAddress");

    remote_network_adapters.children.push(child_1);
    remote_network_adapters.children.push(child_2);
    remote_network_adapters.children.push(child_3);
    remote_network_adapters.children.push(child_4);
    remote_network_adapters.children.push(child_5);
    remote_network_adapters.children.push(child_6);
    remote_network_adapters.children.push(child_7);
    remote_network_adapters.children.push(child_8);
    remote_network_adapters.children.push(child_9);

    network_adapters.children.push(remote_network_adapters);

    network_adapters
}