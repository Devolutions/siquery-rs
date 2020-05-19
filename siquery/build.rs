use std::env;

const WINDOWS_TABLES: &'static [&'static str] = &[
    "etc_hosts",
    "etc_protocols",
    "etc_services",
    "system_info",
    "os_version",
    "logical_drives",
    "uptime",
    "processes",
    "interface_address",
    "interface_details",
    "process_open_sockets",
    "process_memory_map",
    "products",
    "users",
    "logged_in_users",
    "logon_sessions",
    "groups",
    "wmi_computer_info",
    "wmi_os_version",
    "wmi_printers",
    "wmi_services",
    "wmi_products",
    "wmi_hotfixes",
    "wmi_shares",
    "wmi_network_adapters",
    "wmi_local_accounts",
    "wmi_bios",
    "wmi_motherboard",
    "wmi_processor",
    "wmi_physical_memory",
    "wmi_sound",
    "wmi_video",
    "wmi_monitors",
    "wmi_keyboard",
    "wmi_pointing_device",
    "wmi_start_up",
    "wmi_time_zone",
    "proxies"
];

const LINUX_TABLES: &'static [&'static str] = &[
    "etc_hosts",
    "etc_protocols",
    "etc_services",
    "interface_address",
    "interface_details",
    "system_info",
    "os_version",
    "uptime",
    "processes",
    "process_open_sockets",
    "process_memory_map",
    "process_envs",
    "mounts",
    "groups",
    "users",
    "logged_in_users",
    "proxies"
];

const MACOS_TABLES: &'static [&'static str] = &[
    "etc_hosts",
    "etc_protocols",
    "etc_services",
    "os_version",
    "processes",
    "process_envs",
    "system_info",
    "uptime",
    "mounts",
    "groups",
    "users",
    "proxies",
    "logged_in_users",
    "launchd",
    "launchd_overrides",
];

fn emit_features(features: &'static [&'static str]) {
    for feature in features.iter() {
        println!("cargo:rustc-cfg=feature=\"{}\"", feature);
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();

    if let Some(os) = target.get(2) {
        match os {
            &"windows" => {
                emit_features(WINDOWS_TABLES.clone());
            },
            &"linux"=> {
                emit_features(LINUX_TABLES.clone());
            },
            &"darwin"=> {
                emit_features(MACOS_TABLES.clone());
            },
            _ => {}
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
