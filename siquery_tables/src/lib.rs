#![recursion_limit="128"]
#[macro_use]
extern crate cfg_if;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate rusqlite;
extern crate csv;
extern crate prettytable;
extern crate proxy_config;
extern crate treexml;
extern crate heck;
extern crate chrono;
#[macro_use]
extern crate horrorshow;

pub mod vtab;
mod common;
mod utils;
pub mod printer;
pub mod query;
pub mod tables;
pub mod html;

cfg_if! {
    if #[cfg(all(target_os = "linux",fuzzing))] {
        extern crate sysconf;
        extern crate uname;

        pub mod windows;
        pub mod linux;
        pub use linux as sys;
    } else if #[cfg(all(target_os = "linux",not(fuzzing)))] {
        extern crate sysconf;
        extern crate uname;
        extern crate nix;
        extern crate libc;

        pub mod linux;
        pub use linux as sys;
    }else if #[cfg(target_os = "macos")] {
        extern crate plist;
        extern crate uname;
        extern crate libc;
        extern crate byteorder;
        #[macro_use]
        extern crate objc;
        extern crate objc_foundation;
        extern crate objc_id;
        extern crate walkdir;
        extern crate glob;

        pub mod macos;
        pub use macos as sys;
    } else if #[cfg(target_os = "windows")] {
        extern crate kernel32;
        extern crate winreg;
        #[macro_use]
        extern crate winapi;
        extern crate widestring;
        extern crate libc;

        pub mod windows;
        pub use windows as sys;
        pub mod inventory;
    }
}