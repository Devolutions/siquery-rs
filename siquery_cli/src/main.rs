#[macro_use]
extern crate clap;

extern crate siquery;

use clap::App;

use siquery::SystemInfo;
use siquery::sys::SystemReader;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();

    let system_reader = Box::new(SystemReader::new());
    let system_info = SystemInfo::new(system_reader);

    let table = matches.value_of("table").unwrap_or("").to_string();

    match table.as_str() {
        "os_version" => {
            println!("Operating System");
            println!("  Platform: {} ", system_info.os_version.platform);
            println!("  Name: {}", system_info.os_version.name);
            println!(
                "  Version: {} Major: {} Minor: {}",
                system_info.os_version.version, system_info.os_version.major, system_info.os_version.minor
            );
        },
        "system_info" => {
            println!("Cpu brand: {}", system_info.system_info.cpu_brand);
            println!("Cpu cores: {}", system_info.system_info.cpu_logical_cores);
            println!(
                "Physical memory: {} bytes",
                system_info.system_info.physical_memory
            );
        },
        "logical_drives" => {
            println!("Logical Drives");
            for drive in &system_info.logical_drives {
                println!("  Name: {}", drive.device_id);
                println!("  File system: {}", drive.file_system);
                println!("  Size: {}", drive.size);
                println!("  Free space: {}", drive.free_space);
            }
        },
        "etc_hosts" => {
            for hosts in &system_info.etc_hosts {
                println!("{:?}", &hosts);
            }
        },
        "etc_protocols" => {
            for protocol_entries in &system_info.etc_protocols {
                println!("{:?}", protocol_entries);
            }
        },
        "etc_services" => {
            for services_entries in &system_info.etc_services {
                println!("{:?}", services_entries);
            }
        },
        "uptime" => {
            println!("{:?}", &system_info.uptime);
        }
        _ => {

        }
    }
}
