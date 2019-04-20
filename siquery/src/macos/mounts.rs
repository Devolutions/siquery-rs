use std::ffi::CStr;
use libc::{ c_int,statfs };
use std::{ ptr,slice };

use crate::tables::MountsRow;

extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "getmntinfo$INODE64")]
    pub fn getmntinfo (mntbufp: &*mut statfs, flags: c_int) -> c_int;
}

static MNT_WAIT : i32 = 1;

impl MountsRow {
    pub fn get_specific() -> Vec<MountsRow> {
        let mptr: *mut statfs = ptr::null_mut();
        let len = unsafe { getmntinfo(&mptr,MNT_WAIT) } as usize;
        let mnt = unsafe { slice::from_raw_parts(mptr, len) };
        if len == 0 {
            return Vec::new()
        } else {
            let mut out = Vec::new();
            for i in 0..len {
                out.push (
                    MountsRow {
                        device: unsafe {
                            CStr::from_ptr(
                                &mnt[i].f_mntfromname as *const _
                            ).to_str().unwrap_or("invalid data").to_owned()
                        },
                        device_alias: unsafe {
                            CStr::from_ptr(
                                &mnt[i].f_mntfromname as *const _
                            ).to_str().unwrap_or("invalid data").to_owned()
                        },
                        path: unsafe {
                            CStr::from_ptr(
                                &mnt[i].f_mntonname as *const _
                            ).to_str().unwrap_or("invalid data").to_owned()
                        },
                        device_type: unsafe {
                            CStr::from_ptr(
                                &mnt[i].f_fstypename as *const _
                            ).to_str().unwrap_or("invalid data").to_owned()
                        },
                        blocks_size: mnt[i].f_bsize as i64,
                        blocks: mnt[i].f_blocks as i64,
                        blocks_free: mnt[i].f_bfree as i64,
                        blocks_available: mnt[i].f_bavail as i64,
                        inodes: mnt[i].f_files as i64,
                        inodes_free: mnt[i].f_ffree as i64,
                        flags: mnt[i].f_flags.to_string(),
                    }
                )
            }
            out
        }
    }
}