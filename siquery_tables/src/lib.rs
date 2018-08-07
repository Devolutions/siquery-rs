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


pub mod vtab;
mod common;
mod utils;
pub mod table_printer;
pub mod query;
pub mod tables;

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

        pub mod linux;
        pub use linux as sys;
    }else if #[cfg(target_os = "macos")] {
        extern crate plist;
        extern crate uname;
        extern crate libc;
        extern crate byteorder;

        pub mod macos;
        pub use macos as sys;
    } else if #[cfg(target_os = "windows")] {
        extern crate kernel32;
        #[macro_use]
        extern crate winapi;

        pub mod windows;
        pub use windows as sys;
    }
}