use std::process::Command;
use std::borrow::Borrow;

use crate::tables::{WmiServices,WmiServicesIface};
use crate::utils;

pub struct Reader {}
impl WmiServicesIface for Reader {
    fn get_wmi_services_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["service",
                "get",
                "AcceptPause,AcceptStop,Caption,CreationClassName,Description,DesktopInteract,\
                DisplayName,ErrorControl,ExitCode,Name,PathName,ServiceType,Started,StartMode,\
                StartName,State,Status,SystemCreationClassName,SystemName",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

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

    pub fn get_specific_ex (reader: &dyn WmiServicesIface) -> Vec<WmiServices> {

        let mut output: Vec<WmiServices> = Vec::new();

        if let Some(service_info) = reader.get_wmi_services_info() {
            let mut service = WmiServices::new();
            let lines = service_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if service.accept_pause != "" {
                        output.push(service);
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
                        service.accept_pause = v.to_lowercase();
                    },
                    "AcceptStop" => {
                        service.accept_stop = v.to_lowercase();
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
                        service.desktop_interact = v.to_lowercase();
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
                        service.started = v.to_lowercase();
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

        output
    }

    pub(crate) fn get_specific () -> Vec<WmiServices> {
        let reader: Box<dyn WmiServicesIface> = Box::new(Reader{});
        let out = WmiServices::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiServicesIface for Test {
        fn get_wmi_services_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-services.txt")))
        }
    }
    #[test]
    fn test_wmi_services () {
        let reader: Box<dyn WmiServicesIface> = Box::new(Test{});
        let test_services = &WmiServices::get_specific_ex(reader.borrow())[0];
        assert_eq!(test_services.accept_pause, "false");
        assert_eq!(test_services.accept_stop, "true");
        assert_eq!(test_services.caption, "Windows Push Notifications User Service_10b2b340");
        assert_eq!(test_services.creation_class_name, "Win32_Service");
        assert_eq!(test_services.description, "do something");
        assert_eq!(test_services.desktop_interact, "false");
        assert_eq!(test_services.display_name, "Windows Push Notifications User Service_10b2b340");
        assert_eq!(test_services.error_control, "Ignore");
        assert_eq!(test_services.exit_code, 0);
        assert_eq!(test_services.name, "WpnUserService_10b2b340");
        assert_eq!(test_services.path_name, "C:\\WINDOWS\\system32\\svchost.exe -k UnistackSvcGroup");
        assert_eq!(test_services.service_type, "Unknown");
        assert_eq!(test_services.started, "true");
        assert_eq!(test_services.start_mode, "Auto");
        assert_eq!(test_services.start_name, "");
        assert_eq!(test_services.state, "Running");
        assert_eq!(test_services.status, "OK");
        assert_eq!(test_services.system_creation_class_name, "Win32_ComputerSystem");
        assert_eq!(test_services.system_name, "waka-waka");
    }
}