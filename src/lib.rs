#[macro_use]
extern crate cfg_if;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

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

        pub mod macos;
        pub use macos as sys;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub use windows as sys;
    }
}

pub use sys::SystemInfo;
