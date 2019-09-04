use crate::tables::Users;
use libc::{getpwent, passwd,endpwent,c_char};
use std::{
    ffi::CStr,
    sync::{Mutex,RwLock},
    str,
    ptr
};

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
        let mut _pwd: *mut passwd  = ptr::null_mut();
        let pwd_enumeration_mutex = Mutex::new(0_u32);
        unsafe {
            let _lock = RwLock::new(pwd_enumeration_mutex);
            _pwd = getpwent();
            while _pwd != ptr::null_mut() {
                gen_users(&mut users, _pwd);
                _pwd = getpwent();
            }
            endpwent();
        }
        users
    }
}

pub fn gen_users(users: &mut Vec<Users>, pwd: *mut passwd) {
    let mut user = Users::new();
    unsafe {
        user.uid = (*pwd).pw_uid as i64;
        user.gid = (*pwd).pw_gid as i64;
        user.uid_signed = (*pwd).pw_uid as i32 as i64;
        user.gid_signed = (*pwd).pw_gid as i32 as i64;
        if !(*pwd).pw_name.is_null() {
            user.username = c_string_to_string((*pwd).pw_name);
        }
        if !(*pwd).pw_gecos.is_null() {
            user.description = c_string_to_string((*pwd).pw_gecos);
        }
        if !(*pwd).pw_dir.is_null() {
            user.directory = c_string_to_string((*pwd).pw_dir);
        }
        if !(*pwd).pw_shell.is_null() {
            user.shell = c_string_to_string((*pwd).pw_shell);
        }
        user.uuid = "".to_string();
        user.type_ = "".to_string();
    }
    users.push(user);
}

pub fn c_string_to_string(c_string : *const c_char) -> String {
    let c_buf: *const c_char = c_string;
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    str_buf
}
