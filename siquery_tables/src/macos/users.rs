use libc::{
    getpwnam,
    c_void,
    c_int,
    uid_t,
    c_char,
    c_uchar
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

use tables::Users;

extern "C" {
    fn uuid_unparse(uu: *mut c_uchar, out: *mut c_char) -> c_int;
    fn mbr_uid_to_uuid(id: uid_t, uu: *mut u8) -> c_int;
}

impl Users {
    pub fn get_specific() -> Vec<Users> {
        if let Some(vec) = gen_od_entreies() {
            return vec
        } else {
            return Vec::new()
        }
    }
}

// Objective-C interface to query Open Directory entries.
pub fn gen_od_entreies () -> Option<Vec<Users>> {
    let mut out = Vec::new();
    let mut usernames = HashSet::new();  // TODO: Is a set really necessary here? Does ODQuery return double entries?

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
        let username = unsafe {
            CStr::from_ptr(name_c).to_string_lossy().into_owned()
        };

        if usernames.insert(username.clone()){   // True when the value was added.
            let pwd_p = unsafe {
                getpwnam(name_c)
            };
            if pwd_p == ptr::null_mut() {
                return None
            }
            let pwd = unsafe {*pwd_p};

            // Get uuid string.
            let mut uuid= [0u8;16];
            unsafe{mbr_uid_to_uuid(pwd.pw_uid.clone(),&mut uuid as *mut _ as *mut _)};
            let mut uuid_string = [0i8;37];
            unsafe{uuid_unparse(&mut uuid as *mut _ as *mut _, &mut uuid_string as *mut _ as *mut _)};

            out.push(
                Users {
                    uid : pwd.pw_uid as i64,
                    gid: pwd.pw_gid as i64,
                    uid_signed: pwd.pw_uid as i32 as i64,
                    gid_signed: pwd.pw_gid as i32 as i64,
                    username,
                    description: unsafe{CStr::from_ptr(pwd.pw_gecos).to_string_lossy().into_owned()},
                    directory: unsafe{CStr::from_ptr(pwd.pw_dir).to_string_lossy().into_owned()},
                    shell: unsafe{CStr::from_ptr(pwd.pw_shell).to_string_lossy().into_owned()},
                    uuid: unsafe{CStr::from_ptr(uuid_string.as_ptr()).to_string_lossy().into_owned()},
                    type_: "0".to_string(),
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
            let od_record_type = NSString::from("dsRecTypeStandard:Users");
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