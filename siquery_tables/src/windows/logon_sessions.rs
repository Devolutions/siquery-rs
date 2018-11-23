use tables::LogonSessions;
use winapi::um::ntlsa::*;
use winapi::um::winnt::PLUID;
use winapi::um::winnt::LUID;
use std::{ptr, mem};

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
        get_logon_sessions(&mut logon_sessions);
        logon_sessions
    }
}

fn get_logon_sessions(logon_sessions: &mut Vec<LogonSessions>) {
    unsafe {
        let kLsaStatusSuccess: i32 = 0;

        let mut session_count_int = 0u32;
        let session_count: *mut u32 = &mut session_count_int as *mut u32;

        // get sessions size
        let mut sessions: *mut PLUID = ptr::null_mut();
        let mut session_array: Vec<u16> = Vec::with_capacity((mem::size_of::<LUID>()) as usize);
        let psession_array = session_array.as_mut_ptr();
        sessions = psession_array as *mut _;


        let mut status: i32 = 0;
        status = LsaEnumerateLogonSessions(session_count, sessions);

        if status == kLsaStatusSuccess {
            /*sessions = ptr::null_mut();*/
            let mut sessions_sized: *mut PLUID = ptr::null_mut();
            let mut session_array_sized: Vec<LUID> = Vec::with_capacity(session_count_int as usize);
            let psession_array_sized = session_array_sized.as_mut_ptr();
            sessions_sized = psession_array_sized as *mut _;

            status = LsaEnumerateLogonSessions(session_count, sessions_sized);


            if status == kLsaStatusSuccess {
                for i in 0..session_count_int {
                    let mut session_data: *mut PSECURITY_LOGON_SESSION_DATA = ptr::null_mut();
                    let mut session_data_array: Vec<u16> = Vec::with_capacity((mem::size_of::<SECURITY_LOGON_SESSION_DATA>()) as usize);
                    let psession_data_array = session_data_array.as_mut_ptr();
                    session_data = psession_data_array as *mut _;

                    status = LsaGetLogonSessionData(*sessions_sized, session_data);
                    println!("logon_id {:?}", (**session_data).LogonId.LowPart);
                    sessions_sized = sessions_sized.add(1);
                }
            }
        }
    }
}