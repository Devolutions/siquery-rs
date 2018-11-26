use tables::LogonSessions;
use winapi::um::ntlsa::*;
use winapi::um::winnt::PLUID;
use winapi::um::winnt::LUID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::PWSTR;
use winapi::um::winnt::LPWSTR;
use winapi::um::winnt::PSID;
use widestring::WideString;
use winapi::shared::minwindef::DWORD;
use winapi::shared::sddl::ConvertSidToStringSidW;
use std::{ptr, mem};
use libc;

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

        // get sessions array size
        let mut sessions: *mut PLUID = ptr::null_mut();
        let mut session_array: Vec<LUID> = Vec::with_capacity((mem::size_of::<LUID>()) as usize);
        let psession_array = session_array.as_mut_ptr();
        sessions = psession_array as *mut _;

        let mut status: i32 = 0;
        status = LsaEnumerateLogonSessions(session_count, sessions);

        if status == kLsaStatusSuccess {
            sessions = ptr::null_mut();
            let mut session_array_sized: Vec<LUID> = Vec::with_capacity(session_count_int as usize);
            let psession_array_sized = session_array_sized.as_mut_ptr();
            sessions = psession_array_sized as *mut _;

            status = LsaEnumerateLogonSessions(session_count, sessions);

            if status == kLsaStatusSuccess {
                for i in 0..session_count_int {

                    let mut session_data: *mut PSECURITY_LOGON_SESSION_DATA = ptr::null_mut();
                    let mut session_data_array: Vec<SECURITY_LOGON_SESSION_DATA> = Vec::with_capacity(10);
                    let psession_data_array = session_data_array.as_mut_ptr();
                    session_data = psession_data_array as *mut _;

                    status = LsaGetLogonSessionData(*sessions, session_data);

                    if status != kLsaStatusSuccess {
                        *sessions = (*sessions).add(1);
                        continue;
                    }

                    let mut logon_session = LogonSessions::new();

                    logon_session.logon_id = (**session_data).LogonId.LowPart as i32;
                    logon_session.user =  pwstr_to_string((**session_data).UserName.Buffer,
                                                          (**session_data).UserName.Length).unwrap_or("".to_string());
                    logon_session.logon_domain = pwstr_to_string((**session_data).LogonDomain.Buffer,
                                                                 (**session_data).LogonDomain.Length).unwrap_or("".to_string());
                    logon_session.authentication_package = pwstr_to_string((**session_data).AuthenticationPackage.Buffer,
                                                                           (**session_data).AuthenticationPackage.Length).unwrap_or("".to_string());
                    logon_session.logon_type = logon_type_to_string((**session_data).LogonType);
                    logon_session.session_id = (**session_data).Session as i32;
                    logon_session.logon_sid = sid_to_string((**session_data).Sid).unwrap_or("".to_string());

                    // todo implement longIntToUnixtime
                    //logon_session.logon_time =

                    logon_session.logon_server = pwstr_to_string((**session_data).LogonServer.Buffer,
                                                                 (**session_data).LogonServer.Length).unwrap_or("".to_string());

                    logon_session.dns_domain_name = pwstr_to_string((**session_data).DnsDomainName.Buffer,
                                                                    (**session_data).DnsDomainName.Length).unwrap_or("".to_string());

                    logon_session.upn = pwstr_to_string((**session_data).Upn.Buffer,
                                                        (**session_data).Upn.Length).unwrap_or("".to_string());

                    logon_session.logon_script = pwstr_to_string((**session_data).LogonScript.Buffer,
                                                                 (**session_data).LogonScript.Length).unwrap_or("".to_string());

                    logon_session.profile_path = pwstr_to_string((**session_data).ProfilePath.Buffer,
                                                                 (**session_data).ProfilePath.Length).unwrap_or("".to_string());

                    logon_session.home_directory = pwstr_to_string((**session_data).HomeDirectory.Buffer,
                                                                   (**session_data).HomeDirectory.Length).unwrap_or("".to_string());

                    logon_session.home_directory_drive = pwstr_to_string((**session_data).HomeDirectoryDrive.Buffer,
                                                                         (**session_data).HomeDirectoryDrive.Length).unwrap_or("".to_string());

                    *sessions = (*sessions).add(1);
                    logon_sessions.push(logon_session);
                }
            }
        }
    }
}

pub fn logon_type_to_string(logon_type : u32) -> String {
    match logon_type {
        Interactive => {
            return "Interactive".to_string();
        },
        Network => {
            return "Interactive".to_string();
        },
        Batch => {
            return "Interactive".to_string();
        },
        Proxy => {
            return "Interactive".to_string();
        },
        Unlock => {
            return "Interactive".to_string();
        },
        NetworkCleartext => {
            return "Interactive".to_string();
        },
        NewCredentials => {
            return "Interactive".to_string();
        },
        RemoteInteractive => {
            return "Interactive".to_string();
        },
        CachedInteractive => {
            return "Interactive".to_string();
        },
        CachedRemoteInteractive => {
            return "Interactive".to_string();
        },
        CachedUnlock => {
            return "Interactive".to_string();
        },
        _ => return "".to_string()
    }
}

pub fn sid_to_string(sid: PSID) -> Result<String, DWORD> {
    let mut buf: LPWSTR = ptr::null_mut() as LPWSTR;
    unsafe {
        if ConvertSidToStringSidW(sid, &mut buf) == 0 ||
            buf == (ptr::null_mut() as LPWSTR) {
            return Err(GetLastError());
        }
        lpwstr_to_string(buf)
    }
}

pub fn lpwstr_to_string(lpwstr: LPWSTR) -> Result<String, DWORD> {
    let buf_size = unsafe { libc::wcslen(lpwstr) };
    let string = unsafe { WideString::from_ptr(lpwstr, buf_size) };
    Ok(string.to_string_lossy())
}

// convert LSA_UNICODE_STRING to rust string
pub fn pwstr_to_string(pwstr: PWSTR, buf_size: u16) -> Result<String, DWORD> {
    let string = unsafe { WideString::from_ptr(pwstr, buf_size as usize / 2) };
    Ok(string.to_string_lossy())
}