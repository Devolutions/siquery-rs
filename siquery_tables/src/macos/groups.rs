use libc::{
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

        //https://stackoverflow.com/questions/49833059/list-all-the-users-logged-in-a-specific-group
        //TODO: store all groupnames in a set and use getgrnam(groupname) instead.
        while group_p != ptr::null_mut() {
            let groupname = unsafe{CStr::from_ptr((*group_p).gr_name).to_str().unwrap_or("")};
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

        let s = ODSession {
            ptr: ptr::null_mut()
        };
        println!("{:?}", s.defaultSession());

        let err = ptr::null_mut();    // Size of NSError is 40.

        let root = ODNode {
            ptr: ptr::null_mut()
        };
        root.get_root(s,err);

        if err != ptr::null_mut() {
            println!("Error getting roots");
        }


        println!("{:?}", err);

        out
    }
}

use objc::runtime::{Object};
#[derive(Debug)]
pub struct ODSession { ptr: *mut Object }
impl ODSession {
    pub fn defaultSession(&self) -> Self {
        ODSession {
            ptr: unsafe { msg_send![class!(ODSession), defaultSession]
            }
        }
    }
}

#[derive(Debug)]
pub struct ODNode { ptr: *mut Object }
impl ODNode {
    pub fn get_root(&self, s: ODSession, err: *mut u8) -> Self {
        ODNode {
            ptr: unsafe { msg_send![class!(ODNode), nodeWithSession:s.ptr name:NSString::from("/Local/Default") error:err]
            }
        }
    }
}

use libc;
#[derive(Debug)]
pub struct NSString { ptr: *mut Object }
impl NSString {
    pub fn from(content: &str) -> Self {
        NSString {
            ptr: unsafe {
                let string: *mut Object = msg_send![class!(NSString), alloc];
                msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:4]  // 32 or 64 bit here? should be os dependent
            }
        }
    }
}

#[derive(Debug)]
pub struct ODQuery { ptr: *mut Object }
impl ODQuery {
    pub fn get_root(&self, s: ODQuery, err: *mut u8) -> Self {
        ODQuery {
            ptr: unsafe { msg_send![class!(ODQuery), queryWithNode:root
                       forRecordTypes:list  // cannot use forRecordTypes:type because of rust's compiler.
                            attribute:kODAttributeTypeUniqueID  // must be an NSArray object containing NSString objects for multiple types
                            matchType:kODMatchEqualTo
                          queryValues:nil
                     returnAttributes:kODAttributeTypeStandardOnly
                       maximumResults:0
                                error:&err]
            }
        }
    }
}