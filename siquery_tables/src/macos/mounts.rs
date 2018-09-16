use std::ffi::CStr;
use libc::{ c_int,c_char,uid_t,int32_t};
use std::{ ptr,slice };
use tables::MountsRow;

extern "C" {
    pub fn getmntinfo (mntbufp: &*mut statfs, flags: c_int) -> c_int;
}

#[repr(C)]
struct fsid_t {
    val: [int32_t; 2],
}

cfg_if! {
    // Since Mac OSX 10.5 the struct statfs was updated to accommodate for wider f_fstypename,
    // f_mntonname, and f_mntfromname fields. To ensure reliable output interpretation and proper
    // allignment, the statfs struct should be defined differently when the
    // _DARWIN_FEATURE_64_BIT_INODE macro is defined.
    // https://www.unix.com/man-page/osx/2/stat/

    if #[cfg(not(env = "darwin_feature_64_bit_inode"))] {
        use libc::{c_short,c_long};

        const MFSNAMELEN: usize = 15;
        const MNAMELEN: usize = 90;

        #[repr(C)]
        // Use legacy statfs for 32 bit inodes.
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
    }
    else if #[cfg(env = "darwin_feature_64_bit_inode")] {
        use libc::{uint32_t,uint64_t};

        const MFSTYPENAMELEN : usize = 16;
        const MAXPATHLEN : usize = 1024;

        #[repr(C)]
        // Use new statfs for 64 bit inodes.
        pub struct statfs {
            f_bsize: uint32_t,
            f_iosize: int32_t,
            f_blocks: uint64_t,
            f_bfree: uint64_t,
            f_bavail: uint64_t,
            f_files: uint64_t,
            f_ffree: uint64_t,
            f_fsid:	fsid_t,
            f_owner: uid_t,
            f_type:	uint32_t,
            f_flags: uint32_t,
            f_fssubtype: uint32_t,
            f_fstypename: [c_char;MFSTYPENAMELEN],
            f_mntonname: [c_char;MAXPATHLEN],
            f_mntfromname: [c_char;MAXPATHLEN],
            f_reserved: [uint32_t;8],
        }
    }
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