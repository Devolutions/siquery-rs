extern crate winapi;

use tables::ProcessesRow;
use windows::SystemReaderInterface;

use utils;
use std::os::raw::c_void;
use winapi::um::tlhelp32::*;
use winapi::shared::minwindef::*;
use std::mem::size_of;
use winapi::um::handleapi::*;
use winapi::um::winnt::*;
use std::ptr;
use winapi::shared::ntdef::*;
use winapi::um::errhandlingapi::*;
use winapi::um::processthreadsapi::*;
use winapi::shared::winerror::*;
use winapi::um::psapi::*;
use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::HANDLE;
use winapi::um::libloaderapi::*;
use winapi::um::securitybaseapi::*;

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
            uid: 0, //TODO
            gid: 0, //TODO
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

    //TODO getUidFromSid()
    //TODO getGidFromSid()

    pub(crate) fn gen_processes_table (system_reader: &SystemReaderInterface) -> Vec<ProcessesRow> {
        let mut out: Vec<ProcessesRow> = Vec::new();
        let current_pid = unsafe{GetCurrentProcessId()} as i64;

        if let Some(process_info) = system_reader.get_wmi_process_info() {
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
                        processes_row.pid = pid;
                        #[allow(unused_assignments)]
                        let mut h_process: *mut winapi::ctypes::c_void = 0 as *mut c_void;
                        let gid : i64 = -1;
                        let uid : i64 = -1;
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
                        let mut tok_owner: TOKEN_OWNER = TOKEN_OWNER {
                            Owner: 0 as *mut c_void,
                        };
                        let mut buffer: Vec<u8> = Vec::new();
                        let mut ret = unsafe {OpenProcessToken(h_process, TOKEN_READ, tok)};
                        if ret != 0 && tok != &mut null_pointer.clone() {
                            let mut tok_owner_buff_len : u32 = 0;
                            ret = unsafe {GetTokenInformation(*tok, TokenUser, null_pointer.clone(), 0, &mut tok_owner_buff_len)};
                            if ret == 0 && unsafe{GetLastError()} == ERROR_INSUFFICIENT_BUFFER {
                                buffer = Vec::with_capacity(tok_owner_buff_len as usize);
                                ret = unsafe{GetTokenInformation(*tok, TokenUser,buffer.as_mut_ptr() as *mut c_void , tok_owner_buff_len, &mut tok_owner_buff_len)};
                            }
                            // Check if the process is using an elevated token
                            let elevation = TOKEN_ELEVATION{
                                TokenIsElevated: 0
                            };

                            let mut cb_size: DWORD = size_of::<TOKEN_ELEVATION>() as u32;
                            if unsafe {GetTokenInformation(*tok, TokenElevation, buffer.as_mut_ptr() as *mut c_void, size_of::<TOKEN_ELEVATION>() as u32, &mut cb_size)} != 0 {
                                processes_row.is_elevated_token = elevation.TokenIsElevated as i32;
                            }
                            if processes_row.uid != 0 && ret != 0 && tok_owner.Owner != 0 as *mut c_void {
                                //TODO let sid = &mut tok_owner;
                                processes_row.uid = 0;//TODO getUidFromSid(sid);
                                processes_row.gid = 0;//TODO INTEGER(getGidFromSid(sid));
                            } else {
                                processes_row.uid = uid;
                                processes_row.gid = gid;
                            }
                            if h_process != null_pointer.clone() {
                                unsafe {CloseHandle(h_process)};
                            }
                            if tok != &mut null_pointer.clone() {
                                unsafe {CloseHandle(*tok)};
                                tok = &mut null_pointer.clone();
                            }
                        };
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
}
