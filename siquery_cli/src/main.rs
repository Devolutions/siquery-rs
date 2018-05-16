
extern crate siquery;

use siquery::SystemInfo;
use siquery::sys::SystemReader;

fn main() {
    let system_reader = Box::new(SystemReader::new());
    let system_info = SystemInfo::new(system_reader);
    println!("System Information");
    println!("Computer name: {}", system_info.system_info.computer_name);
    for hosts in &system_info.etc_hosts {
        println!("{:?}", &hosts);
    }
    for protocol_entries in &system_info.etc_protocols {
        println!("{:?}", protocol_entries);
    }
    for services_entries in &system_info.etc_services {
        println!("{:?}", services_entries);
    }
    println!("Operating System");
    println!("  Platform: {} ", system_info.os_version.platform);
    println!("  Name: {}", system_info.os_version.name);
    println!(
        "  Version: {} Major: {} Minor: {}",
        system_info.os_version.version, system_info.os_version.major, system_info.os_version.minor
    );

    println!("Cpu brand: {}", system_info.system_info.cpu_brand);
    println!("Cpu cores: {}", system_info.system_info.cpu_logical_cores);
    println!(
        "Physical memory: {} bytes",
        system_info.system_info.physical_memory
    );

    println!("Logical Drives");
    for drive in &system_info.logical_drives {
        println!("  Name: {}", drive.device_id);
        println!("  File system: {}", drive.file_system);
        println!("  Size: {}", drive.size);
        println!("  Free space: {}", drive.free_space);
    }

    println!("\n");
    println!("{}", system_info.to_json());
}

