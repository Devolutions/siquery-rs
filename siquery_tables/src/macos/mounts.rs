use std::ffi::CStr;
use libc::{ c_int,c_char,uid_t,int32_t,uint32_t,uint64_t,};
use std::{ ptr,slice };
use tables::MountsRow;

extern "C" {
    pub fn getmntinfo64 (mntbufp: &*mut statfs64, flags: c_int) -> c_int;
}

const MFSTYPENAMELEN : usize = 16;
const MAXPATHLEN : usize = 1024;

#[repr(C)]
struct fsid_t {
    val: [int32_t; 2],
}

#[repr(C)]
pub struct statfs64 {
    f_bsize:	     uint32_t,
    f_iosize:	     int32_t ,
    f_blocks:	     uint64_t,
    f_bfree:	     uint64_t,
    f_bavail:	     uint64_t,
    f_files:	     uint64_t,
    f_ffree:	     uint64_t,
    f_fsid:	         fsid_t  ,
    f_owner:	     uid_t	 ,
    f_type:	         uint32_t,
    f_flags:	     uint32_t,
    f_fssubtype:     uint32_t,
    f_fstypename:    [c_char;MFSTYPENAMELEN],
    f_mntonname:	 [c_char;MAXPATHLEN],
    f_mntfromname:   [c_char;MAXPATHLEN],
    f_reserved:      [uint32_t;8],
}

static MNT_WAIT : i32 = 1;

impl MountsRow {

    pub fn get_specific() -> Vec<MountsRow> {
        let mptr: *mut statfs64 = ptr::null_mut();
        let len = unsafe { getmntinfo64(&mptr,MNT_WAIT) } as usize;
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