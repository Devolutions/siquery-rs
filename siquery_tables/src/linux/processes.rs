use regex::Regex;

use std::io::Read;
use std::fs::{read_dir, File, canonicalize, read_link};

use linux::SystemReaderInterface;

use tables::{
    ProcessesRow,
    ProcessEnvsRow,
    ProcessMemoryMapRow,
};

pub struct SimpleProcStat {
    name: String,
    real_uid: String,
    real_gid: String,
    effective_uid: String,
    effective_gid: String,
    saved_uid: String,
    saved_gid: String,
    resident_size: String,
    total_size: String,
    state: String,
    parent: String,
    group: String,
    nice: String,
    threads: String,
    user_time: String,
    system_time: String,
    start_time: String,
}

pub struct SimpleProcIo {
    read_bytes: String,
    write_bytes: String,
    cancelled_write_bytes: String,
}

pub struct SimpleProcNs {
    cgroup_namespace: String,
    ipc_namespace: String,
    mnt_namespace: String,
    net_namespace: String,
    pid_namespace: String,
    user_namespace: String,
    uts_namespace: String,
}

fn get_proc_list_internal () -> Option<Vec<String>> {
    let mut pid_list: Vec<String> = Vec::new();
    let regex = Regex::new(r"(?m)\d{1,5}").ok()?;
    let paths = read_dir("/proc").ok()?;
    for path in paths {
        let unwrapped_path = path.ok()?.path();
        let regex_captures = regex.captures(unwrapped_path.to_str()?);
        if let Some(capture) = regex_captures {
            if let Some(pid) = capture.get(0) {
                pid_list.push(pid.as_str().to_string());
            }
        }
    }
    Some(pid_list)
}

fn get_proc_list() -> Vec<String> {
    get_proc_list_internal().unwrap_or_else(|| Vec::new())
}

impl ProcessesRow {

    pub fn get_proc_attr (attr: &str, pid: &str) -> String {
        format!("/proc/{}/{}",pid,attr)
    }

    pub fn read_proc_cmdline (pid: &str) -> String {
        let mut content = String::new();
        let attr = ProcessesRow::get_proc_attr("cmdline", pid);

        if let Some(mut file) = File::open(attr).ok() {
            &file.read_to_string(&mut content);
        }
        content.trim().to_owned()
    }

    pub fn read_proc_link (attr: &str, pid: &str) -> String {
        let attr_path = ProcessesRow::get_proc_attr(attr, pid);
        let mut link: String = String::new();
        if let Some(full_path) = canonicalize(attr_path).ok() {
            link = full_path.to_str().unwrap_or("").to_string();
        }
        link
    }

    pub fn gen_processes_row (pid: &str) -> ProcessesRow {

        // Parse /proc/status and /proc/stat
        let proc_stat = SimpleProcStat::get_proc_stat(pid);

        // Parse /proc/io
        let proc_io = SimpleProcIo::get_proc_io(pid);

        // Parse /proc/ns
        let proc_ns = SimpleProcNs::read_full_proc_namespace(pid).unwrap_or(SimpleProcNs::new());

        ProcessesRow {
            pid: pid.parse::<i64>().unwrap_or(-2),
            name: proc_stat.name,
            path: ProcessesRow::read_proc_link("exe", pid),
            cmdline: ProcessesRow::read_proc_cmdline(pid).replace("\0", " "),
            state: proc_stat.state,
            cwd: ProcessesRow::read_proc_link("cwd", pid),
            root: ProcessesRow::read_proc_link("root", pid),
            uid: proc_stat.real_uid.parse::<i64>().unwrap_or(0),
            gid: proc_stat.real_gid.parse::<i64>().unwrap_or(0),
            euid: proc_stat.effective_uid.parse::<i64>().unwrap_or(0),
            egid: proc_stat.effective_gid.parse::<i64>().unwrap_or(0),
            suid: proc_stat.saved_uid.parse::<i64>().unwrap_or(0),
            sgid: proc_stat.saved_gid.parse::<i64>().unwrap_or(0),
            on_disk: -2,    //TODO
            wired_size: 0,    // No support for unpagable counters in linux
            resident_size: proc_stat.resident_size.parse::<i64>().unwrap_or(0),
            total_size:  proc_stat.total_size.parse::<i64>().unwrap_or(0),
            user_time: proc_stat.user_time.parse::<i64>().unwrap_or(0),
            system_time: proc_stat.system_time.parse::<i64>().unwrap_or(0),
            disk_bytes_read: proc_io.read_bytes.parse::<i64>().unwrap_or(0),
            disk_bytes_written: proc_io.write_bytes.parse::<i64>().unwrap_or(0) +  proc_io.cancelled_write_bytes.parse::<i64>().unwrap_or(0),
            start_time: proc_stat.start_time.parse::<i64>().unwrap_or(0),
            parent: proc_stat.parent.parse::<i64>().unwrap_or(0),
            pgroup: proc_stat.group.parse::<i64>().unwrap_or(0),
            threads: proc_stat.threads.parse::<i32>().unwrap_or(0),
            nice: proc_stat.nice.parse::<i32>().unwrap_or(0),
            is_elevated_token: 0,   // NA for linux
            cgroup_namespace: proc_ns.cgroup_namespace,
            ipc_namespace: proc_ns.ipc_namespace,
            mnt_namespace: proc_ns.mnt_namespace,
            net_namespace: proc_ns.net_namespace,
            pid_namespace: proc_ns.pid_namespace,
            user_namespace: proc_ns.user_namespace,
            uts_namespace: proc_ns.uts_namespace,
        }

    }
    #[allow(unused_variables)]
    pub fn gen_processes_table (_system_reader: &SystemReaderInterface) -> Vec<ProcessesRow> {
        let pid_list = get_proc_list();
        let mut pid_table: Vec<ProcessesRow> = Vec::new();
        for pid in pid_list.iter() {
            pid_table.push(ProcessesRow::gen_processes_row (pid));
        }
        pid_table
    }
}

impl ProcessEnvsRow {

    pub fn gen_proc_environ_row(pid: &str) -> Option<Vec<ProcessEnvsRow>> {
        let mut table: Vec<ProcessEnvsRow> = Vec::new();
        let attr = ProcessesRow::get_proc_attr("environ", pid);
        let mut content = String::new();
        let _result = File::open(attr).ok()?.read_to_string(&mut content);
        let buff = content.to_owned();
        // Read a NULL delimited string
        let lines: Vec<_> = buff.split("\0").collect();
        for line in lines {
            let line: Vec<_> = line.split('=').collect();
            if line.len() == 2 {
                table.push (
                    ProcessEnvsRow {
                        pid: pid.to_owned().parse::<i32>().unwrap_or(-1),
                        key: line[0].to_owned(),
                        value: line[1].to_owned(),
                    }
                );
            }
        }
        Some(table)
    }

    pub fn gen_proc_environ_table() -> Vec<ProcessEnvsRow> {
        let pid_list = get_proc_list();
        let mut table: Vec<ProcessEnvsRow> = Vec::new();
        for pid in pid_list.iter() {
            table.append(&mut ProcessEnvsRow::gen_proc_environ_row (pid).unwrap_or_else(|| Vec::new()));
        }
        table
    }
}

impl ProcessMemoryMapRow {

    pub (crate) fn new() -> ProcessMemoryMapRow {
        ProcessMemoryMapRow {
            pid: 0,
            start: "".to_string(),
            end: "".to_string(),
            permissions: "".to_string(),
            offset: 0,
            device: "".to_string(),
            inode: 0,
            path: "".to_string(),
            pseudo: 1,
        }
    }

    fn gen_process_map_internal (pid: &str) -> Option<Vec<ProcessMemoryMapRow>>{
        let map = ProcessesRow::get_proc_attr("maps", pid);
        let mut content = String::new();
        let mut table_row: Vec<ProcessMemoryMapRow> = Vec::new();
        if let Some(mut file) = File::open(map).ok() {
            &file.read_to_string(&mut content);
        }

        for line in content.split('\n') {
            let mut struct_buffer = ProcessMemoryMapRow::new();
            let fields: Vec<_> = line.split_whitespace().collect();

            if fields.len() >= 5 {

                let address: Vec<_> = fields[0].split('-').collect();
                if address.len() == 2 {
                    struct_buffer.start = address[0].to_owned();
                    struct_buffer.end = address[1].to_owned();
                }
                struct_buffer.pid = pid.to_owned().parse::<i32>().unwrap_or(-1);
                struct_buffer.permissions = fields[1].to_owned();
                struct_buffer.offset = fields[2].to_owned().parse::<i64>().unwrap_or(-1);
                struct_buffer.device = fields[3].to_owned();
                struct_buffer.inode = fields[4].to_owned().parse::<i32>().unwrap_or(-1);
                if fields.len() == 6 {
                    struct_buffer.path = fields[5].to_owned();
                }
                if (struct_buffer.inode == 0) && (struct_buffer.path == "".to_owned()) {
                    struct_buffer.pseudo = 0;
                }
            }
            table_row.push(struct_buffer);
        }
        Some(table_row)
    }

<<<<<<< HEAD
    pub fn gen_process_memory_map_table () -> Vec<Vec<ProcessMemoryMapRow>> {
=======
    pub fn gen_process_map () -> Vec<ProcessMemoryMapRow> {
>>>>>>> 4425f055164c5ce7df613c341b8cc6cdb40f3feb
        let pid_list = get_proc_list();
        let mut table: Vec<ProcessMemoryMapRow> = Vec::new();
        for pid in pid_list.iter() {
            table.append(&mut ProcessMemoryMapRow::gen_process_map_internal (pid).unwrap_or_else(|| Vec::new()));
        }
        table
    }
}

impl SimpleProcStat {

    pub(crate) fn new() -> SimpleProcStat {
        SimpleProcStat {
            name: "".to_string(),
            real_uid: "".to_string(),
            real_gid: "".to_string(),
            effective_uid: "".to_string(),
            effective_gid: "".to_string(),
            saved_uid: "".to_string(),
            saved_gid: "".to_string(),
            resident_size: "".to_string(),
            total_size: "".to_string(),
            state: "".to_string(),
            parent: "".to_string(),
            group: "".to_string(),
            nice: "".to_string(),
            threads: "".to_string(),
            user_time: "".to_string(),
            system_time: "".to_string(),
            start_time: "".to_string(),
        }
    }

    pub fn get_proc_stat(pid: &str) -> SimpleProcStat {
        let status_attr =ProcessesRow::get_proc_attr("status", pid);
        let stat_attr =ProcessesRow::get_proc_attr("stat", pid);
        let mut status_content = String::new();
        let mut stat_content = String::new();
        let mut proc_stat = SimpleProcStat::new();

        if let Some(mut file) = File::open(status_attr).ok() {
            &file.read_to_string(&mut status_content);
        }

        for line in status_content.lines() {
            if line.starts_with("Name") {
                let name: Vec<_> = line.split_whitespace().collect();
                if name.len() == 2 {
                    proc_stat.name = name[1].to_owned();
                }
            }
            if line.starts_with("VmRSS") {
                let vmrss: Vec<_> = line.split_whitespace().collect();
                if vmrss.len() == 3 {
                    proc_stat.resident_size = vmrss[1].to_owned() + "000";
                }
            }
            if line.starts_with("VmSize") {
                let vmsize: Vec<_> = line.split_whitespace().collect();
                if vmsize.len() == 3 {
                    proc_stat.total_size = vmsize[1].to_owned() + "000";
                }
            }
            if line.starts_with("Gid") {
                let v: Vec<_> = line.split_whitespace().collect();
                if v.len() == 5 {
                    proc_stat.real_gid = v[1].to_owned();
                    proc_stat.effective_gid = v[2].to_owned();
                    proc_stat.saved_gid = v[3].to_owned();
                }
            }
            if line.starts_with("Uid") {
                let v: Vec<_> = line.split_whitespace().collect();
                if v.len() == 5 {
                    proc_stat.real_uid = v[1].to_owned();
                    proc_stat.effective_uid = v[2].to_owned();
                    proc_stat.saved_uid = v[3].to_owned();
                }
            }
        }

        if let Some(mut file) = File::open(stat_attr).ok() {
            &file.read_to_string(&mut stat_content);
        }

        let buff: Vec<_> = stat_content.split(')').collect();
        if buff.len() >= 1 {
            let stat_info_extract: Vec<_> = buff[1].split_whitespace().collect();
            if stat_info_extract.len() >= 18 {
                proc_stat.state = stat_info_extract[0].to_owned();
                proc_stat.parent = stat_info_extract[1].to_owned();
                proc_stat.group = stat_info_extract[2].to_owned();
                proc_stat.user_time = stat_info_extract[11].to_owned();
                proc_stat.system_time = stat_info_extract[12].to_owned();
                proc_stat.nice = stat_info_extract[16].to_owned();
                proc_stat.threads = stat_info_extract[17].to_owned();
                proc_stat.start_time = stat_info_extract[19].to_owned();
            }
        }
        proc_stat
    }
}

impl SimpleProcIo {
    pub(crate) fn new() -> SimpleProcIo {
        SimpleProcIo {
            read_bytes: "".to_string(),
            write_bytes: "".to_string(),
            cancelled_write_bytes: "".to_string(),
        }
    }

    pub fn get_proc_io(pid: &str) -> SimpleProcIo {
        let attr =ProcessesRow::get_proc_attr("io", pid);
        let mut proc_io = SimpleProcIo::new();
        let mut io_content = String::new();

        if let Some(mut file) = File::open(attr).ok() {
            &file.read_to_string(&mut io_content);
        }

        for line in io_content.lines() {
            let buffer: Vec<_> = line.split(':').collect();
            if buffer.len() == 2 {
                if buffer[0] == "read_bytes" {
                    proc_io.read_bytes = buffer[1].trim().to_owned();
                } else if buffer[0] == "write_bytes" {
                    proc_io.write_bytes = buffer[1].trim().to_owned();
                } else if buffer[0] == "cancelled_write_bytes" {
                    proc_io.cancelled_write_bytes = buffer[1].trim().to_owned();
                }
            }
        }
        proc_io
    }
}

impl SimpleProcNs {

    pub (crate) fn new () -> SimpleProcNs {
        SimpleProcNs {
            cgroup_namespace: "".to_string(),
            ipc_namespace: "".to_string(),
            mnt_namespace: "".to_string(),
            net_namespace: "".to_string(),
            pid_namespace: "".to_string(),
            user_namespace: "".to_string(),
            uts_namespace: "".to_string(),
        }
    }

    pub fn read_full_proc_namespace(pid: &str) -> Option<SimpleProcNs> {
        let mut namespace = SimpleProcNs::new();
        let attr =ProcessesRow::get_proc_attr("ns", pid);
        let ns_folder = read_dir(attr).ok();
        if let Some(files) = ns_folder {
            for file in files {
                let file_path = file.ok()?.path();
                let buff = read_link(&file_path).ok()?;
                let content = buff.to_str()?.to_string();
                if content.contains(":[") {
                    let v: Vec<_> = content.replace("]", "").split(":[").map(|s| s.to_string()).collect();
                    if v.len() == 2 {
                        match v[0].as_str() {
                            "cgroup" => namespace.cgroup_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "ipc" => namespace.ipc_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "mnt" => namespace.mnt_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "net" => namespace.net_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "pid" => namespace.pid_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "user" => namespace.user_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            "uts" => namespace.uts_namespace = v.get(1).unwrap_or(&"".to_string()).to_owned(),
                            &_ => (),
                        }
                    }
                }
            }
        }
        Some(namespace)
    }
}

