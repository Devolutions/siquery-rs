//! Query for configuration related to memory pages.
//!
//! # Huge Pages
//! Some systems support "huge pages" that are larger than the system's default page size. In the
//! documentation for this module, we use the Linux name of "huge pages," although they go by
//! different names on different platforms - superpages on Mac, and large pages on Windows.

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

mod hugepage;
pub use self::hugepage::*;

/// Get the system's page size.
#[cfg(any(unix, windows))]
#[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
#[inline(always)]
#[cfg(feature = "nightly")]
pub fn pagesize() -> usize {
    *PAGESIZE
}

/// Get the system's page size.
#[cfg(any(unix, windows))]
#[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
#[inline(always)]
#[cfg(not(feature = "nightly"))]
pub fn pagesize() -> usize {
    priv_pagesize()
}

#[cfg(feature = "nightly")]
#[cfg(any(unix, windows))]
lazy_static!{ static ref PAGESIZE: usize = priv_pagesize(); }

#[cfg(unix)]
fn priv_pagesize() -> usize {
    // sysconf(_SC_PAGESIZE) is required by POSIX 1003.1: http://www.unix.com/man-page/posix/3p/sysconf/
    ::raw::sysconf(::raw::SysconfVariable::ScPagesize).expect("sysconf(_SC_PAGESIZE) failed, but _SC_PAGESIZE is required by POSIX 1003.1") as usize
}

#[cfg(windows)]
fn priv_pagesize() -> usize {
    use self::kernel32::GetSystemInfo;
    use self::winapi::SYSTEM_INFO;
    use core::mem::uninitialized;
    unsafe {
        let mut info = uninitialized::<SYSTEM_INFO>();
        GetSystemInfo(&mut info);
        info.dwPageSize as usize
    }
}
