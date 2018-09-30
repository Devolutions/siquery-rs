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

/// Converts a raw SID into a SID string representation.
pub fn sid_to_string(sid: PSID) -> Result<String, DWORD> {
    let mut buf: LPWSTR = NULL as LPWSTR;
    if unsafe { ConvertSidToStringSidW(sid, &mut buf) } == 0 ||
        buf == (NULL as LPWSTR) {
        return Err(unsafe { GetLastError() });
    }

    let buf_size = unsafe { libc::wcslen(buf) };
    let sid_string = unsafe { WideString::from_ptr(buf, buf_size) };

    unsafe { LocalFree(buf as HLOCAL) };

    Ok(sid_string.to_string_lossy())
}



