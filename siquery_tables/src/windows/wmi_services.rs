use tables::WmiServices;
use utils;
use windows::SystemReaderInterface;

impl WmiServices {
    pub(crate) fn new() -> WmiServices {
        WmiServices {
            accept_pause: String::new(),
            accept_stop: String::new(),
            caption: String::new(),
            creation_class_name: String::new(),
            description: String::new(),
            desktop_interact: String::new(),
            display_name: String::new(),
            error_control: String::new(),
            exit_code: 0,
            name: String::new(),
            path_name: String::new(),
            service_type: String::new(),
            started: String::new(),
            start_mode: String::new(),
            start_name: String::new(),
            state: String::new(),
            status: String::new(),
            system_creation_class_name: String::new(),
            system_name: String::new(),
        }
    }

    pub(crate) fn get_services_info(system_reader: &SystemReaderInterface) -> Vec<WmiServices> {

        let mut services: Vec<WmiServices> = Vec::new();

        if let Some(service_info) = system_reader.get_wmi_services_info() {
            let mut service = WmiServices::new();
            let lines = service_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if service.accept_pause != "" {
                        services.push(service);
                    }
                    service = WmiServices::new();
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
                    "AcceptPause" => {
                        service.accept_pause = v;
                    },
                    "AcceptStop" => {
                        service.accept_stop = v;
                    },
                    "Caption" => {
                        service.caption = v;
                    },
                    "CreationClassName" => {
                        service.creation_class_name = v;
                    },
                    "Description" => {
                        service.description = v;
                    },
                    "DesktopInteract" => {
                        service.desktop_interact = v;
                    },
                    "DisplayName" => {
                        service.display_name = v;
                    },
                    "ErrorControl" => {
                        service.error_control = v;
                    },
                    "ExitCode" => {
                        service.exit_code = v.parse::<u32>().unwrap_or(0);
                    },
                    "Name" => {
                        service.name = v;
                    },
                    "PathName" => {
                        service.path_name = v;
                    },
                    "ServiceType" => {
                        service.service_type = v;
                    },
                    "Started" => {
                        service.started = v;
                    },
                    "StartMode" => {
                        service.start_mode = v;
                    },
                    "StartName" => {
                        service.start_name = v;
                    },
                    "State" => {
                        service.state = v;
                    },
                    "Status" => {
                        service.status = v;
                    },
                    "SystemCreationClassName" => {
                        service.system_creation_class_name = v;
                    },
                    "SystemName" => {
                        service.system_name = v;
                    },
                    _ => ()
                }
            }
        }

        services
    }
}