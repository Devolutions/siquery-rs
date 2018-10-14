use tables::Users;
use winreg::RegKey;
use winreg::enums::*;
use winapi::um::winnt::LPSTR;
use winapi::ctypes::*;
use std::ffi::CStr;
use std::ptr;
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
use std::i64::MAX;

use winapi::{
    um::{
        winnt::{PSID,PSID_NAME_USE},
        errhandlingapi::GetLastError,
        winbase::LocalFree,
        winbase::LocalReAlloc
    },
    shared::{
        minwindef::{
            DWORD,
            HLOCAL
        },
        ntdef::{
            LPWSTR,
            LPCSTR,
            NULL
        },
        sddl::ConvertSidToStringSidW
    }
};
use widestring::WideString;
use libc;

extern "C" {
    pub fn LookupAccountSidW (
        lpSystemName: LPCSTR,
        Sid: PSID,
        Name: LPWSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE
    ) -> BOOL;
}

const NERR_Success: u32 = 0;
const kWellKnownSids: [&'static str; 17 ] = [
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
        let mut processed_sid: Vec<String> = Vec::new();
        process_local_acounts(&mut users, &mut processed_sid);
        process_roaming_profiles(&mut users, &mut processed_sid);
        users
    }
}

fn process_local_acounts(users: &mut Vec<Users>, processed_sid: &mut Vec<String>) {
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

            for i in 0..unsafe { *dw_num_users_read }  {
                let mut user = Users::new();
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
                        unsafe { NetApiBufferFree(*user_lvl_4buff.as_mut_ptr() as *mut c_void) };
                    }
                    println!("with error code {:?}", ret);

                    unsafe { iter_buff = iter_buff.add(1) as *mut _; }
                    continue;
                }

                // Will return empty string on fail
                let mut lp_user_info_4: LPUSER_INFO_4 = unsafe { ptr::read(user_lvl_4buff.as_mut_ptr() as _) };
                let mut sid: *mut c_void = unsafe { (*lp_user_info_4).usri4_user_sid };

                unsafe {
                    if let Ok(username) = lpwstr_to_string((*iter_buff).usri3_name) {
                        user.username = username;
                    }
                    if let Ok(description) = get_user_description((*lp_user_info_4).usri4_comment) {
                        user.description = description;
                    }
                }

                user.shell = "C:\\Windows\\System32\\cmd.exe".to_string();
                user.type_ = "local".to_string();

                if let Ok(sid_string) = sid_to_string(sid) {
                    user.uuid = sid_string.clone();
                    processed_sid.push(sid_string.clone());
                    user.directory = get_user_home_dir(sid_string);
                } unsafe {
                    user.uid = (*iter_buff).usri3_user_id as i64;
                    user.gid = (*iter_buff).usri3_primary_group_id as i64;
                    user.uid_signed = user.uid;
                    user.gid_signed = user.gid;
                }
                if user_lvl_4buff.as_mut_ptr() != ptr::null_mut() {
                    unsafe { NetApiBufferFree(*user_lvl_4buff.as_mut_ptr() as *mut c_void)};
                }
                users.push(user);
                unsafe { iter_buff = iter_buff.add(1) as LPUSER_INFO_3; };
            }
        // if there are no local users
        } else {
            println!("NetUserEnum failed with {:?}", ret);
        }

        if user_buffer.as_mut_ptr() != ptr::null_mut() {
            unsafe { NetApiBufferFree(*user_buffer.as_mut_ptr() as *mut c_void) };
        }

        if ret != ERROR_MORE_DATA {
            break;
        }
    }
}

//todo
fn process_roaming_profiles(users: &mut Vec<Users>, processed_sid: &mut Vec<String>){
    let key = r#"Software\Microsoft\Windows NT\CurrentVersion\ProfileList"#;
    let hklm = &RegKey::predef(HKEY_LOCAL_MACHINE);
    let processed_sid_iter = processed_sid.iter();

    if let Ok(profile) = hklm.open_subkey_with_flags(key, KEY_READ) {
        let mut type_ : String = "".to_string();

        for _x in 0..profile.enum_keys().count() {
            let mut user = Users::new();
            let type_key = profile.enum_keys().nth(_x).unwrap();

            if type_ == "subkey".to_string() {
                continue;
            }

            let mut processed = false;
            if let Ok(sid_string) = type_key {
                for sid in processed_sid.iter() {
                    if *sid == sid_string {
                        processed = true;
                        continue;
                    }
                }
                if !processed {
                    user.uuid = sid_string.clone();
                    user.directory = get_user_home_dir(sid_string.clone());
                    user.uid = get_uid_from_sid(sid_string.clone());
                    user.gid = user.uid;
                    user.uid_signed = user.uid;
                    user.gid_signed = user.gid;
                    let mut known_sid = false;
                    for sid in kWellKnownSids.iter(){
                        if *sid == sid_string.as_str() {
                            user.type_ = "special".to_string();
                            known_sid = true;
                        }
                    }
                    if !known_sid {
                        user.type_ = "roaming".to_string();
                    }
                    user.shell = "C:\\Windows\\system32\\cmd.exe".to_string();
                    user.description = "".to_string();
                    users.push(user);
                }
            }
        }
    }
}

fn get_user_home_dir(sid_string: String)->String {
    let key = format!(r#"Software\Microsoft\Windows NT\CurrentVersion\ProfileList\{}"#, sid_string);
    let hklm = &RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut home_dir: String = "".to_string();

    if let Ok(subkey) = hklm.open_subkey_with_flags(key, KEY_READ) {
        home_dir = subkey.get_value("ProfileImagePath").unwrap_or("".to_string());
    }

    home_dir
}

fn from_wide_string(s: &[u16]) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    let slice = s.split(|&v| v == 0).next().unwrap();
    OsString::from_wide(slice).to_string_lossy().into()
}

fn get_user_description (lpwstr: LPWSTR) -> Result<String, DWORD> {
    unsafe {
        let buf_size = libc::wcslen(lpwstr);
        let string = WideString::from_ptr(lpwstr, buf_size);
        Ok(string.to_string_lossy())
    }
}

pub fn sid_to_string(sid: PSID) -> Result<String, DWORD> {
    let mut buf: LPWSTR = NULL as LPWSTR;
    if unsafe { ConvertSidToStringSidW(sid, &mut buf) } == 0 ||
        buf == (NULL as LPWSTR) {
        return Err(unsafe { GetLastError() });
    }
    lpwstr_to_string(buf)
}

pub fn lpwstr_to_string(lpwstr: LPWSTR) -> Result<String, DWORD> {
    let buf_size = unsafe { libc::wcslen(lpwstr) };
    let string = unsafe { WideString::from_ptr(lpwstr, buf_size) };
    Ok(string.to_string_lossy())
}

pub fn get_uid_from_sid (sid_string: String) -> i64 {
    let components: Vec<_> = sid_string.as_str().split('-').collect();
    if components.len() < 1 {
        return MAX
    }
    let uid = components[components.len() - 1].parse::<i64>().unwrap_or(MAX);
    return uid
}

pub fn lookup_account_sid_internal (
    lp_system_name: LPCSTR,
    sid: PSID,
    name: LPWSTR,
    cch_name: LPDWORD,
    referenced_domain_name: LPWSTR,
    cch_referenced_domain_name: LPDWORD,
    pe_use: PSID_NAME_USE
) -> BOOL {
    unsafe {
        LookupAccountSidW (
            lp_system_name,
            sid,
            name,
            cch_name,
            referenced_domain_name,
            cch_referenced_domain_name,
            pe_use
        )
    }
}

//todo
pub fn get_roaming_acount_user_name() {}