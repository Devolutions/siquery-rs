#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use tables::LoggedInUsers;
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::LPSTR;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::LARGE_INTEGER;
use winapi::um::winnt::CHAR;
use winapi::shared::minwindef::UCHAR;
use winapi::ctypes::*;
use std::{ptr, mem};
use libc;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::ntdef::LPWSTR;
use std::collections::HashMap;

const WINSTATIONNAME_LENGTH: usize = 32;
const DOMAIN_LENGTH: usize = 17;
const NASIUSERNAME_LENGTH: usize = 47;
const USERNAME_LENGTH: usize = 21;

#[repr(C)]
#[allow(dead_code)]
pub enum _WTS_CONNECTSTATE_CLASS {
    WTSActive = 0,
    WTSConnected = 1,
    WTSConnectQuery = 2,
    WTSShadow = 3,
    WTSDisconnected = 4,
    WTSIdle = 5,
    WTSListen = 6,
    WTSReset = 7,
    WTSDown = 8,
    WTSInit = 9
} pub type WTS_CONNECTSTATE_CLASS = _WTS_CONNECTSTATE_CLASS;


#[link(name = "Wtsapi32")]
pub enum _WTS_INFO_CLASS {
    WTSInitialProgram = 0,
    WTSApplicationName = 1,
    WTSWorkingDirectory = 2,
    WTSOEMId = 3,
    WTSSessionId = 4,
    WTSUserName = 5,
    WTSWinStationName = 6,
    WTSDomainName = 7,
    WTSConnectState = 8,
    WTSClientBuildNumber = 9,
    WTSClientName = 10,
    WTSClientDirectory = 11,
    WTSClientProductId = 12,
    WTSClientHardwareId = 13,
    WTSClientAddress = 14,
    WTSClientDisplay = 15,
    WTSClientProtocolType = 16,
    WTSIdleTime = 17,
    WTSLogonTime = 18,
    WTSIncomingBytes = 19,
    WTSOutgoingBytes = 20,
    WTSIncomingFrames = 21,
    WTSOutgoingFrames = 22,
    WTSClientInfo = 23,
    WTSSessionInfo = 24,
    WTSSessionInfoEx = 25,
    WTSConfigInfo = 26,
    WTSValidationInfo = 27,
    WTSSessionAddressV4 = 28,
    WTSIsRemoteSession = 29
} pub type WTS_INFO_CLASS = _WTS_INFO_CLASS;

#[repr(C)]
    pub struct _WTS_SESSION_INFOW  {
        SessionId: DWORD,
        pWinStationName: DWORD,
        State: WTS_CONNECTSTATE_CLASS,
}
pub type PWTS_SESSION_INFOW  = *mut _WTS_SESSION_INFOW ;

#[repr(C)]  
pub struct _WTSINFOA {
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBy: DWORD,
    WinStationName:[CHAR; WINSTATIONNAME_LENGTH],
    Domain:[CHAR; DOMAIN_LENGTH],
    UserName:[CHAR; USERNAME_LENGTH],
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
}
pub type PWTSINFOA  = *mut _WTSINFOA ;

#[link(name = "Wtsapi32")]
extern "system" {
    fn WTSEnumerateSessionsW(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOW,
        pCount: *mut DWORD) -> bool;

    fn WTSQuerySessionInformationW(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: WTS_INFO_CLASS,
        ppBuffer: *mut *mut _WTSINFOA,
        pBytesReturned: *mut DWORD,
    ) -> bool;

    fn WTSFreeMemory(pMemory: *mut c_void);
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
        let mut WTS_CURRENT_SERVER_HANDLE: *mut c_void = ptr::null_mut();

        //let mut pSessionInfo: *mut PWTS_SESSION_INFOW;
        //pSessionInfo = Vec::with_capacity((mem::size_of::<_WTS_SESSION_INFOW>()) as usize).as_mut_ptr() as *mut PWTS_SESSION_INFOW;

        let mut count_int = 0u32;
        let count: *mut c_ulong = &mut count_int as *mut c_ulong;

        let mut reserved = 0u32;
        let mut version = 1u32;
        let mut pSessionInfo: *mut PWTS_SESSION_INFOW;
        pSessionInfo = Vec::with_capacity((mem::size_of::<WTS_INFO_CLASS>()) as usize).as_mut_ptr() as *mut PWTS_SESSION_INFOW;

        let mut res =
        WTSEnumerateSessionsW(WTS_CURRENT_SERVER_HANDLE,
                              reserved,
                              version,
                              pSessionInfo,
                              count);
        //println! ("step 1");
        if GetLastError() != 0 {
            return
        }

        // get pSessionInfo size
        /*let mut pSessionInfo_sized: *mut PWTS_SESSION_INFOW;
        pSessionInfo_sized = Vec::with_capacity(((mem::size_of::<WTS_INFO_CLASS>()) * count_int as usize) as usize).as_mut_ptr() as *mut PWTS_SESSION_INFOW;

        let mut WTS_CURRENT_SERVER_HANDLE_sized: *mut c_void = ptr::null_mut();
        let mut count_int_sized = 0u32;
        let count_sized: *mut c_ulong = &mut count_int_sized as *mut c_ulong;

        WTSEnumerateSessionsW(WTS_CURRENT_SERVER_HANDLE_sized,
                              reserved,
                              version,
                              pSessionInfo_sized,
                              count_sized);
        println! ("step 2");
        if GetLastError() != 0 {
            return
        }*/

        println! ("number of users {:?}", count_int_sized);

        /*if sessionInfo != ptr::null_mut() {
            WTSFreeMemory(pSessionInfo);
            pSessionInfo = ptr::null_mut();
            WTSFreeMemory(pSessionInfo_sized);
            pSessionInfo_sized = ptr::null_mut();
        }*/
        /*for i in 0..count_int {
            let mut sessionInfo: *mut PWTSINFOA = ptr::null_mut();
            sessionInfo = Vec::with_capacity((mem::size_of::<_WTSINFOA>()) as usize).as_mut_ptr() as *mut PWTSINFOA;

            let mut bytesRet_int = 0u32;
            let bytesRet: *mut c_ulong = &mut bytesRet_int as *mut c_ulong;

            println!("pSessionInfo {:?} ", (**pSessionInfo_sized).SessionId);
            let res = WTSQuerySessionInformationW(WTS_CURRENT_SERVER_HANDLE,
                                                  (**pSessionInfo_sized).SessionId,
                                                  _WTS_INFO_CLASS::WTSSessionInfo,
                                                  sessionInfo,
                                                  bytesRet);

            if !res || sessionInfo == ptr::null_mut() {
                println!("Error querying WTS session information  : {:?}", GetLastError());
                continue;
            }

            println!("user {:?}", (**sessionInfo).UserName);
            //println!("State {:?}", (**sessionInfo).State);

            if sessionInfo != ptr::null_mut() {
                WTSFreeMemory(sessionInfo);
                sessionInfo = ptr::null_mut();
            }

            pSessionInfo_sized =  (pSessionInfo_sized).add(1);
        }*/
    }
}