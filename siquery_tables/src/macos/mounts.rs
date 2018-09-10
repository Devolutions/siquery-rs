use libc::statfs;

use tables::MountsRow;

extern "C" {
    pub fn getmntinfo (mntbufp: *const statfs, flags: i32) -> c_int;
    pub fn canonicalize_file_name (name : *const c_char) -> *mut c_char;
}

impl MountsRow {

    fn get_specific() -> Vec<MountsRow> {
        let mntbuf: *mut statfs;
        let ret_val = unsafe {
            getmntinfo(mntbuf,MNT_WAIT)
        };
        if ret_val == 0 {
            return Vec::new()
        } else {
            let mut out = Vec::new();
            for i in ret_val {
                let mnt = unsafe {*mntbuf};
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
        }
    }
}