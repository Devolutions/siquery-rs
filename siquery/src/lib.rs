#![recursion_limit="128"]
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
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
        pub mod windows;
        pub mod linux;
        pub use linux as sys;
    } else if #[cfg(all(target_os = "linux",not(fuzzing)))] {
        pub mod linux;
        pub use crate::linux as sys;
    } else if #[cfg(target_os = "macos")] {
        #[macro_use]
        extern crate objc;

        pub mod macos;
        pub use crate::macos as sys;
    } else if #[cfg(target_os = "windows")] {
        #[macro_use]
        extern crate winapi;

        pub mod windows;
        pub use crate::windows as sys;
        pub mod inventory;
    }
}