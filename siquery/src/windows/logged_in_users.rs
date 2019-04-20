#![allow(warnings)]
use crate::tables::LoggedInUsers;
use winapi::{
    shared::
    {
        minwindef::DWORD,
        ws2def::{AF_INET, AF_INET6},
        minwindef::FILETIME,
        ntdef::{LPWSTR, ULONG},
    },
    um::{
        winnt::{HANDLE, LARGE_INTEGER, CHAR, PVOID, LPSTR },
        errhandlingapi::GetLastError,
    },
    ctypes::*,
};
use std::{ptr, mem, str, ffi::CStr};
use widestring::WideString;

const WINSTATIONNAME_LENGTH: usize = 32;
const DOMAIN_LENGTH: usize = 17;
const USERNAME_LENGTH: usize = 21;
const CLIENTNAME_LENGTH: usize = 20;
const CLIENTADDRESS_LENGTH: usize = 30;
const MAX_PATH : usize = 260;

#[repr(C)]
macro_rules! enum_str {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        pub enum $name {
            $($variant),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

#[repr(C)]
enum_str! {
pub enum WTS_CONNECTSTATE_CLASS {
    Active,
    Connected,
    ConnectQuery,
    Shadow,
    Disconnected,
    Idle,
    Listen,
    Reset,
    Down,
    Init,
}
}

#[link(name = "Wtsapi32")]
#[repr(C)]
pub enum WTS_INFO_CLASS {
    WTSInitialProgram,
    WTSApplicationName,
    WTSWorkingDirectory,
    WTSOEMId,
    WTSSessionId,
    WTSUserName,
    WTSWinStationName,
    WTSDomainName,
    WTSConnectState,
    WTSClientBuildNumber,
    WTSClientName,
    WTSClientDirectory,
    WTSClientProductId,
    WTSClientHardwareId,
    WTSClientAddress,
    WTSClientDisplay,
    WTSClientProtocolType,
    WTSIdleTime,
    WTSLogonTime,
    WTSIncomingBytes,
    WTSOutgoingBytes,
    WTSIncomingFrames,
    WTSOutgoingFrames,
    WTSClientInfo,
    WTSSessionInfo,
    WTSSessionInfoEx,
    WTSConfigInfo,
    WTSValidationInfo, // Info Class value used to fetch Validation Information through the WTSQuerySessionInformation
    WTSSessionAddressV4,
    WTSIsRemoteSession,
}

#[repr(C)]
pub struct WTS_SESSION_INFO_1W {
    ExecEnvId: DWORD,
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    pSessionName: LPWSTR,
    pHostName: LPWSTR,
    pUserName: LPWSTR,
    pDomainName: LPWSTR,
    pFarmName: LPWSTR,
}
pub type PWTS_SESSION_INFOW  = *mut WTS_SESSION_INFO_1W ;

#[repr(C)]
pub struct WTSINFOA {
    State: isize,
    SessionId: DWORD,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBy: DWORD,
    WinStationName:[CHAR; WINSTATIONNAME_LENGTH],
    Domain:[i8; DOMAIN_LENGTH],
    UserName:[i8; USERNAME_LENGTH],
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
}
pub type PWTSINFOA  = *mut WTSINFOA ;

#[repr(C)]
pub struct WTSCLIENTA {
    ClientName: [CHAR; CLIENTNAME_LENGTH + 1],
    Domain: [CHAR; DOMAIN_LENGTH + 1],
    UserName: [CHAR; USERNAME_LENGTH + 1],
    WorkDirectory: [i8; MAX_PATH + 1],
    InitialProgram: [i8; MAX_PATH + 1],
    EncryptionLevel: u8,
    ClientAddressFamily: u64,
    ClientAddress: [u16; CLIENTADDRESS_LENGTH + 1],
    HRes: u16,
    VRes: u16,
    ColorDepth: u16,
    ClientDirectory: [CHAR; MAX_PATH + 1],
    ClientBuildNumber: u64,
    ClientHardwareId: u64,
    ClientProductId: u16,
    OutBufCountHost: u16,
    OutBufCountClient: u16,
    OutBufLength: u16,
    DeviceId: [CHAR; MAX_PATH + 1],
} pub type PWTSCLIENTA = *mut WTSCLIENTA;

#[link(name = "Wtsapi32")]
extern "system" {
    pub fn WTSEnumerateSessionsExW(
        hServer: HANDLE,
        pLevel: *mut DWORD,
        Filter: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOW,
        pCount: *mut DWORD
    ) -> bool;

    pub fn WTSQuerySessionInformationA(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: usize,
        ppBuffer: *mut LPSTR,
        pBytesReturned: *mut DWORD,
    ) -> bool;

    pub fn WTSQuerySessionInformationW(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: usize,
        ppBuffer: *mut LPWSTR,
        pBytesReturned: *mut DWORD,
    ) -> bool;

    pub fn WTSFreeMemoryExW(WTSTypeClass: usize, pMemory: PVOID, NumberOfEntries: ULONG);
}

impl LoggedInUsers {
    pub(crate) fn new() -> LoggedInUsers {
        LoggedInUsers {
            type_: String::new(),
            user: String::new(),
            tty: String::new(),
            host: String::new(),
            time: 0,
            pid: 0,
        }
    }

    pub fn get_specific() -> Vec<LoggedInUsers> {
        let mut logged_in_users: Vec<LoggedInUsers> = Vec::new();
        get_logged_in_users(&mut logged_in_users);
        logged_in_users
    }
}

fn get_logged_in_users(logged_in_users: &mut Vec<LoggedInUsers>) {
    unsafe {
        let mut logged_in_user = LoggedInUsers::new();
        let WTS_CURRENT_SERVER_HANDLE: *mut c_void = ptr::null_mut();

        let mut count_int = 0u32;
        let count: *mut c_ulong = &mut count_int as *mut c_ulong;

        let mut level_int = 1u32;
        let level: *mut c_ulong = &mut level_int as *mut c_ulong;

        let mut pSessionInfo: *mut PWTS_SESSION_INFOW = ptr::null_mut();
        let mut sessionInfo_array: Vec<u16> = Vec::with_capacity((mem::size_of::<WTS_SESSION_INFO_1W>()) as usize);
        let pSessionInfo_array = sessionInfo_array.as_mut_ptr();
        pSessionInfo = pSessionInfo_array as *mut _;

        let mut res = WTSEnumerateSessionsExW(WTS_CURRENT_SERVER_HANDLE,
                                              level,
                                              0,
                                              pSessionInfo,
                                              count
        );

        if !res {
            println!("error");
            return
        }

        WTSFreeMemoryExW(2,
                         *pSessionInfo as *mut c_void,
                         count_int);

        pSessionInfo = ptr::null_mut();
        let sessionInfo_array_sized: Vec<WTS_SESSION_INFO_1W> = Vec::with_capacity(count_int as usize);
        let pSessionInfo_array_sized = sessionInfo_array.as_mut_ptr();
        pSessionInfo = pSessionInfo_array_sized as *mut _;

        res = WTSEnumerateSessionsExW(WTS_CURRENT_SERVER_HANDLE,
                                      level,
                                      0,
                                      pSessionInfo,
                                      count
        );

        if !res {
            println!("error");
            return
        }

        for i in 0..count_int {
            let mut sessionInfo: *mut PWTSINFOA = ptr::null_mut();
            let mut sessionInfo_data: Vec<u16> = Vec::with_capacity((mem::size_of::<WTSINFOA>()) as usize);
            sessionInfo = sessionInfo_data.as_mut_ptr() as *mut PWTSINFOA;

            let mut bytesRet_int_ = 0u32;
            let mut bytesRet: *mut c_ulong = &mut bytesRet_int_ as *mut c_ulong;

            // get username
            res = WTSQuerySessionInformationW(WTS_CURRENT_SERVER_HANDLE,
                                              (**pSessionInfo).SessionId,
                                              25,
                                              sessionInfo as *mut *mut u16,
                                              bytesRet);

            let username_vec = ((**sessionInfo).UserName).to_vec();
            logged_in_user.user =  i8_to_string(username_vec);

            res = WTSQuerySessionInformationA(WTS_CURRENT_SERVER_HANDLE,
                                              (**pSessionInfo).SessionId,
                                              25,
                                              sessionInfo as *mut *mut i8,
                                              bytesRet);

            if !res || sessionInfo == ptr::null_mut() {
                println!("Error querying WTS session information  : {:?}", GetLastError());
                continue;
            }

            logged_in_user.type_ = (**pSessionInfo).State.name().to_string();
            logged_in_user.pid = -1;
            logged_in_user.tty = get_user_tty((**pSessionInfo).pSessionName);
            logged_in_user.time = get_unixtime((**sessionInfo).ConnectTime);

            let mut clientInfo: *mut PWTSCLIENTA = ptr::null_mut();
            let mut clientInfo_data: Vec<u16> = Vec::with_capacity((mem::size_of::<WTSCLIENTA>()) as usize);
            clientInfo = clientInfo_data.as_mut_ptr() as *mut PWTSCLIENTA;
            bytesRet_int_ = 0;

            res = WTSQuerySessionInformationA(WTS_CURRENT_SERVER_HANDLE,
                                              (**pSessionInfo).SessionId,
                                              24, // clientInfo
                                              clientInfo as *mut *mut i8,
                                              bytesRet);

            if !res || clientInfo == ptr::null_mut() {
                println!("Error querying WTS session information  : {:?}", GetLastError());
                logged_in_users.push(logged_in_user);
                logged_in_user = LoggedInUsers::new();
                *pSessionInfo = (*pSessionInfo).add(1);
                continue;
            }

            // to test
            if (**clientInfo).ClientAddressFamily == AF_INET as u64 {
                let mut host_u16 = String::from_utf16(&(**clientInfo).ClientAddress).unwrap();
                logged_in_user.host = host_u16;

            } else if (**clientInfo).ClientAddressFamily == AF_INET6 as u64 {
                let mut host = String::from_utf16(&(**clientInfo).ClientAddress).unwrap();
                logged_in_user.host = host;
            }

            *pSessionInfo = (*pSessionInfo).add(1);
            logged_in_users.push(logged_in_user);
            logged_in_user = LoggedInUsers::new();
        }
    }
}

pub fn i8_to_string(vec: Vec<i8>) -> String {
    let mut vec_c: Vec<_> = Vec::new();
    for i in vec.iter() {
        if *i != 0 {
            vec_c.push(*i);
        }
    }
    let s: String = String::from_utf8(vec_c.iter().map(|&c| c as u8).collect()).unwrap();
    s
}

pub fn filetime_to_unixtime(ft : &mut FILETIME) -> i64 {
    unsafe {
        let mut date: LARGE_INTEGER = mem::uninitialized();
        let mut adjust: LARGE_INTEGER = mem::uninitialized();
        date.u_mut().HighPart = ft.dwHighDateTime as i32;
        date.u_mut().LowPart = ft.dwLowDateTime;
        *adjust.QuadPart_mut() = 11644473600000 * 10000;
        *date.QuadPart_mut() -= *adjust.QuadPart_mut();
        return *date.QuadPart_mut() / 10000000;
    }
}

pub fn get_user_tty(session_name: LPWSTR) -> String {
    let mut buf_session_name = session_name;
    if buf_session_name != ptr::null_mut() {
        let mut v: [u16; 25] = unsafe { mem::uninitialized() };
        let buf_size = 25;
        let session_name = unsafe { WideString::from_ptr(buf_session_name, buf_size) };
        let mut st = session_name.to_string().unwrap_or("".to_owned());
        let v: Vec<_> = st.split("\u{0}").collect();
        v[0].to_string()
    } else {
        "".to_string()
    }
}

pub fn get_unixtime(connectTime: LARGE_INTEGER) -> i64 {
    unsafe {
        let mut utcTime: FILETIME = mem::uninitialized();
        utcTime.dwLowDateTime = connectTime.u().LowPart as u32;
        utcTime.dwHighDateTime = connectTime.u().HighPart as u32;
        let mut unixTime: i64 = 0;
        if utcTime.dwLowDateTime != 0 || utcTime.dwHighDateTime != 0 {
            unixTime = filetime_to_unixtime(&mut utcTime);
        }
        unixTime
    }
}