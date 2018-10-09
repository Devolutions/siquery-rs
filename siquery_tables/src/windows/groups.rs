use std::{
    ptr,
    mem
};
use winapi::{
    shared::{
        minwindef::{
            DWORD,
            LPBYTE,
            LPDWORD,
            FALSE,
            BOOL
        },
        winerror::{
            ERROR_MORE_DATA,
            ERROR_INSUFFICIENT_BUFFER
        },
        lmcons::MAX_PREFERRED_LENGTH,
        ntdef::{
            LPCWSTR,
            LPWSTR
        }
    },
    um::{
        lmaccess::{
            NetLocalGroupEnum,
            LPLOCALGROUP_INFO_1,
            LOCALGROUP_INFO_1
        },
        winnt::{
            PSID,
            SID_NAME_USE,
            PSID_NAME_USE,
            SidTypeUnknown
        },
        winbase::{
            LookupAccountNameW
        },
        securitybaseapi::IsValidSid,
        errhandlingapi::GetLastError
    }
};
use widestring::WideString;
use libc;

use tables::{
    GroupsRow,
    ProcessesRow
};
use utils;

static NERR_Success: DWORD = 0;

pub fn get_sid_from_username(account_name: LPCWSTR) -> PSID {
    //? if account_name is empty print "No account name provided"
    // return ptr_null_mut();

    // Parameters.
    let mut sid_buf_size = 0u32;
    let sid_buf_size_p: LPDWORD = &mut sid_buf_size as LPDWORD;
    let mut dom_name_size = 0u32;
    let dom_name_size_p: LPDWORD = &mut dom_name_size as LPDWORD;
    let mut e_sid_type: SID_NAME_USE = SidTypeUnknown as SID_NAME_USE;
    let e_sid_type_p: PSID_NAME_USE = &mut e_sid_type as PSID_NAME_USE;

    // Get the buffers sizes.
    let mut ret: BOOL =  unsafe {
        LookupAccountNameW(
            ptr::null_mut(),
            account_name,
            ptr::null_mut(),
            sid_buf_size_p,
            ptr::null_mut(),
            dom_name_size_p,
            e_sid_type_p
        )
    };

    if ret == 0 && unsafe {GetLastError()} != ERROR_INSUFFICIENT_BUFFER {
        println!("failed to lookup account name with error {}",/*account_name.to_string(),*/ unsafe {GetLastError()});
        return ptr::null_mut();
    };

    // Buffers.
    let mut sid_buf: Vec<u16> = Vec::with_capacity(sid_buf_size as usize);
    let sid_buf_p: PSID = sid_buf.as_mut_ptr() as PSID;
    let mut dom_name: Vec<u16> = Vec::with_capacity(dom_name_size as usize);
    let dom_name_p: LPWSTR = dom_name.as_mut_ptr() as LPWSTR;

    ret = unsafe {
        LookupAccountNameW (
            ptr::null_mut(),
            account_name,
            sid_buf_p,
            sid_buf_size_p,
            dom_name_p,
            dom_name_size_p,
            e_sid_type_p
        )
    };

    if ret == 0 {
        println!("failed to lookup account name with error {}", /*account_name.to_string(),*/ unsafe { GetLastError() });
        return ptr::null_mut();
    } else if unsafe { IsValidSid(sid_buf_p)} == FALSE {
        println!("The sid for is invalid"/*, account_name.to_string()*/);
        return ptr::null_mut();
    }
    sid_buf_p
}

impl GroupsRow {
    pub fn get_specific () -> Vec<GroupsRow> {
        let mut out = Vec::<GroupsRow>::new();

        // Parameters.
        let mut group_info_level = 1u32;
        let mut entries_read = 0u32;
        let entries_read_p: LPDWORD = &mut entries_read as LPDWORD;
        let mut total_entries = 0u32;
        let total_entries_p: LPDWORD = &mut total_entries as LPDWORD;
        //let mut buf_ptr : LOCALGROUP_INFO_1 = unsafe {mem::zeroed()};
        let mut buf = Vec::<u8>::with_capacity(MAX_PREFERRED_LENGTH as usize);
        let mut buf_ptr = buf.as_mut_ptr();

        let ret = unsafe {
            NetLocalGroupEnum (
                ptr::null(),
                group_info_level,
                buf_ptr as *mut LPBYTE,
                MAX_PREFERRED_LENGTH,
                entries_read_p,  //*mut DWORD
                total_entries_p, //*mut DWORD
                ptr::null_mut()
            )
        };
        println!("{}", unsafe {GetLastError()});

        if buf_ptr == ptr::null_mut() || (ret != NERR_Success && ret != ERROR_MORE_DATA) {
            println!("NetLocalGroupEnum() failed, the return value is : {}", ret);
            return Vec::new()
        }

        // entry here is LOCALGROUP_INFO_1

        for entry in 0..entries_read {
            //let local_group_info: LPLOCALGROUP_INFO_1 = unsafe { ptr::read(buf_ptr as *mut _) }; // might be unnecessary becausse of line 10
            // TODO offset to the next struct and read it.
            let local_group_info_1_p: LPLOCALGROUP_INFO_1 = unsafe { ptr::read(buf_ptr as *mut _) };
            let sid_p: PSID = get_sid_from_username(unsafe{*local_group_info_1_p}.lgrpi1_name);  // /!\ TODO ->  sid_p as *mut _ as PSID; .... the fn returns a LOCALGROUP_INFO_1 struct where lgrpi1_name: ::LPWSTR but want PSID

            if sid_p != ptr::null_mut(){ // null_ptr or null_ptr_mut?
                // Read from buffers.
                let groupname_size = unsafe { libc::wcslen(unsafe{*local_group_info_1_p}.lgrpi1_name) };
                let groupname_string = unsafe { WideString::from_ptr(unsafe{*local_group_info_1_p}.lgrpi1_name, groupname_size) };
                let comment_size = unsafe { libc::wcslen(unsafe{*local_group_info_1_p}.lgrpi1_comment) };
                let comment_string = unsafe { WideString::from_ptr(unsafe{*local_group_info_1_p}.lgrpi1_comment, comment_size) };
                out.push(
                    GroupsRow{
                        gid: -22,// TODO get_rid_from_sid(sid_p),
                        gid_signed: -22, // TODO get_rid_from_sid(sid_p),
                        groupname: groupname_string.to_string().unwrap_or("".to_string()),
                        group_sid: utils::sid_to_string(sid_p).unwrap_or("".to_string()),
                        comment: comment_string.to_string().unwrap_or("".to_string())
                    }
                );
            } else {
                let groupname_size = unsafe { libc::wcslen(unsafe{*local_group_info_1_p}.lgrpi1_name) };
                let groupname_string = unsafe { WideString::from_ptr(unsafe{*local_group_info_1_p}.lgrpi1_name, groupname_size) };
                print!("Failed to find sid from LookupAccountNameW for group: {:?}", groupname_string);
            }
        }
        out
    }
}