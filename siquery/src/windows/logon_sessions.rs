#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::tables::LogonSessions;
use winapi::{
    um::{
        ntlsa::*,
        winnt::{LUID, PWSTR, LPWSTR, PSID, LARGE_INTEGER}
    },
    shared::{minwindef::DWORD,ntstatus::*,
             sddl::ConvertSidToStringSidW}
};
use winapi::um::errhandlingapi::GetLastError;
use widestring::WideString;
use std::{ptr, mem};
use std::ffi::c_void;
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
        get_logon_sessions()
    }
}

fn get_logon_sessions() ->  Vec<LogonSessions> {
    let mut logon_sessions: Vec<LogonSessions> = Vec::new();
    unsafe {
        let mut session_count = 0u32;
        let mut _sessions: *mut LUID = ptr::null_mut();

        let mut _status = LsaEnumerateLogonSessions(&mut session_count as *mut u32, &mut _sessions);

        if _status == STATUS_SUCCESS {
            for _i in 0..session_count {

                let mut _session_data: PSECURITY_LOGON_SESSION_DATA = ptr::null_mut();

                _status = LsaGetLogonSessionData(_sessions.offset(_i as isize), &mut _session_data);

                if _status != STATUS_SUCCESS {
                    continue; /* STATUS_ACCESS_DENIED most of the time */
                }

                let mut logon_session = LogonSessions::new();

                logon_session.logon_id = (*_session_data).LogonId.LowPart as i32;

                logon_session.user =  pwstr_to_string((*_session_data).UserName.Buffer,
                                                      (*_session_data).UserName.Length).unwrap_or("".to_string());

                logon_session.logon_domain = pwstr_to_string((*_session_data).LogonDomain.Buffer,
                                                             (*_session_data).LogonDomain.Length).unwrap_or("".to_string());

                logon_session.authentication_package = pwstr_to_string((*_session_data).AuthenticationPackage.Buffer,
                                                                       (*_session_data).AuthenticationPackage.Length).unwrap_or("".to_string());
                logon_session.logon_type = logon_type_to_string((*_session_data).LogonType);

                logon_session.session_id = (*_session_data).Session as i32;

                logon_session.logon_sid = sid_to_string((*_session_data).Sid).unwrap_or("".to_string());

                logon_session.logon_time = long_int_to_unixtime(&mut (*_session_data).LogonTime);

                logon_session.logon_server = pwstr_to_string((*_session_data).LogonServer.Buffer,
                                                             (*_session_data).LogonServer.Length).unwrap_or("".to_string());

                logon_session.dns_domain_name = pwstr_to_string((*_session_data).DnsDomainName.Buffer,
                                                                (*_session_data).DnsDomainName.Length).unwrap_or("".to_string());

                logon_session.upn = pwstr_to_string((*_session_data).Upn.Buffer,
                                                    (*_session_data).Upn.Length).unwrap_or("".to_string());

                logon_session.logon_script = pwstr_to_string((*_session_data).LogonScript.Buffer,
                                                             (*_session_data).LogonScript.Length).unwrap_or("".to_string());

                logon_session.profile_path = pwstr_to_string((*_session_data).ProfilePath.Buffer,
                                                             (*_session_data).ProfilePath.Length).unwrap_or("".to_string());

                logon_session.home_directory = pwstr_to_string((*_session_data).HomeDirectory.Buffer,
                                                               (*_session_data).HomeDirectory.Length).unwrap_or("".to_string());

                logon_session.home_directory_drive = pwstr_to_string((*_session_data).HomeDirectoryDrive.Buffer,
                                                                     (*_session_data).HomeDirectoryDrive.Length).unwrap_or("".to_string());

                logon_sessions.push(logon_session);

                LsaFreeReturnBuffer(&mut _session_data as *mut _ as *mut c_void);
            }
        }

        LsaFreeReturnBuffer(&mut _sessions as *mut _ as *mut c_void);
    }

    logon_sessions
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
        let mut ull: LARGE_INTEGER = mem::zeroed();
        let mut adjust: LARGE_INTEGER = mem::zeroed();

        ull.u_mut().LowPart = ft.u_mut().LowPart;
        ull.u_mut().HighPart = ft.u_mut().HighPart;

        *adjust.QuadPart_mut() = 11644473600000 * 10000;
        *ull.QuadPart_mut() -= *adjust.QuadPart_mut();

        return *ull.QuadPart_mut() / 10000000;
    }
}