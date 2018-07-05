#![recursion_limit="128"]
#[macro_use]
extern crate cfg_if;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate time;

mod common;
mod tables;
mod utils;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern crate sysconf;
        extern crate uname;

        pub mod linux;
        pub use linux as sys;
    } else if #[cfg(target_os = "macos")] {
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

pub use sys::SystemInfo;
