use std::ffi::CStr;
use libc::{ c_long,c_short,c_int,c_char,uid_t,int32_t };
use std::{ ptr,slice };
use tables::MountsRow;

extern "C" {
    pub fn getmntinfo (mntbufp: &*mut statfs, flags: c_int) -> c_int;
}

const MFSNAMELEN: usize = 15;
const MNAMELEN: usize = 90;

#[repr(C)]
struct fsid_t {
    val: [int32_t; 2],
}

#[repr(C)]
pub struct statfs {
    f_otype: c_short,
    f_oflags: c_short,
    f_bsize: c_long,
    f_iosize: c_long,
    f_blocks: c_long,
    f_bfree: c_long,
    f_bavail: c_long,
    f_files: c_long,
    f_ffree: c_long,
    f_fsid: fsid_t,
    f_owner: uid_t,
    f_reserved1: c_short,
    f_type: c_short,
    f_flags: c_long,
    f_reserved2: [c_long;2],
    f_fstypename: [c_char;MFSNAMELEN],
    f_mntonname: [c_char;MNAMELEN],
    f_mntfromname: [c_char;MNAMELEN],
    f_reserved3: c_char,
    f_reserved4: [c_long;4],
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