use libc::setmntent;    // Reads /proc/mounts and returns a pointer to be used by getmntent(). [1]
use libc::getmntent;    // Reads the next line and returns a mntent struct. [1]
use libc::mntent;
use libc::endmntent;       // Closes the file handle.
use libc::statfs;
use crate::tables::MountsRow;
use std::{ptr,mem};
use libc::FILE;
use std::os::raw::c_char;
use std::ffi::{CStr,CString};
// [1] https://linux.die.net/man/3/setmntent
// [2] https://www.gnu.org/software/libc/manual/html_node/Symbolic-Links.html

extern "C" {
    pub fn canonicalize_file_name (name : *const c_char) -> *mut c_char;
}

impl MountsRow {

    fn new() -> MountsRow {
        MountsRow {
            device: String::new(),
            device_alias: String::new(),
            path: String::new(),
            device_type: String::new(),
            blocks_size: 0i64,
            blocks: 0i64,
            blocks_free: 0i64,
            blocks_available: 0i64,
            inodes: 0i64,
            inodes_free: 0i64,
            flags: String::new(),
        }
    }

    fn get_specific_ex () -> Option<Vec<MountsRow>> {
        let mut out = Vec::new();
        let mounts: *mut FILE  = unsafe {
            setmntent(
                CString::new("/proc/mounts").ok()?.as_ptr() as *const i8,
                CString::new("r").ok()?.as_ptr() as *const i8
            )
        };
        if mounts == ptr::null_mut::<FILE>() {
            return {
                None
            };
        }
        #[allow(unused_assignments)]
        let mut ent_ptr: *mut mntent = ptr::null_mut::<mntent>();
        loop {
            ent_ptr = unsafe { getmntent(mounts) };
            if ent_ptr != ptr::null_mut() {
                let mut row = MountsRow::new();
                let ent = unsafe{ *ent_ptr };

                row.device = unsafe{
                    CStr::from_ptr(ent.mnt_fsname).to_str().unwrap_or("invalid data").to_owned()
                };
                row.path = unsafe{
                    CStr::from_ptr(ent.mnt_dir).to_str().unwrap_or("invalid data").to_owned()
                };
                row.device_type = unsafe{
                    CStr::from_ptr(ent.mnt_type).to_str().unwrap_or("invalid data").to_owned()
                };
                row.flags = unsafe{
                    CStr::from_ptr(ent.mnt_opts).to_str().unwrap_or("invalid data").to_owned()
                };

                let canonical_file_name_ptr = unsafe {
                    canonicalize_file_name(ent.mnt_fsname)
                };
                if canonical_file_name_ptr != ptr::null_mut() {
                    row.device_alias = unsafe{
                        CStr::from_ptr(canonical_file_name_ptr).to_str().unwrap_or("").to_owned()
                    };
                } else {
                    // canonicalize_file_name() returns ENOENT[2] on names without forward slashes.
                    row.device_alias = unsafe{
                        CStr::from_ptr(ent.mnt_fsname).to_str().unwrap_or("invalid data").to_owned()
                    };
                }

                let mut st: statfs = unsafe {mem::zeroed()};
                if unsafe{ statfs(ent.mnt_dir, &mut st as *mut _) == 0 } {
                    row.blocks_size = st.f_bsize as i64;
                    row.blocks = st.f_blocks as i64;
                    row.blocks_free = st.f_bfree as i64;
                    row.blocks_available = st.f_bavail as i64;
                    row.inodes = st.f_files as i64;
                    row.inodes_free = st.f_ffree as i64;
                }

                out.push(row)
            } else {
                break
            }
        }
        unsafe{ endmntent(mounts) };
        Some(out)
    }

    pub fn get_specific () -> Vec<MountsRow> {
        if let Some(result) = MountsRow::get_specific_ex() {
            return result
        }
        else {
            return Vec::new()
        }
    }
}
