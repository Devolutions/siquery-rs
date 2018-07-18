
use winapi::um::tlhelp32::{MODULEENTRY32W,CreateToolhelp32Snapshot,Module32FirstW,Module32NextW};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::VirtualQueryEx;
use std::{ptr,mem};
use tables::{ProcessMemoryMapRow};
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::shared::minwindef::FALSE;
use winapi::um::tlhelp32::{TH32CS_SNAPMODULE,TH32CS_SNAPMODULE32};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::handleapi::CloseHandle;
use winapi::_core::mem::size_of;
use winapi::shared::minwindef::HINSTANCE__;
use winapi::um::winnt::MEMORY_BASIC_INFORMATION;
use winapi::ctypes::c_void;
use winapi::um::winbase::FormatMessageW;
use winapi::um::winbase::FORMAT_MESSAGE_FROM_SYSTEM;
use winapi::um::errhandlingapi::GetLastError;

use tables::ProcessesRow;

impl ProcessMemoryMapRow {
    pub fn new() -> ProcessMemoryMapRow {
        ProcessMemoryMapRow {
            pid: 0,
            start: "".to_owned(),
            end: "".to_owned(),
            permissions: "".to_owned(),
            offset: 0,
            device: "".to_owned(),
            inode: 0,
            path: "".to_owned(),
            pseudo: 0,
        }
    }
    pub fn get_last_error_text () -> String {
        unsafe {
            let mut message = [0u16; 512];
            let length = FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM, ptr::null(), GetLastError(), 0, message.as_mut_ptr(), message.len() as u32, ptr::null_mut());
            String::from_utf16(&message[0..length as usize]).unwrap_or("".to_string())
        }
    }
    pub fn gen_memory_map_table_internal (pid: u32) -> Option<Vec<ProcessMemoryMapRow>> {   //TODO option!
        let mut out: Vec<ProcessMemoryMapRow> = Vec::new();
        let mut memory_map_row = ProcessMemoryMapRow::new();
        let process_handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid) };
        if process_handle == ((ptr::null::<c_void>()) as *mut c_void) {
            // Failed to openhandle to process
            memory_map_row.pid = pid as i32;
            memory_map_row.start = "-1".to_owned();
            memory_map_row.end = "-1".to_owned();
            memory_map_row.offset = -1;
            memory_map_row.device = "-1".to_owned();
            memory_map_row.inode = -1;
            memory_map_row.path = "".to_owned();
            memory_map_row.pseudo = -1;
        }

        let mod_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };

        if mod_snap == INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(process_handle) };
        }

        //TODO permissions enum

        let me: *mut MODULEENTRY32W =
            &mut MODULEENTRY32W {
                dwSize: size_of::<MODULEENTRY32W>() as u32,
                th32ModuleID: 0 as u32,
                th32ProcessID: 0 as u32,
                GlblcntUsage: 0 as u32,
                ProccntUsage: 0 as u32,
                modBaseAddr: 0 as *mut u8,
                modBaseSize: 0 as u32,
                hModule: 0 as *mut HINSTANCE__,
                szModule: [0 as u16; 256],
                szExePath: [0 as u16; 260],
            };

        let m_info: *mut MEMORY_BASIC_INFORMATION =
            &mut MEMORY_BASIC_INFORMATION {
                BaseAddress: ptr::null_mut(),
                AllocationBase: ptr::null_mut(),
                AllocationProtect: 0,
                RegionSize: 0 as usize,
                State: 0,
                Protect: 0,
                Type: 0,
            };

        let mut ret = unsafe { Module32FirstW(mod_snap, me) };

        while ret != FALSE {
            let mut p = unsafe{(*me).modBaseAddr};
            let mut size = unsafe { VirtualQueryEx(process_handle, p as *const c_void, m_info, mem::size_of::<MEMORY_BASIC_INFORMATION>()) };
            while size == mem::size_of::<MEMORY_BASIC_INFORMATION>() && p < unsafe{(*me).modBaseAddr.offset((*me).modBaseSize as isize)} {
                out.push(
                    ProcessMemoryMapRow {
                        pid: pid as i32,
                        start: format!("{:?}", unsafe {(*m_info).BaseAddress}),
                        end: format!("{:?}", unsafe {(*m_info).BaseAddress.offset((*m_info).RegionSize as isize)}),
                        permissions: "".to_owned(),  //TODO
                        offset: format!("{:?}", unsafe {(*m_info).AllocationBase}).parse::<i64>().unwrap_or(-1),
                        device: "-1".to_owned(),
                        inode: -1,
                        path: format! ("{}", String::from_utf16(&unsafe{(*me).szExePath}).unwrap_or("".to_string()).trim_matches('\0')),
                        pseudo: -1,
                    }
                );
                unsafe{p = p.offset((*m_info).RegionSize as isize)};
                ret = unsafe {Module32NextW(mod_snap, me)};
                size = unsafe { VirtualQueryEx(process_handle, p as *const c_void, m_info, mem::size_of::<MEMORY_BASIC_INFORMATION>())};
            }
        }
        unsafe {
            CloseHandle(process_handle);
            CloseHandle(mod_snap);
        }
        Some(out)
    }
    pub fn get_specific () -> Vec<ProcessMemoryMapRow>{
        let pid_list = ProcessesRow::get_proc_list();
        let mut table: Vec<ProcessMemoryMapRow> = Vec::new();
        for pid in pid_list.iter() {
            if *pid != 0 {
                table.append(&mut ProcessMemoryMapRow::gen_memory_map_table_internal (*pid).unwrap_or_else(|| Vec::new()));
            }
        }
        table
    }
}
