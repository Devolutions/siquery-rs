#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

use libc::*;
use libc::timeval;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    mem,
    ptr,
    path::Path,
    ffi::{
        CStr,
        OsStr
    },
    str,
    collections::HashMap
};

use crate::tables::ProcessesRow;

pub struct ProcCred {
    parent: u32,
    group: u32,
    status: u32,
    nice: i32,
    real_uid: uid_t,
    real_gid: gid_t,
    effective_uid: uid_t,
    effective_gid: gid_t,
    saved_uid: uid_t,
    saved_gid: gid_t,
}

pub struct ProcArgs {
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

struct RootAndCwdInternal {
    root: String,
    cwd: String,
}

struct NamePathAndOnDiskInternal {
    name: String,
    path: String,
    on_disk: i32,
}

struct RessourceUsageInternal {
    wired_size: i64,
    resident_size: i64,
    total_size: i64,
    user_time: i64,
    system_time: i64,
    start_time: i64,
    disk_bytes_read: i64,
    disk_bytes_written: i64,
}

#[link(name = "proc", kind = "dylib")]
extern "C" {
    fn proc_pidinfo(
        pid: c_int,
        flavor: c_int,
        arg: u64,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
    fn proc_listpids(
        proc_type: u32,
        typeinfo: u32,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
    fn proc_pidpath(
        pid: c_int,
        buffer: *mut c_void,
        buffersize: u32,
    ) -> c_int;
    fn proc_name(
        pid: c_int,
        buffer: *mut c_void,
        buffersize: u32,
    ) -> c_int;
    fn proc_pid_rusage(
        pid: i32,
        flavor: i32,
        buffer: *mut c_void,
    ) -> c_int;
}

#[repr(C)]
struct proc_bsdshortinfo {
    pbsi_pid: u32, // process id
    pbsi_ppid: u32, // process parent id
    pbsi_pgid: u32, // process perp id
    pbsi_status: u32, // p_stat value, SZOMB, SRUN, etc
    pbsi_comm: [c_char; 16], // MAXCOMLEN - upto 16 characters of process name
    pbsi_flags: u32, // 64bit; emulated etc
    pbsi_uid: uid_t, // current uid on process
    pbsi_gid: gid_t, // current gid on process
    pbsi_ruid: uid_t, // current ruid on process
    pbsi_rgid: gid_t, // current tgid on process
    pbsi_svuid: uid_t, // current svuid on process
    pbsi_svgid: gid_t, // current svgid on process
    pbsi_rfu: u32, // reserved for future use
}

#[repr(C)]
struct proc_vnodepathinfo {
    pvi_cdir: vnode_info_path,
    pvi_rdir: vnode_info_path,
}

#[repr(C)]
struct vnode_info_path {
    vip_vi: vnode_info,
    vip_path: [c_char; 1024], // MAXPATHLEN = 1024
}

#[repr(C)]
struct vnode_info {
    vi_stat: vinfo_stat,
    vi_type: i32,
    vi_pad: i32,
    vi_fsid: fsid_t,
}

#[repr(C)]
struct vinfo_stat {
    vst_dev: u32,
    vst_mode: u16,
    vst_nlink: u16,
    vst_ino: u64,
    vst_uid: uid_t,
    vst_gid: gid_t,
    vst_atime: i64,
    vst_atimensec: i64,
    vst_mtime: i64,
    vst_mtimensec: i64,
    vst_ctime: i64,
    vst_ctimensec: i64,
    vst_birthtime: i64,
    vst_birthtimensec: i64,
    vst_size: off_t,
    vst_blocks: i64,
    vst_blksize: i32,
    vst_flags: u32,
    vst_gen: u32,
    vst_rdev: u32,
    vst_qspare: [i64; 2],
}

#[repr(C)]
struct proc_taskinfo {
    pti_virtual_size: u64, // virtual memory size (bytes)
    pti_resident_size: u64, // resident memory size (bytes)
    pti_total_user: u64, // total time
    pti_total_system: u64,
    pti_threads_user: u64, // existing threads only
    pti_threads_system: u64,
    pti_policy: i32, // default policy for new threads
    pti_faults: i32, // number of page faults
    pti_pageins: i32, // number of actual pageins
    pti_cow_faults: i32, // number of copy-on-write faults
    pti_messages_sent: i32, // number of messages sent
    pti_messages_received: i32, // number of messages received
    pti_syscalls_mach: i32, // number of mach system calls
    pti_syscalls_unix: i32, // number of unix system calls
    pti_csw: i32, // number of context switches
    pti_threadnum: i32, // number of threads in the task
    pti_numrunning: i32, // number of running threads
    pti_priority: i32,  // task priority
}

#[repr(C)]
struct rusage_info_v2 {
    ri_uuid: [u8; 16],
    ri_user_time: u64,
    ri_system_time: u64,
    ri_pkg_idle_wkups: u64,
    ri_interrupt_wkups: u64,
    ri_pageins: u64,
    ri_wired_size: u64,
    ri_resident_size: u64,
    ri_phys_footprint: u64,
    ri_proc_start_abstime: u64,
    ri_proc_exit_abstime: u64,
    ri_child_user_time: u64,
    ri_child_system_time: u64,
    ri_child_pkg_idle_wkups: u64,
    ri_child_interrupt_wkups: u64,
    ri_child_pageins: u64,
    ri_child_elapsed_abstime: u64,
    ri_diskio_bytesread: u64,
    ri_diskio_byteswritten: u64,
}

type mach_timebase_info_data_t = mach_timebase_info;

impl ProcessesRow {
    fn new() -> ProcessesRow {
        ProcessesRow {
            pid: 0,
            name: "".to_owned(),
            path: "".to_owned(),
            cmdline: "".to_owned(),
            state: "".to_owned(),
            cwd: "".to_owned(),
            root: "".to_owned(),
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            suid: 0,
            sgid: 0,
            on_disk: 0,
            wired_size: 0,
            resident_size: 0,
            total_size: 0,
            user_time: 0,
            system_time: 0,
            disk_bytes_read: 0,
            disk_bytes_written: 0,
            start_time: 0,
            parent: 0,
            pgroup: -1,
            threads: 0,
            nice: 0,
            is_elevated_token: 0,  // NA for mac
            cgroup_namespace: "".to_owned(),
            ipc_namespace: "".to_owned(),
            mnt_namespace: "".to_owned(),
            net_namespace: "".to_owned(),
            pid_namespace: "".to_owned(),
            user_namespace: "".to_owned(),
            uts_namespace: "".to_owned(),
        }
    }

    pub fn get_proc_list() -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        let PROC_ALL_PIDS = 1;

        let mut bufsize = unsafe {
            proc_listpids(
                PROC_ALL_PIDS,
                0,
                ptr::null_mut(),
                0,
            )
        };
        if bufsize <= 0 {
            println!("An error occurred while retrieving the process list");
            return out;
        }
        // Use twice the number of PIDs returned to handle races.
        let mut pid_list: Vec<pid_t> = Vec::with_capacity(2 * bufsize as usize);
        bufsize = unsafe {
            proc_listpids(
                PROC_ALL_PIDS,
                0,
                pid_list.as_mut_ptr() as *mut c_void,
                2 * bufsize,
            )
        };
        if bufsize <= 0 {
            println!("An error occurred while retrieving the process list");
            return pid_list;
        }
        let items_count = bufsize as usize / mem::size_of::<i32>();
        unsafe { pid_list.set_len(items_count) }
        for pid in pid_list {
            // Dismiss negative PIDs as they do not represent real processes.
            if pid >= 0 {
                out.push(pid);
            }
        }
        out
    }

    fn new_proc_cred() -> ProcCred {
        let proc_cred: ProcCred = unsafe { mem::zeroed() };
        proc_cred
    }

    fn get_proc_path(pid: i32) -> String {
        let mut pathbuf: Vec<u8> = Vec::with_capacity(4 * 1024); // 4 * MAXPATHLEN
        let mut ret = 0 as i32;
        let mut out = String::new();

        unsafe {
            ret = proc_pidpath(
                pid,
                pathbuf.as_mut_ptr() as *mut c_void,
                pathbuf.capacity() as u32,
            );
        };

        if ret > 0 {
            unsafe {
                pathbuf.set_len(ret as usize);
            }
            out = String::from_utf8(pathbuf).unwrap_or("An error occurred while retrieving process path".to_string())
        }
        out
    }

    pub fn gen_proc_cred(pid: i32) -> ProcCred {
        let mut cred: ProcCred = ProcessesRow::new_proc_cred();
        let mut bsdinfo: proc_bsdinfo = unsafe { mem::zeroed() };
        let p_bsdinfo: *mut c_void = &mut bsdinfo as *mut _ as *mut c_void;
        let mut bsdinfo_short: proc_bsdshortinfo = unsafe { mem::zeroed() };
        let p_bsdinfo_short: *mut c_void = &mut bsdinfo_short as *mut _ as *mut c_void;
        let proc_pidtbsdinfo_var = 3;
        let PROC_PIDTBSDINFO_SIZE = mem::size_of::<proc_bsdinfo>() as i32;
        let PROC_PIDT_SHORTBSDINFO = 13;
        let PROC_PIDT_SHORTBSDINFO_SIZE = mem::size_of::<proc_bsdshortinfo>() as i32;

        if unsafe {
            proc_pidinfo(
                pid,
                proc_pidtbsdinfo_var,
                1,
                p_bsdinfo,
                PROC_PIDTBSDINFO_SIZE,
            )
        } == PROC_PIDTBSDINFO_SIZE {
            cred.parent = bsdinfo.pbi_ppid;
            cred.group = bsdinfo.pbi_pgid;
            cred.status = bsdinfo.pbi_status;
            cred.nice = bsdinfo.pbi_nice;
            cred.real_uid = bsdinfo.pbi_ruid;
            cred.real_gid = bsdinfo.pbi_rgid;
            cred.effective_uid = bsdinfo.pbi_uid;
            cred.effective_gid = bsdinfo.pbi_gid;
            cred.saved_uid = bsdinfo.pbi_svuid;
            cred.saved_gid = bsdinfo.pbi_svgid;
        } else if unsafe {
            proc_pidinfo(
                pid,
                PROC_PIDT_SHORTBSDINFO,
                1,
                p_bsdinfo_short,
                PROC_PIDT_SHORTBSDINFO_SIZE,
            )
        } == PROC_PIDT_SHORTBSDINFO_SIZE {
            cred.parent = bsdinfo_short.pbsi_ppid;
            cred.group = bsdinfo_short.pbsi_pgid;
            cred.status = bsdinfo_short.pbsi_status;
            cred.real_uid = bsdinfo_short.pbsi_ruid;
            cred.real_gid = bsdinfo_short.pbsi_rgid;
            cred.effective_uid = bsdinfo_short.pbsi_uid;
            cred.effective_gid = bsdinfo_short.pbsi_gid;
            cred.saved_uid = bsdinfo_short.pbsi_svuid;
            cred.saved_gid = bsdinfo_short.pbsi_svgid;
        }
        cred
    }

    pub fn gen_max_args() -> usize {
        let mut arg_max: usize = 0;

        if arg_max == 0 {
            let mut mib: [i32; 2] = [CTL_KERN, KERN_ARGMAX];
            let mut size = mem::size_of::<i32>();
            if unsafe {
                sysctl(
                    &mut mib as *mut _ as *mut i32,
                    2,
                    &mut arg_max as *mut _ as *mut c_void,
                    &mut size as *mut _ as *mut usize,
                    ptr::null::<c_void>() as *mut c_void,
                    0,
                )
            } == -1 {
                return 0;
            }
        }
        arg_max
    }

    fn get_proc_root_and_cwd(pid: i32) -> RootAndCwdInternal {
        let proc_pidvnodepathinfo_var = 9;
        let mut pathinfo: proc_vnodepathinfo = unsafe { mem::zeroed() };
        let mut stringarray: RootAndCwdInternal = unsafe { mem::zeroed() };

        if unsafe {
            proc_pidinfo(
                pid,
                proc_pidvnodepathinfo_var,
                0,
                &mut pathinfo as *mut _ as *mut c_void,
                mem::size_of::<proc_vnodepathinfo>() as i32,
            )
        } == mem::size_of::<proc_vnodepathinfo>() as i32 {
            if pathinfo.pvi_rdir.vip_vi.vi_stat.vst_dev != 0 {
                let v = pathinfo.pvi_rdir.vip_path.iter().cloned().collect::<Vec<_>>();
                let c_str: &CStr = unsafe { CStr::from_ptr(v.as_ptr()) };
                let buf: &[u8] = c_str.to_bytes();
                stringarray.root = str::from_utf8(buf).unwrap_or("An error occurred while retrieving process root").to_string();
            }
            if pathinfo.pvi_cdir.vip_vi.vi_stat.vst_dev != 0 {
                let v = pathinfo.pvi_cdir.vip_path.iter().cloned().collect::<Vec<_>>();
                let c_str: &CStr = unsafe { CStr::from_ptr(v.as_ptr()) };
                let buf: &[u8] = c_str.to_bytes();
                stringarray.cwd = str::from_utf8(buf).unwrap_or("An error occurred while retrieving process cwd").to_string();
            }
        }
        stringarray
    }

    fn gen_proc_name_path_and_on_disk(pid: i32, cred: ProcCred) -> NamePathAndOnDiskInternal {
        let mut name = String::new();
        let mut path = String::new();
        let mut on_disk = -2 as i32;
        let mut out: NamePathAndOnDiskInternal = unsafe { mem::zeroed() };

        if pid == 0 {
            name = "kernel_task".to_owned();
            path = "".to_owned();
        } else if cred.status != 5 {
            // If the process is not a Zombie, try to find the path and name.
            path = ProcessesRow::get_proc_path(pid);
            name = Path::new(&path).file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").to_string();
        } else {
            path = "".to_owned();
            // proc_name truncates the name to 16 bytes.
            let mut name_buf: Vec<u8> = Vec::with_capacity(16);    // MAXCOMLEN (max command name remembered)
            let mut int_t = 0 as usize;
            unsafe {
                int_t = proc_name(pid, name_buf.as_mut_ptr() as *mut c_void, 16) as usize;
                name_buf.set_len(int_t);
            };
            name = String::from_utf8(name_buf).unwrap_or("".to_string());
        }

        // Set on_disk.
        if path.is_empty() {
            on_disk = -1;
        } else if Path::new(&path).exists() {
            on_disk = 1;
        } else {
            on_disk = 0;
        }

        out.name = name;
        out.path = path;
        out.on_disk = on_disk;
        out
    }

    fn gen_proc_num_threads(pid: i32) -> i32 {
        let mut threads = 0 as i32;
        let mut task_info: proc_taskinfo = unsafe { mem::zeroed() };
        let status = unsafe {
            proc_pidinfo(
                pid,
                PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut c_void,
                mem::size_of::<proc_taskinfo>() as i32,
            )
        };
        if status == mem::size_of::<proc_taskinfo>() as i32 {
            threads = task_info.pti_threadnum;
        } else {
            threads = -1;
        }
        threads
    }

    pub fn get_proc_raw_args(pid: i32, mut argmax: size_t) -> ProcArgs {
        let mut args: ProcArgs = ProcArgs {
            args: Vec::new(),
            env: HashMap::new(),
        };

        let mut procargs_buffer: Vec<u8> = Vec::with_capacity(argmax);
        let mut mib: [i32; 3] = [CTL_KERN, KERN_PROCARGS2, pid];
        let ret = unsafe {
            sysctl(
                &mut mib as *mut _ as *mut i32,
                3,
                procargs_buffer.as_mut_ptr() as *mut c_void,
                &mut argmax as *mut _ as *mut usize,
                ptr::null::<c_void>() as *mut c_void,
                0,
            )
        };

        if ret == -1 || argmax == 0 {
            return args;
        } else if ret == 0 {
            // When successful, the system call changes argmax to reflect the amount of data copied.
            unsafe { procargs_buffer.set_len(argmax) };
            // The number of arguments is an integer in front of the result buffer.
            let (mut first_part, second_part) = procargs_buffer.split_at(4);
            let number_of_args = first_part.read_u32::<LittleEndian>().unwrap_or(0) as usize;
            let s = String::from_utf8(second_part.to_vec()).unwrap_or("".to_string());
            let v: Vec<_> = s.split('\0').collect();
            // Skip the exec/program_name pair at the beginning of the buffer.
            for entry in v.iter().skip(1) {
                /* To get CLI arguments, walk the returned list until reaching the appended number
                   or the maximum expected number of arguments.
                   All remaining arguments after that point are environment. */
                if entry != &"" && args.args.len() < number_of_args && args.args.len() < argmax {
                    args.args.push(entry.to_owned().to_string());
                } else if entry.contains('=') && args.args.len() == number_of_args && args.args.len() < argmax {
                    let v2: Vec<_> = entry.split('=').collect();
                    if v2.len() == 2 {
                        args.env.insert(v2[0].to_owned(), v2[1].to_owned());
                    }
                }
            }
        }
        args
    }

    fn gen_proc_cmdline(pid: i32) -> String {
        let argmax = ProcessesRow::gen_max_args();
        // The command line invocation including arguments.
        let args = ProcessesRow::get_proc_raw_args(pid, argmax);
        let cmdline: String = args.args.join(" ");
        cmdline
    }

    fn get_uptime_in_usec() -> i64 {
        let CPU_TIME_RATIO = 1000000 as i64;
        let mut boot_time: timeval = unsafe { mem::zeroed() };
        let mut len = mem::size_of::<timeval>() as size_t;
        let mut mib: [i32; 2] = [CTL_KERN, KERN_BOOTTIME];
        if unsafe { sysctl(&mut mib as *mut _ as *mut i32, 2, &mut boot_time as *mut _ as *mut c_void, &mut len as *mut usize, ptr::null::<c_void>() as *mut _, 0) } < 0 {
            return -1;
        }

        let seconds_since_boot: time_t = boot_time.tv_sec.into();

        let mut tv: timeval = unsafe { mem::zeroed() };
        unsafe { gettimeofday(&mut tv as *mut timeval, ptr::null::<c_void>() as *mut _) };

        // Ignoring boot_time.tv_usec
        unsafe { difftime(tv.tv_sec.into(), seconds_since_boot).round() as i64 * CPU_TIME_RATIO + tv.tv_usec as i64 }
    }

    fn gen_proc_ressource_usage(pid: i32) -> RessourceUsageInternal {
        let CPU_TIME_RATIO = 1000000 as i64;
        let NSECS_IN_USEC = 1000;
        let mut out: RessourceUsageInternal = unsafe { mem::zeroed() };

        let mut rusage_info_data: rusage_info_v2 = unsafe { mem::zeroed() };

        let RUSAGE_INFO_V2 = 2;
        let status = unsafe { proc_pid_rusage(pid, RUSAGE_INFO_V2, &mut rusage_info_data as *mut _ as *mut c_void) }; //(rusage_info_t*) &rusage_info_data);
        // proc_pid_rusage returns -1 if it was unable to gather information
        if status == 0 {
            // size/memory information
            out.wired_size = rusage_info_data.ri_wired_size as i64;
            out.resident_size = rusage_info_data.ri_resident_size as i64;
            out.total_size = rusage_info_data.ri_phys_footprint as i64;

            // time information
            out.user_time = rusage_info_data.ri_user_time as i64 / CPU_TIME_RATIO;
            out.system_time = rusage_info_data.ri_system_time as i64 / CPU_TIME_RATIO;

            // disk i/o information
            out.disk_bytes_read = rusage_info_data.ri_diskio_bytesread as i64;
            out.disk_bytes_written = rusage_info_data.ri_diskio_byteswritten as i64;

            // start_time
            // Initialize time conversions.
            let mut time_base: mach_timebase_info_data_t = unsafe { mem::zeroed() };
            if time_base.denom == 0 {
                unsafe { mach_timebase_info(&mut time_base as *mut mach_timebase_info) };
            }

            // Below is the logic to caculate the start_time since boot time
            // with higher precision
            let uptime = ProcessesRow::get_uptime_in_usec();
            let absoluteTime: u64 = unsafe { mach_absolute_time() };

            let multiply = time_base.numer as c_double / time_base.denom as c_double;
            let diff = (absoluteTime - rusage_info_data.ri_proc_start_abstime) as c_long;

            // This is a negative value
            let seconds_since_launch = (diff * multiply.round() as i64) as c_long / NSECS_IN_USEC;

            // Get the start_time of process since the computer started
            out.start_time = (uptime + seconds_since_launch) / CPU_TIME_RATIO;
        } else {
            out.wired_size = -1;
            out.resident_size = -1;
            out.total_size = -1;
            out.user_time = -1;
            out.system_time = -1;
            out.start_time = -1;
        }
        out
    }

    pub fn get_specific() -> Vec<ProcessesRow> {
        let mut processes_table: Vec<ProcessesRow> = Vec::new();
        let pidlist = ProcessesRow::get_proc_list();
        for pid in pidlist {
            let mut processes_row = ProcessesRow::new();

            processes_row.pid = pid as i64;

            processes_row.cmdline = ProcessesRow::gen_proc_cmdline(pid);

            // The process relative root and current working directory.
            let root_and_cwd = ProcessesRow::get_proc_root_and_cwd(pid);
            processes_row.root = root_and_cwd.root;
            processes_row.cwd = root_and_cwd.cwd;

            let cred = ProcessesRow::gen_proc_cred(pid);
            processes_row.parent = cred.parent as i64;
            processes_row.pgroup = cred.group as i64;
            processes_row.state = match cred.status {
                1 => "I".to_owned(),   //SSLEEP - awaiting an event
                2 => "R".to_owned(),   //SWAIT - (abandoned state)
                3 => "S".to_owned(),   //SRUN - running
                4 => "T".to_owned(),   //SIDL - intermediate state in process creation
                5 => "Z".to_owned(),   //SZOMB - intermediate state in process termination
                _ => "".to_owned(),
            };
            processes_row.nice = cred.nice;
            processes_row.uid = cred.real_uid as i64;
            processes_row.gid = cred.real_gid as i64;
            processes_row.euid = cred.effective_uid as i64;
            processes_row.egid = cred.effective_gid as i64;
            processes_row.suid = cred.saved_uid as i64;
            processes_row.sgid = cred.saved_gid as i64;

            let path_name_and_on_disk = ProcessesRow::gen_proc_name_path_and_on_disk(pid, cred);
            processes_row.path = path_name_and_on_disk.path;
            processes_row.name = path_name_and_on_disk.name;
            processes_row.on_disk = path_name_and_on_disk.on_disk;

            // systems usage and time information
            let ressource_usage = ProcessesRow::gen_proc_ressource_usage(pid);
            processes_row.wired_size = ressource_usage.wired_size;
            processes_row.resident_size = ressource_usage.resident_size;
            processes_row.total_size = ressource_usage.total_size;
            processes_row.user_time = ressource_usage.user_time;
            processes_row.system_time = ressource_usage.system_time;
            processes_row.start_time = ressource_usage.start_time;
            processes_row.disk_bytes_read = ressource_usage.disk_bytes_read;
            processes_row.disk_bytes_written = ressource_usage.disk_bytes_written;

            processes_row.threads = ProcessesRow::gen_proc_num_threads(pid);
            processes_table.push(processes_row);
        }
        processes_table
    }
}

