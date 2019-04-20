use libc::{
    getgrent,
    setgrent,
    endgrent,
    getgrnam,
    group,
    c_void
};
use std::{
    ptr,
    ffi::CStr
};
use std::collections::HashSet;
use objc::runtime::{
    Class,
    Object
};
use objc_foundation::{
    INSArray,
    NSArray,
    NSObject,
    INSObject
};
use objc_id::Id;

use crate::tables::GroupsRow;

impl GroupsRow {
    pub fn get_specific() -> Vec<GroupsRow> {
        if let Some(vec) = gen_od_entreies() {
            return vec
        } else if let Some(vec) = gen_grgent_entries() {
            return vec
        } else {
            return Vec::new()
        }
    }
}

// If Objective-C interface fails.
pub fn gen_grgent_entries () -> Option<Vec<GroupsRow>> {
    println!("Objective-C interface failes, querying with getgrent().");
    let mut out = Vec::new();
    let mut groupnames = HashSet::new();
    unsafe {setgrent()};
    let mut group_p: *mut group = unsafe {getgrent()};
    /*The getgrent() function searches all available directory services on it's
    first invocation.  It caches the returned entries in a list and returns
    group entries one at a time.
    NOTE that getgrent() may cause a very lengthy search for group records by
    opendirectoryd and may result in a large number of group records being
    cached by the calling process.  Use of this function is not advised.*/

    //https://stackoverflow.com/questions/49833059/list-all-the-users-logged-in-a-specific-group
    while group_p != ptr::null_mut() {
        let groupname = unsafe{CStr::from_ptr((*group_p).gr_name).to_str().unwrap_or("")};
        if groupnames.insert(groupname){   // true when the value was added
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
    Some(out)
}

// Objective-C interface to query Open Directory entries.
pub fn gen_od_entreies () -> Option<Vec<GroupsRow>> {
    let mut out = Vec::new();
    let mut groupnames = HashSet::new();  // TODO: Is a set really necessary here? Does ODQuery return double entries?

    let s = ODSession::new().default_session();
    let root = ODNode::new().get_root(s).unwrap_or(ODNode::new());
    let q = ODQuery::new().query(root).unwrap_or(ODQuery::new());
    let od_results = q.get_od_results().unwrap_or(NSArray::new());

    // Extract ODRecords from the NSArray.
    let vec = od_results.objects_in_range(0..unsafe { msg_send![od_results, count] });

    for i in 0..vec.len() {
        // Get the record name.
        let name_obj_c = NSString {
            ptr: unsafe {
                msg_send![vec[i], recordName]
            }
        };
        // Convert into a rust String.
        let name_c : *const i8 = unsafe {
            msg_send![name_obj_c.ptr, UTF8String]
        };
        let groupname = unsafe {
            CStr::from_ptr(name_c).to_string_lossy().into_owned()
        };

        if groupnames.insert(groupname.clone()){   // True when the value was added.
            let gr = unsafe {
                getgrnam(name_c)
            };
            out.push(
                GroupsRow {
                    gid: unsafe {*gr}.gr_gid as i64,
                    gid_signed: unsafe {*gr}.gr_gid as i32 as i64,
                    groupname,
                    group_sid: "".to_string(),
                    comment: "".to_string()
                }
            );
        }
    }
    Some(out)   // FIXME: Prevent Objective-C errors from unwinding in rust!
}

// Objective-C structures.
pub struct ODSession { ptr: *mut Object }
impl ODSession {
    pub fn new() -> Self {
        ODSession {
            ptr: ptr::null_mut()
        }
    }
    pub fn default_session(&self) -> Self {
        ODSession {
            ptr: unsafe { msg_send![class!(ODSession), defaultSession]
            }
        }
    }
}
pub struct ODNode { ptr: *mut Object }
impl ODNode {
    pub fn new() -> Self {
        ODNode {
            ptr: ptr::null_mut()
        }
    }
    pub fn get_root(&self, s: ODSession) -> Result<Self, String> {
        let err : *mut c_void = ptr::null_mut();
        let root = unsafe { msg_send![class!(ODNode), nodeWithSession:s.ptr name:NSString::from("/Local/Default") error:err] };
        if err != ptr::null_mut() {
            Err("Error getting roots".to_string())
        } else {
            Ok(ODNode { ptr: root})
        }
    }
}
pub struct NSString { ptr: *mut Object }
impl NSString {
    pub fn from(content: &str) -> Self {
        NSString {
            ptr: unsafe {
                let string: *mut Object = msg_send![class!(NSString), alloc];
                msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:4]  // TODO: 32 or 64 bit here? should be os dependent
            }
        }
    }
}
pub struct ODQuery { ptr: *mut Object }
impl ODQuery {
    pub fn new() -> Self {
        ODQuery {
            ptr: ptr::null_mut()
        }
    }
    pub fn query(&self, root: ODNode) -> Result<Self, String> {
        #[allow(non_snake_case)] {
            let od_record_type = NSString::from("dsRecTypeStandard:Groups");
            let kODAttributeTypeUniqueID = NSString::from("dsAttrTypeStandard:UniqueID");
            let kODMatchEqualTo : i64 = 0x2001;
            let nil : &Class = class!(NSNull);
            let kODAttributeTypeStandardOnly = NSString::from("dsAttributesStandardAll");

            let err : *mut c_void = ptr::null_mut();
            let query = unsafe { msg_send![class!(ODQuery), queryWithNode:root.ptr
                       forRecordTypes:od_record_type
                            attribute:kODAttributeTypeUniqueID
                            matchType:kODMatchEqualTo
                          queryValues:nil
                     returnAttributes:kODAttributeTypeStandardOnly
                       maximumResults:0
                                error:&err]
            };
            if err != ptr::null_mut() {
                Err("Error getting roots".to_string())
            } else {
                Ok(
                    ODQuery {
                    ptr: query
                    }
                )
            }
        }
    }
    pub fn get_od_results(&self) -> Result<Id<NSArray<NSObject>>, String> {
        let err : *mut c_void = ptr::null_mut();
        let od_results : Id<NSArray<NSObject>>  = unsafe {
            msg_send![self.ptr, resultsAllowingPartial:0 error:&err]
        };
        if err != ptr::null_mut() {
            Err("Error getting results".to_string())
        } else {
            Ok(od_results)
        }
    }
}