use crate::tables::LoggedInUsers;
use libc::{getutxent, endutxent};
use libc::utmpx;
use libc::{EMPTY,
           BOOT_TIME,
           NEW_TIME,
           OLD_TIME,
           INIT_PROCESS,
           LOGIN_PROCESS,
           USER_PROCESS,
           DEAD_PROCESS,
           RUN_LVL,
           ACCOUNTING};

use std::{
    ffi::CStr,
    sync::{Mutex,RwLock},
    os::raw::c_char,
    str,
    ptr
};

lazy_static! {
    static ref K_LOGIN_TYPES : HashMap<i16, &'static str> = {
        let mut map = HashMap::new();
        map.insert(EMPTY, "empty");
        map.insert(BOOT_TIME, "boot_time");
        map.insert(NEW_TIME, "new_time");
        map.insert(OLD_TIME, "old_time");
        map.insert(INIT_PROCESS, "init");
        map.insert(LOGIN_PROCESS, "login");
        map.insert(USER_PROCESS, "user");
        map.insert(DEAD_PROCESS, "dead");
        map.insert(RUN_LVL, "runlevel");
        map.insert(ACCOUNTING, "accounting");
        map
};
}

use std::collections::HashMap;

impl LoggedInUsers {
    pub(crate) fn new() -> LoggedInUsers {
        LoggedInUsers {
            type_: String::new(),
            user: String::new(),
            tty: String::new(),
            host: String::new(),
            time: 0,
            pid: 0,
        }
    }

    pub fn get_specific() -> Vec<LoggedInUsers> {
        let mut logged_in_users: Vec<LoggedInUsers> = Vec::new();
        let mut _entry: *mut utmpx = ptr::null_mut();
        let utmpx_enumeration_mutex = Mutex::new(0_u32);
        unsafe {
            let _lock = RwLock::new(utmpx_enumeration_mutex);
            _entry = getutxent();
            while _entry != ptr::null_mut() {
                if (*_entry).ut_pid == 1 {
                    _entry = getutxent();
                    continue;
                }
                gen_logged_in_users(&mut logged_in_users, _entry);
                _entry = getutxent();
            }
            endutxent();
        }
        logged_in_users
    }
}

pub fn gen_logged_in_users(logged_in_users: &mut Vec<LoggedInUsers>, entry: *mut utmpx) {
    let mut logged_in_user = LoggedInUsers::new();
    unsafe{
        if *K_LOGIN_TYPES.get(&(*entry).ut_type).unwrap() == ""{
            logged_in_user.type_ = "unknown".to_string();
        }
        else
        {
            logged_in_user.type_ = (*K_LOGIN_TYPES.get(&(*entry).ut_type).unwrap()).to_string();
        }
        logged_in_user.user = c_char_arr_to_string(((*entry).ut_user).as_ptr()) ;
        logged_in_user.tty = c_char_arr_to_string( ((*entry).ut_line).as_ptr());
        logged_in_user.host = c_char_arr_to_string( ((*entry).ut_host).as_ptr());
        logged_in_user.time = (*entry).ut_tv.tv_sec as i64;
        logged_in_user.pid = (*entry).ut_pid as i64;
    }
    logged_in_users.push(logged_in_user);
}

pub fn c_char_arr_to_string(c_char_ptr : *const c_char) -> String {
    let str_ = unsafe { CStr::from_ptr(c_char_ptr)};
    let str_ = str_.to_owned();
    str_.into_string().unwrap()
}
