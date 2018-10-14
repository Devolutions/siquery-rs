use libc::{
    int32_t,
    getgrent,
    setgrent,
    endgrent,
    group
};
use std::{
    ptr,
    ffi::CStr
};
use std::collections::HashSet;

use tables::GroupsRow;

impl GroupsRow {
    pub fn get_specific() -> Vec<GroupsRow> {
        let mut out = Vec::new();
        let mut hash_set = HashSet::new();
        unsafe {setgrent()};
        let mut group_p: *mut group = unsafe {getgrent()};
        /*The getgrent() function searches all available directory services on it's
        first invocation.  It caches the returned entries in a list and returns
        group entries one at a time.
        NOTE that getgrent() may cause a very lengthy search for group records by
        opendirectoryd and may result in a large number of group records being
        cached by the calling process.  Use of this function is not advised.*/
        //TODO: store all groupnames in a set and use getgrnam(groupname) instead.
        while group_p != ptr::null_mut() {
            let groupname = unsafe{CStr::from_ptr(unsafe {*group_p}.gr_name).to_str().unwrap_or("")};
            if !hash_set.contains(groupname) {
                hash_set.insert(groupname);
                out.push(
                    GroupsRow {
                        gid: unsafe {*group_p}.gr_gid as i64,
                        gid_signed: unsafe {*group_p}.gr_gid as i32 as i64,
                        groupname: groupname.to_string(),
                        group_sid: "".to_string(),
                        comment: "".to_string()
                    }
                );
            }
            group_p = unsafe {getgrent()};
        }
        unsafe {endgrent()};
        out
    }
}
