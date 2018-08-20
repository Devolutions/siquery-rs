use libc::getmntent;    // reads the next line from the file system description file fp and
// returns a pointer to a structure containing the broken out fields from a line in the file. [1]
use libc::setmntent;    // Opens the file system description file filename and returns a file
// pointer which can be used by getmntent(). [1]
// [1] https://linux.die.net/man/3/setmntent
use libc::mntent;       // Result being stored here (struct).
use libc::endmntent;       // Closes the file system description file fp.
use libc::libc::statfs;
use tables::MountsRow;

impl MountsRow {
    fn new() -> MountsRow {
        MountsRow {
            device: String::new(),
            device_alias: String::new(),
            path: String::new(),
            device_type: String::new(),
            blocks_size: 0,
            blocks: 0,
            blocks_free: 0,
            blocks_available: 0,
            inodes: 0,
            inodes_free: 0,
            flags: String::new(0),
        }
    }
    pub fn gen_mounts () -> Vec<MountsRow> {
        let mut out = Vec::new();
        let mounts: *mut FILE  = setmntent("/proc/mounts", "r");
        if mounts == ptr::null() {
            return {};
        }
        let mut ent: *mut mntent = ptr::null();
        while ent = getmntent(mounts) {
            let mut row = MountsRow::new();
            row.device = std::string(ent.mnt_fsname);
            row.device_alias = canonicalize_file_name(ent.mnt_fsname);
            row.path = std::string(ent.mnt_dir);
            row.device_type = std::string(ent.mnt_type);
            row.flags = std::string(ent.mnt_opts);

            let mut st: statfs = mem::zeroed();
            if unsafe{!statfs(ent.mnt_dir, &st)} {
                row.blocks_size = st.f_bsize;
                row.blocks = st.f_blocks;
                row.blocks_free = st.f_bfree;
                row.blocks_available = st.f_bavail;
                row.inodes = st.f_files;
                row.inodes_free = st.f_ffree;
            }
        out.push(row)
        }
        endmntent(mounts);
        out
    }
}
