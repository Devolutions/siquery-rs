use std::{
    ptr
};
use winapi::{
    shared::{
        minwindef::{
            DWORD,
            PDWORD,
            LPBYTE,
            LPDWORD,
            PUCHAR,
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
            LPWSTR,
        },
    },
    um::{
        lmaccess::{
            NetLocalGroupEnum,
            LPLOCALGROUP_INFO_1,
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
        securitybaseapi::{
            IsValidSid,
            GetSidSubAuthority,
            GetSidSubAuthorityCount
        },
        errhandlingapi::GetLastError
    },
};
use widestring::WideString;
use libc;

use tables::{
    GroupsRow,
};
use windows::processes;

#[allow(non_upper_case_globals)]
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

pub fn get_rid_from_sid(sid_p: PSID) -> i64 {
    let count_p: PUCHAR = unsafe {GetSidSubAuthorityCount(sid_p)};
    let count = unsafe{*count_p};
    if count > 0 {
        let rid_p: PDWORD = unsafe {
            GetSidSubAuthority(
                sid_p,
                count as u32 - 1
            )
        };
        return unsafe{*rid_p} as i64;
    } else {
        println!("failed to find rid, acces denied");
        return 0i64;
    }
}

impl GroupsRow {
    pub fn get_specific () -> Vec<GroupsRow> {
        let mut out = Vec::<GroupsRow>::new();

        // Parameters.
        let group_info_level = 1u32;
        let mut entries_read = 0u32;
        let entries_read_p: LPDWORD = &mut entries_read as LPDWORD;
        let mut total_entries = 0u32;
        let total_entries_p: LPDWORD = &mut total_entries as LPDWORD;
        let mut buf = Vec::<u8>::with_capacity(MAX_PREFERRED_LENGTH as usize);
        let buf_ptr = buf.as_mut_ptr();

        let ret = unsafe {
            NetLocalGroupEnum (
                ptr::null(),
                group_info_level,
                buf_ptr as *mut LPBYTE,
                MAX_PREFERRED_LENGTH,
                entries_read_p,
                total_entries_p,
                ptr::null_mut()
            )
        };

        if buf_ptr == ptr::null_mut() || (ret != NERR_Success && ret != ERROR_MORE_DATA) {
            println!("NetLocalGroupEnum() failed, the return value is : {}", ret);
            return Vec::new()
        }

        for entry in 0..entries_read {
            // This loop iterates over all LOCALGROUP_INFO_1 structures in buf_ptr.

            // Read buf_ptr.
            let local_group_info_buf_p: LPLOCALGROUP_INFO_1 = unsafe {
                ptr::read(buf_ptr as *mut _)
            };
            // Offset the pointer.
            let local_group_info_1 = unsafe {
                (*local_group_info_buf_p.add(entry as usize))
            };
            let lgrpi1_name = local_group_info_1.lgrpi1_name;
            let lgrpi1_comment = local_group_info_1.lgrpi1_comment;

            let sid_p: PSID = get_sid_from_username(lgrpi1_name);

            if sid_p != ptr::null_mut(){
                // Read from buffers.
                let groupname = unsafe {
                    WideString::from_ptr(
                        lgrpi1_name,
                        libc::wcslen(lgrpi1_name)
                    )
                };
                let comment = unsafe {
                    WideString::from_ptr(
                        lgrpi1_comment,
                        libc::wcslen(lgrpi1_comment)
                    )
                };

                out.push(
                    GroupsRow{
                        gid: get_rid_from_sid(sid_p),
                        gid_signed: get_rid_from_sid(sid_p),
                        groupname: groupname.to_string().unwrap_or("".to_string()),
                        group_sid: processes::sid_to_string(sid_p).unwrap_or("".to_string()),
                        comment: comment.to_string().unwrap_or("".to_string())
                    }
                );
            } else {
                let groupname = unsafe {
                    WideString::from_ptr(
                        lgrpi1_name,
                        libc::wcslen(lgrpi1_name)
                    )
                };
                print!("Failed to find sid from LookupAccountNameW for group: {:?}", groupname);
            }
        }
        out
    }
}