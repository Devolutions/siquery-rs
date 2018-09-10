use std::ffi::CStr;
use libc::{c_int,c_char,statfs};
use std::{ptr,slice};

use tables::MountsRow;

extern "C" {
    pub fn getmntinfo (mntbufp: *mut *mut statfs, flags: c_int) -> c_int;
    pub fn canonicalize_file_name (name : *const c_char) -> *mut c_char;
}

const MNT_WAIT : i32 = 1;

impl MountsRow {

    pub fn get_specific() -> Vec<MountsRow> {
        let mut mptr: *mut statfs = ptr::null_mut();
        let len = unsafe {
            getmntinfo(&mut mptr,MNT_WAIT)
        } as usize;
        let mounts = unsafe { slice::from_raw_parts(mptr, len) };
        println!("{:?}", mounts.iter().map(|m| m.f_bavail).collect::<Vec<_>>());

        if len == 0 {
            return Vec::new()
        } else {
            let mut out = Vec::new();
            //let mnt = slice::from_raw_parts(mntbufp,ret_val);
/*
            for i in 0..ret_val {  //tODO retval or retval-1?
                //use std::ffi::CString;
                //println!("{:?}",CString::from_raw(&mnt[i].f_fstypename));

                out.push (
                    MountsRow {
                        device: CStr::from_ptr(mnt[i].f_mntfromname).to_str().unwrap_or("invalid data").to_owned(),
                        device_alias: CStr::from_ptr(canonicalize_file_name(mnt[i].f_mntfromname)).to_str().unwrap_or("invalid data").to_owned(),
                        path: CStr::from_ptr(mnt[i].f_mntonname).to_str().unwrap_or("invalid data").to_owned(),
                        device_type: CStr::from_ptr(mnt[i].f_fstypename).to_str().unwrap_or("invalid data").to_owned(),
                        blocks_size: mnt[i].f_bsize as i64,
                        blocks: mnt[i].f_block as i64,
                        blocks_free: mnt[i].f_bfree as i64,
                        blocks_available: mnt[i].f_bavail as i64,
                        inodes: mnt[i].f_files as i64,
                        inodes_free: mnt[i].f_ffree as i64,
                        flags: CStr::from_ptr(mnt[i].f_flags).to_str().unwrap_or("invalid data").to_owned(),
                        //owner: mnt[i].f_owner as i64,
                    }
                )
            }
*/
            out
        }
    }
}