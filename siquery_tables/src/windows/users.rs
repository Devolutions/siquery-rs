use tables::Users;
use winreg::RegKey;
use winreg::enums::*;
use winapi::um::winnt::PSID;
use winapi::um::winnt::LPSTR;
use winapi::um::winbase::LocalFree;
use winapi::ctypes::*;
use std::ffi::CStr;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::shared::lmcons::MAX_PREFERRED_LENGTH;
use winapi::shared::minwindef::LPBYTE;
use winapi::shared::minwindef::LPDWORD;
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::winerror::*;
use std::mem;
use winapi::um::lmaccess::NetUserEnum;
use winapi::um::lmaccess::LPUSER_INFO_3;
use winapi::um::lmaccess::NetUserGetInfo;

const NERR_Success: u32 = 0;

extern "C" {
    pub fn ConvertSidToStringSidA (sid : PSID, sid_out: *mut c_void) -> bool;
}

pub fn get_uid_from_sid(sid: PSID) -> u64 {

    const UID_DEFAULT : u64 = 1;
    let sid_string : *mut c_void = ptr::null_mut();

    if unsafe {ConvertSidToStringSidA(sid, sid_string)} == false {
        println!("get_uid_from_sid failed ConvertSidToStringSidA");
        return UID_DEFAULT;
    }

    let toks = unsafe {CStr::from_ptr(sid_string as *const _)}.to_string_lossy();
        //let toks = sid_string.to_string().split("-");

        if toks.len() < 1 {
            unsafe {LocalFree(sid_string)};
            return UID_DEFAULT;
        }

    let uid_exp = toks.to_string().parse::<u64>().unwrap_or(0);

    // todo handle errors
    /*if uid_exp.isError() {
        LocalFree(sid_string);
        println!( "failed to parse PSID ");
        return UID_DEFAULT;
    }*/

    unsafe {LocalFree(sid_string)};
    return uid_exp;
}



impl Users {
    pub(crate) fn new() -> Users {
        Users {
            uid: 0,
            gid: 0,
            uid_signed: 0,
            gid_signed: 0,
            username: String::new(),
            description: String::new(),
            directory: String::new(),
            shell: String::new(),
            uuid: String::new(),
            type_: String::new(),
        }
    }

    pub fn get_specific() -> Vec<Users> {
        process_local_acounts();
        let mut users: Vec<Users> = Vec::new();
        let mut user = Users::new();

        users
    }
}

fn process_local_acounts(){
    let mut dw_user_info_level : c_ulong  = 3;

    let mut dw_num_users_read_int = 0u32;
    let mut dw_num_users_read: *mut c_ulong  = &mut dw_num_users_read_int as *mut c_ulong;

    let mut dw_total_users_int = 0u32;
    let mut dw_total_users: *mut c_ulong  =  &mut dw_total_users_int as *mut c_ulong;

    let mut resume_handle_int = 0u32;
    let mut resume_handle: *mut c_ulong  = &mut resume_handle_int as *mut c_ulong;

    let mut ret: u32 = 0;

    let mut user_buffer: Vec<*mut u8> = Vec::with_capacity((MAX_PREFERRED_LENGTH) as usize);

    ret = unsafe { NetUserEnum(ptr::null(),
                      dw_user_info_level,
                      0 as DWORD,
                      user_buffer.as_mut_ptr(),
                      MAX_PREFERRED_LENGTH,
                      dw_num_users_read,
                      dw_total_users,
                      resume_handle) };

    if (ret == NERR_Success || ret == ERROR_MORE_DATA) &&
        user_buffer.as_mut_ptr() != ptr::null_mut() {

        let mut iter_buff: LPUSER_INFO_3 = user_buffer.as_ptr() as LPUSER_INFO_3;

        for i in 0..unsafe{*dw_num_users_read} {
            let mut dw_detailed_user_info_level: c_ulong  = 4;

            // todo : get the right size of the buffer
            // see : https://docs.microsoft.com/en-us/windows/desktop/NetMgmt/network-management-function-buffer-lengths
            let mut user_lvl_4buff: Vec<*mut u8> = Vec::new();

            println!("LPUSER_INFO_3 value {:?}", unsafe{(*iter_buff).usri3_name});

            ret = unsafe { NetUserGetInfo(ptr::null(),
                            (*iter_buff).usri3_name,
                            dw_detailed_user_info_level,
                            user_lvl_4buff.as_mut_ptr())};
        }
    }
}

fn process_roaming_profiles(){

}

fn get_user_home_dir()->String {
    "".to_string()
}