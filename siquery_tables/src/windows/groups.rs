use winapi::um::lmaccess::NetLocalGroupEnum;
use std::ptr;
use winapi::shared::lmcons::MAX_PREFERRED_LENGTH;
use winapi::shared::winerror::ERROR_MORE_DATA;

pub fn get_specific () {
    let out = Vec::new();
    let group_info_level = 0u32;
    let entries_read = 0u32;
    let total_entries = 0u32;
    let buf_ptr : *mut LOCALGROUP_INFO_1 = ptr::null();
    let ret = NetLocalGroupEnum (
        ptr::null(),
        group_info_level,
        buf_ptr as *mut LPBYTE,
        MAX_PREFERRED_LENGTH,
        &entries_read,  //*mut DWORD
        &total_entries, //*mut DWORD
        ptr::null()
    );
    if buf_ptr == ptr::null() || (ret != 0 && ret != ERROR_MORE_DATA) {
        println!("NetLocalGroupEnum() failed, the return value is : {}", ret);
        Vec::new()
    }
    for i in 0..entries_read {
        // get_sid_from_username()
    }
}