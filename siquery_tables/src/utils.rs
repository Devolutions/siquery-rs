use winapi::{
    um::{
        winnt::PSID,
        errhandlingapi::GetLastError,
        winbase::LocalFree
    },
    shared::{
        minwindef::{
            DWORD,
            HLOCAL
        },
        ntdef::{
            LPWSTR,
            NULL
        },
        sddl::ConvertSidToStringSidW
    }
};
use widestring::WideString;
use libc;

/// Remove trailing '\n' at the end of a string.
pub fn trim_string(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') || s.ends_with(',') {
        let new_len = s.len() - 1;
        s.truncate(new_len);
    }
}

pub fn sid_to_string(sid: PSID) -> Result<String, DWORD> {
    let mut buf: LPWSTR = NULL as LPWSTR;
    if unsafe { ConvertSidToStringSidW(sid, &mut buf) } == 0 ||
        buf == (NULL as LPWSTR) {
        return Err(unsafe { GetLastError() });
    }
    lpwstr_to_string(buf)
}

pub fn lpwstr_to_string(lpwstr: LPWSTR) -> Result<String, DWORD> {
    let buf_size = unsafe { libc::wcslen(lpwstr) };
    let string = unsafe { WideString::from_ptr(lpwstr, buf_size) };
    unsafe { LocalFree(lpwstr as HLOCAL) };
    Ok(string.to_string_lossy())
}



