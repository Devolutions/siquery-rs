#![allow(unused_assignments)]

extern crate winapi;

use utils;
use std::{
    os::raw::c_void,
    mem::size_of,
    ptr,
    process::Command,
    borrow::Borrow,
    i64::MAX
};
use winapi::{
    um::{
        tlhelp32::{
            CreateToolhelp32Snapshot,
            TH32CS_SNAPPROCESS,
            Process32First,
            Process32Next,
            PROCESSENTRY32,
        },
        winnt::{
            PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ,
            TOKEN_OWNER,
            TOKEN_READ,
            TokenUser,
            TokenElevation,
            PSID,
            TOKEN_USER,
            TOKEN_ELEVATION,
            PSID_NAME_USE,
            SID_NAME_USE,
            SidTypeUnknown
        },
        handleapi::CloseHandle,
        errhandlingapi::GetLastError,
        processthreadsapi::{
            GetCurrentProcessId,
            GetCurrentProcess,
            OpenProcess,
            GetProcessTimes,
            OpenProcessToken
        },
        psapi::GetModuleFileNameExW,
        libloaderapi::GetModuleFileNameW,
        securitybaseapi::GetTokenInformation,
        lmaccess::{
            NetUserGetInfo,
            USER_INFO_3
        }
    },
    shared::{
        minwindef::{
            MAX_PATH,
            HINSTANCE__,
            FILETIME,
            DWORD,
            FALSE,
            LPDWORD,
            BOOL
        },
        ntdef::{
            ULARGE_INTEGER_s,
            HANDLE,
            LPCSTR,
            LPWSTR,
            NULL
        },
        winerror::{
            ERROR_ACCESS_DENIED,
            ERROR_INSUFFICIENT_BUFFER
        }
    },
    ctypes::c_char

};

use tables::{
    ProcessesRow,
    ProcessesIface
};

extern "C" {
    pub fn LookupAccountSidW (
        lpSystemName: LPCSTR,
        Sid: PSID,
        Name: LPWSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE
    ) -> BOOL;
}

static NERR_UserNotFound: DWORD = 2221;
static NERR_Success: DWORD = 0;

pub fn lookup_account_sid_internal (
    lp_system_name: LPCSTR,
    sid: PSID,
    name: LPWSTR,
    cch_name: LPDWORD,
    referenced_domain_name: LPWSTR,
    cch_referenced_domain_name: LPDWORD,
    pe_use: PSID_NAME_USE
) -> BOOL {
    unsafe {
        LookupAccountSidW (
            lp_system_name,
            sid,
            name,
            cch_name,
            referenced_domain_name,
            cch_referenced_domain_name,
            pe_use
        )
    }
}

pub struct Reader {}
impl ProcessesIface for Reader {
    fn get_wmi_process_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["path", "Win32_Process", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl ProcessesRow {
    pub fn new () -> ProcessesRow{
        ProcessesRow {
            pid: 0,
            name: "".to_owned(),
            path: "".to_owned(),
            cmdline: "".to_owned(), //TODO parse to display correctly
            state: "".to_owned(),   // NA for windows
            cwd: "".to_owned(), //TODO
            root: "".to_owned(),    //TODO
            uid: -1, //TODO
            gid: -1, //TODO
            euid:  -1,  // NA for windows
            egid: -1,   // NA for windows
            suid: -1,   // NA for windows
            sgid: -1,   // NA for windows
            on_disk: 0, //TODO
            wired_size: 0,
            resident_size: 0,
            total_size: 0,
            user_time: 0,   //TODO
            system_time: 0, //TODO
            disk_bytes_read: 0, // NA for windows
            disk_bytes_written: 0,  //NA for windows
            start_time: 0,  //TODO double check result
            parent: 0,
            pgroup: -1, // NA for windows
            threads: 0,
            nice: 0,
            is_elevated_token: 0,   //TODO
            cgroup_namespace: "".to_owned(), // NA for windows
            ipc_namespace: "".to_owned(),   // NA for windows
            mnt_namespace: "".to_owned(),   // NA for windows
            net_namespace: "".to_owned(),   // NA for windows
            pid_namespace: "".to_owned(),   // NA for windows
            user_namespace: "".to_owned(),  // NA for windows
            uts_namespace: "".to_owned(),   // NA for windows

        }
    }
    pub fn get_proc_list () -> Vec<u32> {
        //TODO create a set to store pids
        let mut pids : Vec<u32> = Vec::new();
        let proc_snap = unsafe {CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)};
        let proc_entry : *mut PROCESSENTRY32 = &mut PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; MAX_PATH],
        };
        let mut ret = unsafe {Process32First(proc_snap, proc_entry)};
        if ret == FALSE {
            unsafe {CloseHandle(proc_snap)};
        }
        while ret != FALSE {
            pids.push(unsafe{(*proc_entry).th32ProcessID});
            ret = unsafe {Process32Next(proc_snap, proc_entry)};
        }
        unsafe{CloseHandle(proc_snap)};
        pids
    }

    pub fn get_uid_from_sid (sid: PSID) -> i64 {
        if let Ok(sid_string) = utils::sid_to_string(sid) {
            let components : Vec<_> = sid_string.as_str().split('-').collect();
            if components.len() < 1 {
                return MAX
            }
            let uid = components[components.len()-1].parse::<i64>().unwrap_or(MAX);
            return uid
        } else {
            return MAX
        }
    }

    //TODO getGidFromSid()
    pub fn get_gid_from_sid (sid: PSID) -> i64 {
        let mut gid = -1;

        // Parameters.
        let mut e_use: SID_NAME_USE = SidTypeUnknown as SID_NAME_USE;
        let e_use_p: PSID_NAME_USE = &mut e_use as PSID_NAME_USE;
        let mut uname_size: DWORD = 0u32;
        let uname_size_p: LPDWORD = &mut uname_size as LPDWORD;
        let mut dom_name_size: DWORD = 0u32;
        let dom_name_size_p: LPDWORD = &mut dom_name_size as LPDWORD;

        // Get the buffers sizes.
        lookup_account_sid_internal(
            ptr::null(),
            sid,
            ptr::null_mut(),
            uname_size_p,
            ptr::null_mut(),
            dom_name_size_p,
            e_use_p
        );

        // Buffers.
        let mut uname: Vec<u16> = Vec::with_capacity(uname_size as usize);
        let uname_p: LPWSTR = uname.as_mut_ptr() as LPWSTR;
        let mut dom_name: Vec<u16> = Vec::with_capacity(dom_name_size as usize);
        let dom_name_p: LPWSTR = dom_name.as_mut_ptr() as LPWSTR;

        if lookup_account_sid_internal(
            ptr::null(),
            sid,
            uname_p,
            uname_size_p,
            dom_name_p,
            dom_name_size_p,
            e_use_p
        ) == 0 {
            return -1
        };

        let mut user_buf: Vec<u8> = Vec::with_capacity(size_of::<USER_INFO_3>());
        let user_buf_p: *mut *mut u8 = &mut user_buf.as_mut_ptr();
        let ret = unsafe {NetUserGetInfo(ptr::null(), uname_p, 3, user_buf_p)};
        if ret == NERR_UserNotFound {
            if let Ok(sid_string) = utils::sid_to_string(sid) {
                let components : Vec<_> = sid_string.as_str().split('-').collect();
                // TODO What is the default return value? 10?
                gid = components[components.len()-1].parse::<i64>().unwrap_or(10);
            }
        } else if ret == NERR_Success {
            let user_info_3_p = user_buf_p as *mut _ as *mut USER_INFO_3;
            gid = unsafe{*user_info_3_p}.usri3_primary_group_id as i64;
        }
        gid
    }

    pub(crate) fn get_specific_ex (reader: &ProcessesIface) -> Vec<ProcessesRow> {
        let mut out: Vec<ProcessesRow> = Vec::new();
        let current_pid = unsafe{GetCurrentProcessId()} as i64;

        if let Some(process_info) = reader.get_wmi_process_info() {
            let mut processes_row = ProcessesRow::new();
            for line in process_info.lines() {
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);
                #[allow(unreachable_patterns)]
                    match k.as_str() {
                    "CommandLine" => {
                        processes_row = ProcessesRow::new();
                        processes_row.cmdline = v;
                    },
                    "Name" => {
                        processes_row.name = v;
                    },
                    "ExecutablePath" => {
                        processes_row.path = v;
                    },
                    "ExecutionState" => {
                        processes_row.state = v;
                    },
                    "ParentProcessId" => {
                        processes_row.parent = v.parse::<i64>().unwrap_or(-1);
                    },
                    "ProcessId" => {

                        let pid = v.to_owned().parse::<i64>().unwrap_or(-1);
                        let null_pointer = ptr::null::<c_void>() as *mut c_void;
                        //TODO replace null_pointer by ptr::null_mut()!
                        processes_row.pid = pid;
                        let mut h_process: *mut winapi::ctypes::c_void = 0 as *mut c_void;
                        if pid == current_pid {
                            h_process = unsafe {GetCurrentProcess()};
                        } else {
                            h_process = unsafe {OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid as u32)};
                        }
                        if unsafe {GetLastError()} == ERROR_ACCESS_DENIED {
                            processes_row.uid = 0;
                            processes_row.gid = 0;
                        }
                        let mut file_name: Vec<u16> = Vec::with_capacity(MAX_PATH as usize);
                        if pid == current_pid {
                            unsafe{
                                GetModuleFileNameW(null_pointer.clone() as *mut HINSTANCE__, file_name.as_mut_ptr(), MAX_PATH as u32);
                            };
                        } else {
                            unsafe{
                                GetModuleFileNameExW(h_process, null_pointer.clone() as *mut HINSTANCE__, file_name.as_mut_ptr(), MAX_PATH as u32);
                            };
                        }
                        processes_row.cwd = String::from_utf16(&file_name).unwrap_or("could not parse cwd".to_string());
                        let mut create_time: *mut FILETIME = &mut FILETIME {
                            dwLowDateTime: 0,
                            dwHighDateTime: 0,
                        };
                        let mut exit_time: *mut FILETIME = &mut FILETIME {
                            dwLowDateTime: 0,
                            dwHighDateTime: 0,
                        };
                        let mut kernel_time: *mut FILETIME = &mut FILETIME {
                            dwLowDateTime: 0,
                            dwHighDateTime: 0,
                        };
                        let mut user_time: *mut FILETIME = &mut FILETIME {
                            dwLowDateTime: 0,
                            dwHighDateTime: 0,
                        };
                        let proc_ret = unsafe{ GetProcessTimes(h_process, create_time, exit_time, kernel_time, user_time) };
                        if proc_ret == FALSE {
                            processes_row.user_time = -1;
                            processes_row.system_time = -1;
                            processes_row.start_time = -1;
                        } else {
                            let mut utime = ULARGE_INTEGER_s {
                                LowPart: 0,
                                HighPart: 0,
                            };
                            utime.HighPart = unsafe {(*user_time)}.dwHighDateTime;
                            utime.LowPart = unsafe {(*user_time)}.dwLowDateTime;
                            // Windows stores proc times in 100 nanosecond ticks
                            utime.HighPart = unsafe{(*kernel_time)}.dwHighDateTime;
                            utime.LowPart = unsafe{(*kernel_time)}.dwLowDateTime;
                            processes_row.system_time = 0;//TODO QuadPart of utime / 10000000
                            processes_row.start_time = 0;//TODO filetime to unix time
                        }

                        let mut tok: *mut HANDLE = &mut null_pointer.clone();
                        let mut tok_owner: Vec<c_char> = Vec::with_capacity(size_of::<TOKEN_OWNER>());
                                //let mut buffer: Vec<u8> = Vec::new();
                        let mut ret = unsafe {OpenProcessToken(h_process, TOKEN_READ, tok)};
                        if ret != 0 && tok != &mut null_pointer.clone() {
                            let mut tok_owner_buff_len : u32 = 0;
                            ret = unsafe {GetTokenInformation(*tok, TokenUser, null_pointer.clone(), 0, &mut tok_owner_buff_len)};
                            if ret == 0 && unsafe{GetLastError()} == ERROR_INSUFFICIENT_BUFFER {
                                tok_owner = Vec::with_capacity(tok_owner_buff_len as usize);
                                ret = unsafe{GetTokenInformation(*tok, TokenUser,tok_owner.as_mut_ptr() as *mut c_void , tok_owner_buff_len, &mut tok_owner_buff_len)};
                                unsafe {tok_owner.set_len(tok_owner_buff_len as usize)};
                                //println!("-<{:?}",unsafe {CStr::from_ptr(tok_owner.as_mut_ptr())});

                            }
                            // Check if the process is using an elevated token
                            let mut elevation = TOKEN_ELEVATION {
                                TokenIsElevated: 0
                            };

                            let mut cb_size: DWORD = size_of::<TOKEN_ELEVATION>() as u32;
                            if unsafe {GetTokenInformation(*tok, TokenElevation, &mut elevation as *mut _ as *mut c_void, size_of::<TOKEN_ELEVATION>() as u32, &mut cb_size)} != 0 {
                                processes_row.is_elevated_token = elevation.TokenIsElevated as i32;
                            }
                        }

                        /*let mut tok_owner_struct: TOKEN_OWNER = TOKEN_OWNER {
                            Owner: 0 as *mut c_void,
                        };*/
                        //println!("{}-{}-{:?}",processes_row.uid,ret,tok_owner);

                        if processes_row.uid != 0 && ret != 0 && tok_owner.len() != 0  /*.Owner != ptr::null_mut()*/ {
                                //println!("got in first if statement");
                                let sid_ptr = tok_owner.as_ptr() as *mut TOKEN_USER/*.Owner*/;
                            let sid = unsafe{*sid_ptr}.User.Sid;
                            //println!("sid: {:?}",sid);
                                processes_row.uid = ProcessesRow::get_uid_from_sid(sid);
                                processes_row.gid = ProcessesRow::get_gid_from_sid(sid);
                            }/*else {
                                processes_row.uid = uid;
                                processes_row.gid = gid;
                            }*/ //unnecessary since it is already initialized as -1 and then 0 if it doesnt have acces rights.
                        if h_process != null_pointer.clone() {
                                unsafe {CloseHandle(h_process)};
                            }
                        if tok != &mut null_pointer.clone() {
                                unsafe {CloseHandle(*tok)};
                                tok = &mut null_pointer.clone();
                            }
                    },
                    "ExecutionState" => {
                        processes_row.state = v;
                    },
                    "ParentProcessId" => {
                        processes_row.parent = v.parse::<i64>().unwrap_or(-1);
                    },
                    "ThreadCount" => {
                        processes_row.threads = v.parse::<i32>().unwrap_or(-1);
                    },
                    "Priority" => {
                        processes_row.nice = v.parse::<i32>().unwrap_or(-1);
                    },
                    "PrivatePageCount" => {
                        processes_row.wired_size = v.parse::<i64>().unwrap_or(-1);
                    },
                    "WorkingSetSize" => {
                        processes_row.resident_size = v.parse::<i64>().unwrap_or(-1);
                    },
                    "VirtualSize" => {
                        processes_row.total_size = v.parse::<i64>().unwrap_or(-1);
                    },
                    "ExecutionState" => {
                        processes_row.state = v;
                    },
                    "WriteTransferCount" => {
                        if processes_row.pid != 0 {
                            out.push(processes_row);
                        };
                        processes_row = ProcessesRow::new();
                    },
                    _ => ()
                }
            }
        }
        out
    }

    pub(crate) fn get_specific () -> Vec<ProcessesRow> {
        let reader: Box<ProcessesIface> = Box::new(Reader{});
        let out = ProcessesRow::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    //TODO
}
