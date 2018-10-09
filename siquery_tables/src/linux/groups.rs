use libc::{
    getgrent,
    group
};

pub fn get_specific() -> Vec<GroupsRow> {
    let mut out = Vec::new();
    unsafe {setgrent()};
    let group_p: *mut group = unsafe {getgrent()};
    if group_p != ptr::null {
        // why use a set?
        out.push(
            GroupsRow {
                gid: unsafe {*group_p}.gr_gid,
                gid_signed: unsafe {*group_p}.gr_gid as int32_t,
                groupname: unsafe {*group_p}.gr_name,
                group_sid: "".to_string(),
                comment: "".to_string()
            }
        );
    }
    unsafe {endgrent()};
    out
}
