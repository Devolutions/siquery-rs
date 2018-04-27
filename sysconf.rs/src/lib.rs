//! Query runtime configuration information.
//!
//! This crate provides the ability to query for various configuration information about the
//! runtime platform such as memory page size. On POSIX systems, it makes heavy use of the
//! [sysconf] API.
//!
//! [sysconf]: http://man7.org/linux/man-pages/man3/sysconf.3.html

// NOTE: According to Wikipedia, "Originally, the name "POSIX" referred to IEEE Std 1003.1-1988,
// released in 1988." This crate assumes that any behavior required by POSIX 1003.1 will be
// properly implemented on any POSIX system. Running on a POSIX system which does not adhere to
// these requirements will cause either an assertion failure/panic or undefined behavior.

#![cfg_attr(not(test), no_std)]

#[cfg(test)] // In tests, we disable no_std, so core isn't automatically included
extern crate core;
// no-std lazy_static requires nightly
// TODO: Use lazy_static unconditionally once it works on stable
#[cfg(feature = "nightly")]
#[macro_use]
extern crate lazy_static;

pub mod page;
#[cfg(unix)]
pub mod raw;

pub use page::*;
#[cfg(unix)]
pub use raw::*;
