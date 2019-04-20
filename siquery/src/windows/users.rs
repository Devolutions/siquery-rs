use crate::tables::Users;
use winreg::RegKey;
use winreg::enums::*;
use std::{ptr, mem, i64::MAX};
use widestring::WideString;
use libc;
use winapi::{
    ctypes::*,
    um::{
        winnt::{PSID, PSID_NAME_USE, SID_NAME_USE, SidTypeUnknown},
        lmaccess::{NetUserEnum, NetUserGetInfo, LPUSER_INFO_3, LPUSER_INFO_4, USER_INFO_4},
        lmapibuf::NetApiBufferFree,
        errhandlingapi::GetLastError
    },
    shared::{
        minwindef::{DWORD, LPDWORD, BOOL, LPBYTE},
        ntdef::{LPWSTR, LPCWSTR, NULL},
        sddl::{ConvertSidToStringSidW, ConvertStringSidToSidW},
        lmcons::{MAX_PREFERRED_LENGTH, UNLEN, DNLEN},
        winerror::*,
    }
};

extern "system" {
    fn LookupAccountSidW(
        lpSystemName: LPCWSTR,
        Sid: PSID,
        Name: LPWSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE
    ) -> BOOL;
}

const NERR_SUCCESS: u32 = 0;
const K_WELL_KNOWN_SIDS: [&'static str; 17 ] = [
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
        process_local_accounts(&mut users, &mut processed_sid);
        process_roaming_profiles(&mut users, &mut processed_sid);
        users
    }
}

fn process_local_accounts(users: &mut Vec<Users>, processed_sid: &mut Vec<String>) {
    let dw_user_info_level: c_ulong = 3;

    let mut dw_num_users_read_int = 0u32;
    let dw_num_users_read: *mut c_ulong = &mut dw_num_users_read_int as *mut c_ulong;

    let mut dw_total_users_int = 0u32;
    let dw_total_users: *mut c_ulong = &mut dw_total_users_int as *mut c_ulong;

    let mut resume_handle_int = 0u32;
    let resume_handle: *mut c_ulong = &mut resume_handle_int as *mut c_ulong;

    let mut user_buffer = ptr::null_mut();
    let buf_ptr = &mut user_buffer as *mut LPBYTE;
    loop {
        let mut ret = unsafe {
            NetUserEnum(ptr::null(),
                        dw_user_info_level,
                        0 as DWORD,
                        buf_ptr,
                        MAX_PREFERRED_LENGTH,
                        dw_num_users_read,
                        dw_total_users,
                        resume_handle)
        };

        if (ret == NERR_SUCCESS || ret == ERROR_MORE_DATA) &&
            buf_ptr != ptr::null_mut() {

            let mut iter_buff: LPUSER_INFO_3 = unsafe { ptr::read(buf_ptr as *mut _) };

            for _i in 0..unsafe { *dw_num_users_read }  {
                let mut user = Users::new();
                let mut dw_detailed_user_info_level: c_ulong = 4;
                let mut user_lvl_4buff: Vec<*mut u8> = Vec::with_capacity((mem::size_of::<USER_INFO_4>()) as usize);

                ret = unsafe {
                    NetUserGetInfo(ptr::null(),
                                   (*iter_buff).usri3_name,
                                   dw_detailed_user_info_level,
                                   user_lvl_4buff.as_mut_ptr())
                };

                if ret != NERR_SUCCESS || user_lvl_4buff.as_mut_ptr() == ptr::null_mut() {
                    unsafe {
                        if user_lvl_4buff.as_mut_ptr() != ptr::null_mut() {
                            NetApiBufferFree(*user_lvl_4buff.as_mut_ptr() as *mut c_void);
                        }
                        println!("with error code {:?}", ret);

                        iter_buff = iter_buff.add(1) as *mut _;
                    }
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
                }
                unsafe {
                    user.uid = (*iter_buff).usri3_user_id as i64;
                    user.gid = (*iter_buff).usri3_primary_group_id as i64;
                    user.uid_signed = user.uid;
                    user.gid_signed = user.gid;
                }
                users.push(user);

                unsafe {
                    iter_buff = iter_buff.add(1) as LPUSER_INFO_3;
                    if user_lvl_4buff.as_mut_ptr() != ptr::null_mut() {
                        NetApiBufferFree(*user_lvl_4buff.as_mut_ptr() as *mut c_void);
                    }
                }
            }
        // if there are no local users
        } else {
            println!("NetUserEnum failed with {:?}", ret);
        }

        if buf_ptr != ptr::null_mut() {
            unsafe { NetApiBufferFree(*buf_ptr as *mut c_void) };
        }

        if ret != ERROR_MORE_DATA {
            break;
        }
    }
}

fn process_roaming_profiles(users: &mut Vec<Users>, processed_sid: &mut Vec<String>){
    let key = r#"Software\Microsoft\Windows NT\CurrentVersion\ProfileList"#;
    let hklm = &RegKey::predef(HKEY_LOCAL_MACHINE);

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
                    for sid in K_WELL_KNOWN_SIDS.iter(){
                        if *sid == sid_string.as_str() {
                            user.type_ = "special".to_string();
                            known_sid = true;
                        }
                    }
                    if !known_sid {
                        user.type_ = "roaming".to_string();
                    }
                    let mut sid: *mut PSID = convert_string_sid_to_sid(sid_string.clone());
                    user.username = get_roaming_profiles_username(sid);
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

fn get_user_description (lpwstr: LPWSTR) -> Result<String, DWORD> {
    unsafe {
        let buf_size = libc::wcslen(lpwstr);
        let string = WideString::from_ptr(lpwstr, buf_size);
        Ok(string.to_string_lossy())
    }
}

pub fn sid_to_string(sid: PSID) -> Result<String, DWORD> {
    let mut buf: LPWSTR = NULL as LPWSTR;
    unsafe {
        if ConvertSidToStringSidW(sid, &mut buf) == 0 ||
            buf == (NULL as LPWSTR) {
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

pub fn get_uid_from_sid (sid_string: String) -> i64 {
    let components: Vec<_> = sid_string.as_str().split('-').collect();
    if components.len() < 1 {
        return MAX
    }
    let uid = components[components.len() - 1].parse::<i64>().unwrap_or(MAX);
    return uid
}

pub fn convert_string_sid_to_sid(sid_string: String) -> *mut PSID {
    let mut sid_c_str: Vec<u16> = sid_string.clone().as_str().encode_utf16().collect();
    sid_c_str.push(0);

    let mut sid_buf = ptr::null_mut();
    let sid: *mut PSID  = &mut sid_buf as *mut PSID;
    unsafe {
        let ret = ConvertStringSidToSidW(sid_c_str.as_ptr(), sid);
        let last_error = GetLastError();
        if ret == 0 && last_error != ERROR_INSUFFICIENT_BUFFER {
            println!("failed with error {:?}", last_error);
        };
        sid
    }
}

pub fn get_roaming_profiles_username(sid: *mut PSID) -> String {
    unsafe {
        let mut dom_name_len = DNLEN;
        let dom_name_len_p: LPDWORD = &mut dom_name_len as LPDWORD;
        let mut dom_name: [wchar_t; DNLEN as usize] = [0; DNLEN as usize];

        let mut account_name_len = UNLEN;
        let account_name_len_p: LPDWORD = &mut account_name_len as LPDWORD;
        let mut account_name: [wchar_t; UNLEN as usize] = [0; UNLEN as usize];

        let mut e_sid_type: SID_NAME_USE = SidTypeUnknown as SID_NAME_USE;
        let e_sid_type_p: PSID_NAME_USE = &mut e_sid_type as PSID_NAME_USE;

        let ret: BOOL =
            LookupAccountSidW(
                ptr::null_mut(),
                *sid,
                account_name.as_mut_ptr(),
                account_name_len_p,
                dom_name.as_mut_ptr(),
                dom_name_len_p,
                e_sid_type_p
            );


        let last_error = GetLastError();
        if ret == 0 && last_error != ERROR_INSUFFICIENT_BUFFER {
            println!("failed to lookup account name with {:?}", last_error);
            return "".to_string()
        };

        let mut acc_name = String::from_utf16_lossy(&account_name).to_string();

        while acc_name.ends_with("\u{0}") {
            let len = acc_name.len();
            let new_len = len.saturating_sub("\u{0}".len());
            acc_name.truncate(new_len);
        }

        acc_name
    }
}