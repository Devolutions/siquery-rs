use tables::LogonSessions;

impl LogonSessions {
    pub(crate) fn new() -> LogonSessions {
        LogonSessions {
            logon_id: 0,
            user: String::new(),
            logon_domain: String::new(),
            authentication_package: String::new(),
            logon_type: String::new(),
            session_id: 0,
            logon_sid: String::new(),
            logon_time: 0,
            logon_server: String::new(),
            dns_domain_name: String::new(),
            upn: String::new(),
            logon_script: String::new(),
            profile_path: String::new(),
            home_directory: String::new(),
            home_directory_drive: String::new(),
        }
    }

    pub fn get_specific() -> Vec<LogonSessions> {
        let mut logon_sessions: Vec<LogonSessions> = Vec::new();
        logon_sessions
    }
}