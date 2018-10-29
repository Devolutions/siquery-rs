#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use tables::LoggedInUsers;
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::LPSTR;
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::UCHAR;
use winapi::ctypes::*;
use std::{ptr, mem};
use libc;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::CHAR;
use winapi::shared::ntdef::LPWSTR;

#[repr(C)]
#[allow(dead_code)]
pub enum _WTS_CONNECTSTATE_CLASS {
    WTSActive,
    WTSConnected,
    WTSConnectQuery,
    WTSShadow,
    WTSDisconnected,
    WTSIdle,
    WTSListen,
    WTSReset,
    WTSDown,
    WTSInit
} pub type WTS_CONNECTSTATE_CLASS = _WTS_CONNECTSTATE_CLASS;

pub enum _WTS_INFO_CLASS {
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
    WTSValidationInfo,
    WTSSessionAddressV4,
    WTSIsRemoteSession
} pub type WTS_INFO_CLASS = _WTS_INFO_CLASS;

#[repr(C)]
    pub struct _WTS_SESSION_INFOW  {
        SessionId: DWORD,
        pWinStationName: DWORD,
        State: WTS_CONNECTSTATE_CLASS,
}
pub type PWTS_SESSION_INFOW  = *mut _WTS_SESSION_INFOW ;

#[link(name = "Wtsapi32")]
extern "system" {
    fn WTSEnumerateSessionsW(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOW,
        pCount: *mut DWORD) -> UCHAR;

    fn WTSQuerySessionInformationW(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: WTS_INFO_CLASS,
        ppBuffer: *mut LPWSTR,
        pBytesReturned: *mut DWORD,
    ) -> bool;
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
    let mut WTS_CURRENT_SERVER_HANDLE: *mut c_void = ptr::null_mut();

    let mut pSessionInfo: *mut PWTS_SESSION_INFOW;
    pSessionInfo = Vec::with_capacity((mem::size_of::<_WTS_SESSION_INFOW>()) as usize).as_mut_ptr() as *mut PWTS_SESSION_INFOW;

    let mut count_int = 0u32;
    let count: *mut c_ulong = &mut count_int as *mut c_ulong;

    let mut reserved = 0u32;
    let mut version = 1u32;

    let mut res = unsafe {
        WTSEnumerateSessionsW(WTS_CURRENT_SERVER_HANDLE,
                              reserved,
                              version,
                              pSessionInfo,
                              count);

        if GetLastError() != 0 {
            return
        }

        // get pSessionInfo size
        pSessionInfo = Vec::with_capacity(((mem::size_of::<_WTS_SESSION_INFOW>()) * count_int as usize) as usize).as_mut_ptr() as *mut PWTS_SESSION_INFOW;

        WTSEnumerateSessionsW(WTS_CURRENT_SERVER_HANDLE,
                              reserved,
                              version,
                              pSessionInfo,
                              count);

        if GetLastError() != 0 {
            return
        }
    };

    for i in 0..count_int {
        let sessionInfo: *mut CHAR = ptr::null_mut();

        let mut bytesRet_int = 0u32;
        let bytesRet: *mut c_ulong = &mut bytesRet_int as *mut c_ulong;

        unsafe {

            println! ("pSessionInfo {:?} ", (**pSessionInfo).SessionId);
            /*res = WTSQuerySessionInformationW(WTS_CURRENT_SERVER_HANDLE,


            )*/
            *pSessionInfo =  (*pSessionInfo).add(1);
        }
    }
}


