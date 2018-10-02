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
use winapi::um::lmaccess::LPUSER_INFO_4;
use winapi::um::lmaccess::NetUserGetInfo;
use winapi::um::lmaccess::USER_INFO_4;
use winapi::um::lmaccess::USER_INFO_3;
use winapi::um::lmaccess::PUSER_INFO_4;
use winapi::um::lmaccess::PUSER_INFO_3;
use winapi::um::lmapibuf::NetApiBufferFree;
use winapi::shared::winerror::*;
use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::BOOL;
use utils::sid_to_string;


const NERR_Success: u32 = 0;

const COLUMN_NAMES: [&'static str; 17 ] = [
    "S-1-5-1",
    "S-1-5-2",
    "S-1-5-3",
    "S-1-5-4",
    "S-1-5-6",
    "S-1-5-7",
    "S-1-5-8",
    "S-1-5-9",
    "S-1-5-10",
    "S-1-5-11",
    "S-1-5-12",
    "S-1-5-13",
    "S-1-5-18",
    "S-1-5-19",
    "S-1-5-20",
    "S-1-5-21",
    "S-1-5-32",];

/*pub fn get_uid_from_sid(sid: PSID) -> u64 {

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
}*/

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
        process_local_acounts(&mut users);

        let mut user = Users::new();

        users
    }
}

fn process_local_acounts(users: &mut Vec<Users>) {
    let mut dw_user_info_level: c_ulong = 3;

    let mut dw_num_users_read_int = 0u32;
    let mut dw_num_users_read: *mut c_ulong = &mut dw_num_users_read_int as *mut c_ulong;

    let mut dw_total_users_int = 0u32;
    let mut dw_total_users: *mut c_ulong = &mut dw_total_users_int as *mut c_ulong;

    let mut resume_handle_int = 0u32;
    let mut resume_handle: *mut c_ulong = &mut resume_handle_int as *mut c_ulong;

    let mut ret: u32 = 0;

    let mut user_buffer: Vec<*mut u8> = Vec::with_capacity((MAX_PREFERRED_LENGTH) as usize);
    loop {
        ret = unsafe {
            NetUserEnum(ptr::null(),
                        dw_user_info_level,
                        0 as DWORD,
                        user_buffer.as_mut_ptr(),
                        MAX_PREFERRED_LENGTH,
                        dw_num_users_read,
                        dw_total_users,
                        resume_handle)
        };

        if (ret == NERR_Success || ret == ERROR_MORE_DATA) &&
            user_buffer.as_mut_ptr() != ptr::null_mut() {

            let mut iter_buff: LPUSER_INFO_3 = unsafe { ptr::read(user_buffer.as_mut_ptr() as *mut _) };

            let mut user = Users::new();
            for i in 0..unsafe { *dw_num_users_read } {
                let mut dw_detailed_user_info_level: c_ulong = 4;
                let mut user_lvl_4buff: Vec<*mut u8> = Vec::with_capacity((mem::size_of::<USER_INFO_4>()) as usize);

                ret = unsafe {
                    NetUserGetInfo(ptr::null(),
                                   (*iter_buff).usri3_name,
                                   dw_detailed_user_info_level,
                                   user_lvl_4buff.as_mut_ptr())
                };

                if ret != NERR_Success || user_lvl_4buff.as_mut_ptr() == ptr::null_mut() {
                    if user_lvl_4buff.as_mut_ptr() != ptr::null_mut() {
                        unsafe{NetApiBufferFree(*user_lvl_4buff.as_mut_ptr() as *mut c_void )};
                    }
                    println!("with error code {:?}", ret);

                    //todo incr iter_buff
                    //iter_buff +=1;
                    continue;
                }

                // Will return empty string on fail
                let mut lp_user_info_4: LPUSER_INFO_4 = unsafe { ptr::read(user_lvl_4buff.as_mut_ptr() as _) };
                let mut sid: *mut c_void = unsafe {(*lp_user_info_4).usri4_user_sid};

                // todo prossecedSids
                let sid_string = sid_to_string(sid);

                // todo fill user row
                /*
                username: String::new(),
                description: String::new(),
                directory: String::new(),*/

                user.shell = "C:\\Windows\\System32\\cmd.exe".to_string();
                user.type_ = "local".to_string();
                if let Ok(sid_string) = sid_to_string(sid) {
                    user.uuid = sid_string;
                }

                unsafe {
                    user.uid = (*iter_buff).usri3_user_id as i64;
                    user.gid = (*iter_buff).usri3_primary_group_id as i64;
                    user.uid_signed = user.uid;
                    user.gid_signed = user.gid;
                }
            }
            users.push(user);
        } else {
            println!("NetUserEnum failed with {:?}", ret);
        }

        if user_buffer.as_mut_ptr() != ptr::null_mut() {
            unsafe{NetApiBufferFree(*user_buffer.as_mut_ptr() as *mut c_void )};
        }

        if ret != ERROR_MORE_DATA {
            break;
        }
    }
}

fn process_roaming_profiles(){

}
// todo get home dir using RegKey
fn get_user_home_dir()->String {
    "".to_string()
}

fn from_wide_string(s: &[u16]) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    let slice = s.split(|&v| v == 0).next().unwrap();
    OsString::from_wide(slice).to_string_lossy().into()
}