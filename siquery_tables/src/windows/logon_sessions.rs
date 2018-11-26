#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use tables::LogonSessions;
use winapi::{
    um::{
        ntlsa::*,
        winnt::{PLUID, LUID, PWSTR, LPWSTR, PSID, LARGE_INTEGER        }
    },
    shared::{minwindef::DWORD,
             sddl::ConvertSidToStringSidW}
};
use winapi::um::errhandlingapi::GetLastError;
use widestring::WideString;
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
        let mut _sessions: *mut PLUID = ptr::null_mut();
        let mut session_array: Vec<LUID> = Vec::with_capacity((mem::size_of::<LUID>()) as usize);
        let psession_array = session_array.as_mut_ptr();
        _sessions = psession_array as *mut _;

        let mut _status: i32 = 0;
        _status = LsaEnumerateLogonSessions(session_count, _sessions);

        if _status == kLsaStatusSuccess {
            _sessions = ptr::null_mut();
            let mut session_array_sized: Vec<LUID> = Vec::with_capacity(session_count_int as usize);
            let psession_array_sized = session_array_sized.as_mut_ptr();
            _sessions = psession_array_sized as *mut _;

            _status = LsaEnumerateLogonSessions(session_count, _sessions);

            if _status == kLsaStatusSuccess {
                for _i in 0..session_count_int {

                    let mut _session_data: *mut PSECURITY_LOGON_SESSION_DATA = ptr::null_mut();
                    let mut session_data_struct: *mut SECURITY_LOGON_SESSION_DATA = mem::uninitialized();
                    _session_data = session_data_struct as *mut _;

                    _status = LsaGetLogonSessionData(*_sessions, _session_data);
                    if _status != kLsaStatusSuccess {
                        *_sessions = (*_sessions).add(1);
                        continue;
                    }

                    let mut logon_session = LogonSessions::new();

                    logon_session.logon_id = (**_session_data).LogonId.LowPart as i32;

                    logon_session.user =  pwstr_to_string((**_session_data).UserName.Buffer,
                                                          (**_session_data).UserName.Length).unwrap_or("".to_string());

                    logon_session.logon_domain = pwstr_to_string((**_session_data).LogonDomain.Buffer,
                                                                 (**_session_data).LogonDomain.Length).unwrap_or("".to_string());

                    logon_session.authentication_package = pwstr_to_string((**_session_data).AuthenticationPackage.Buffer,
                                                                           (**_session_data).AuthenticationPackage.Length).unwrap_or("".to_string());
                    logon_session.logon_type = logon_type_to_string((**_session_data).LogonType);

                    logon_session.session_id = (**_session_data).Session as i32;

                    logon_session.logon_sid = sid_to_string((**_session_data).Sid).unwrap_or("".to_string());

                    logon_session.logon_time = long_int_to_unixtime(&mut (**_session_data).LogonTime);

                    logon_session.logon_server = pwstr_to_string((**_session_data).LogonServer.Buffer,
                                                                 (**_session_data).LogonServer.Length).unwrap_or("".to_string());

                    logon_session.dns_domain_name = pwstr_to_string((**_session_data).DnsDomainName.Buffer,
                                                                    (**_session_data).DnsDomainName.Length).unwrap_or("".to_string());

                    logon_session.upn = pwstr_to_string((**_session_data).Upn.Buffer,
                                                        (**_session_data).Upn.Length).unwrap_or("".to_string());

                    logon_session.logon_script = pwstr_to_string((**_session_data).LogonScript.Buffer,
                                                                 (**_session_data).LogonScript.Length).unwrap_or("".to_string());

                    logon_session.profile_path = pwstr_to_string((**_session_data).ProfilePath.Buffer,
                                                                 (**_session_data).ProfilePath.Length).unwrap_or("".to_string());

                    logon_session.home_directory = pwstr_to_string((**_session_data).HomeDirectory.Buffer,
                                                                   (**_session_data).HomeDirectory.Length).unwrap_or("".to_string());

                    logon_session.home_directory_drive = pwstr_to_string((**_session_data).HomeDirectoryDrive.Buffer,
                                                                         (**_session_data).HomeDirectoryDrive.Length).unwrap_or("".to_string());

                    *_sessions = (*_sessions).add(1);
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

pub fn long_int_to_unixtime(ft: &mut LARGE_INTEGER ) -> i64  {
    unsafe {
        let mut ull: LARGE_INTEGER = mem::uninitialized();
        let mut adjust: LARGE_INTEGER = mem::uninitialized();

        ull.u_mut().LowPart = ft.u_mut().LowPart;
        ull.u_mut().HighPart = ft.u_mut().HighPart;

        *adjust.QuadPart_mut() = 11644473600000 * 10000;
        *ull.QuadPart_mut() -= *adjust.QuadPart_mut();

        return *ull.QuadPart_mut() / 10000000;
    }
}