use tables::Users;
use winreg::RegKey;
use winreg::enums::*;
use winapi::um::winnt::PSID;
use winapi::um::winnt::LPSTR;
use winapi::um::winbase::LocalFree;
use winapi::ctypes::*;
use std::ffi::CStr;
use std::ptr;


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
        let mut users: Vec<Users> = Vec::new();
        let mut user = Users::new();

        users
    }


}

fn process_local_acounts(){

}

fn process_roaming_profiles(){

}

fn get_user_home_dir()->String {
    "".to_string()
}