use std::io::Read;
use std::fs::{read_dir, File, read_link};
use std::collections::HashMap;

use tables::{ProcessOpenSocketsRow, ProcessesRow};

pub struct InternalProcNamespaces {
    cgroup_namespace: String,
    ipc_namespace: String,
    mnt_namespace: String,
    net_namespace: String,
    pid_namespace: String,
    user_namespace: String,
    uts_namespace: String,
}
pub struct InternalPidSockets {
    pub pid: String,
    pub fd: String,
    pub inode: String,
}
pub struct InternalBasicSocketInfo {
    family: String,
    protocol: String,
    local_address: String,
    local_port: String,
    remote_address: String,
    remote_port: String,
    inode: String,
    path: String,
    state: String,
    net_namespace: String,
}

impl ProcessOpenSocketsRow {

    pub fn get_all_open_sockets () -> Option<Vec<InternalPidSockets>> {
        let pid_list = read_dir("/proc").ok()?;
        let mut open_sockets: Vec<InternalPidSockets> = Vec::new();
        for pid in pid_list {
            let pid_no = pid.ok()?.file_name().into_string().ok()?;
            let attr = ProcessesRow::get_proc_attr("fd", &pid_no);
            let fd_dir = read_dir(attr).ok();
            if let Some(files) = fd_dir {
                for file in files {
                    let fd = file.ok()?;
                    let file_name = fd.file_name().into_string().ok()?;
                    let file_path = fd.path();
                    let buff = read_link(&file_path).ok()?;
                    let string_content = buff.to_str()?.to_string();
                    if string_content.starts_with("socket:[") {
                        let v: Vec<_> = string_content.replace("]", "")
                            .split(":[")
                            .map(|s| s.to_string())
                            .collect();
                        if v.len() == 2 {
                            open_sockets.push(
                                InternalPidSockets {
                                    pid: pid_no.to_owned(),
                                    fd: file_name,
                                    inode: v[1].to_string(),
                                }
                            );
                        }
                    }
                }
            }
        }
        Some(open_sockets)
    }

    pub fn internal_get_basic_sockets_info (
        file_path: String,
        expected_format: &str,
        expected_columns: u32,
        family: &str,
        protocol: &str,
        ns: &str
    ) -> Option<Vec<InternalBasicSocketInfo>> {
        let mut socket_info: Vec<InternalBasicSocketInfo> = Vec::new();
        let mut _buff = File::open(file_path).ok()?;
        let mut file_contents = String::new();
        _buff.read_to_string(&mut file_contents);
        let mut lines = file_contents.lines();

        if lines.nth(0)?.contains(expected_format) {
            for line in lines.skip(1) {
                let v: Vec<_> = line.split_whitespace().collect();
                if v.len() == expected_columns as usize {
                    if family == "2" || family == "10" {
                        let local_address: Vec<_> = v[1].split(':').collect();
                        let remote_address: Vec<_> = v[2].split(':').collect();
                        if local_address.len() == 2 && remote_address.len() == 2 {
                            socket_info.push(InternalBasicSocketInfo {
                                family: family.to_owned(),
                                protocol: protocol.to_owned(),
                                local_address: local_address[0].to_owned(),
                                local_port: local_address[1].to_owned(),
                                remote_address: remote_address[0].to_owned(),
                                remote_port: remote_address[1].to_owned(),
                                inode: v[9].to_owned(),
                                path: "".to_owned(),
                                state: v[3].to_owned(),
                                net_namespace: ns.to_owned(),
                            });
                        }
                    }
                        else if family == "1" {
                            socket_info.push(InternalBasicSocketInfo {
                                family: family.to_owned(),
                                protocol: protocol.to_owned(),
                                local_address: "".to_owned(),
                                local_port: "".to_owned(),
                                remote_address: "".to_owned(),
                                remote_port: "".to_owned(),
                                inode: v[6].to_owned(),
                                path: v[7].to_owned(),
                                state: v[5].to_owned(),
                                net_namespace: ns.to_owned(),
                            });

                        }
                }
            }
        }
        Some(socket_info)
    }

    pub fn get_basic_sockets_info(pid: &str, ns: &str) -> Option<Vec<InternalBasicSocketInfo>>{
        let mut out: Vec<InternalBasicSocketInfo> = Vec::new();
        let attr = ProcessesRow::get_proc_attr("net",pid);
        let net_directory = read_dir(attr).ok()?;

        for entry in net_directory {
            let file = entry.ok()?;
            let file_path = file.path().to_str()?.to_string();

            match file.file_name().to_str()? {
                "tcp" => {
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "sl", 17, "2", "6", ns)?);
                }
                "tcp6" => {
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "sl", 17, "10", "6", ns)?);
                }
                "udp" => {
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "sl", 13, "2", "17", ns)?);
                }
                "udp6" => {
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "sl", 13, "10", "17", ns)?);
                }
                "unix" => {
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "Num", 8, "1", "0", ns)?);
                }
                &_ => ()
            }
        }
        Some(out)
    }

    pub fn get_all_network_namespaces () -> Option<HashMap<String, HashMap<String, InternalBasicSocketInfo>>> {
        let mut net_namespaces: HashMap <
            String,
            HashMap <
                String,
                InternalBasicSocketInfo
            >
        > = HashMap::new();
        let pid_list = read_dir("/proc").ok()?;
        for pid in pid_list {
            let pid_no = pid.ok()?.file_name().into_string().ok()?;
            let pid_ns = InternalProcNamespaces::read_full_proc_namespace(&pid_no).unwrap_or(InternalProcNamespaces::new());
            let net_ns = pid_ns.net_namespace;
            if !net_namespaces.contains_key(&net_ns) && !&net_ns.is_empty() {
                let mut net_namespace: HashMap<String, InternalBasicSocketInfo> = HashMap::new();
                let socket_info = ProcessOpenSocketsRow::get_basic_sockets_info(&pid_no, &net_ns)?;
                for entry in socket_info {
                    let inode = entry.inode.clone();
                    net_namespace.insert(inode, entry);
                }
                net_namespaces.insert(net_ns, net_namespace);
            }
        }
        Some(net_namespaces)
    }

    pub fn gen_process_open_sockets_table () -> Vec<ProcessOpenSocketsRow>{
        let mut table: Vec<ProcessOpenSocketsRow> = Vec::new();
        let all_namespaces = ProcessOpenSocketsRow::get_all_network_namespaces().unwrap_or(HashMap::new());
        let mut all_namespaces_left = ProcessOpenSocketsRow::get_all_network_namespaces().unwrap_or(HashMap::new());
        let all_pid_ino = ProcessOpenSocketsRow::get_all_open_sockets().unwrap_or(Vec::new());
        let mut list: Vec<_> = Vec::new();

        for (key, val) in all_namespaces.iter() {
            for entry in &all_pid_ino {
                if val.contains_key(&entry.inode) {
                    if let Some(ns) = val.get(&entry.inode) {
                        list.push(key);
                        table.push(
                            ProcessOpenSocketsRow {
                                pid: entry.pid.parse::<i64>().unwrap_or(-1),
                                fd: entry.fd.parse::<i64>().unwrap_or(-1),
                                socket: ns.inode.parse::<i64>().unwrap_or(-1),
                                family: ns.family.parse::<i32>().unwrap_or(-1),
                                protocol: ns.protocol.parse::<i32>().unwrap_or(-1),
                                local_address: ns.local_address.to_owned(),
                                remote_address: ns.remote_address.to_owned(),
                                local_port: ns.local_port.parse::<i32>().unwrap_or(-1),
                                remote_port: ns.remote_port.parse::<i32>().unwrap_or(-1),
                                path: ns.path.to_owned(),
                                state: ns.state.to_owned(),
                                net_namespace: ns.net_namespace.to_owned(),
                            }
                        );
                    }
                }
            }
        }

        for _entry in list{
            for (_ns_key, mut ns_val) in all_namespaces_left.drain() {
                for (_key, mut val) in ns_val.drain() {
                    table.push(
                        ProcessOpenSocketsRow {
                            pid: -1,
                            fd: -1,
                            socket: val.inode.parse::<i64>().unwrap_or(-1),
                            family: val.family.parse::<i32>().unwrap_or(-1),
                            protocol: val.protocol.parse::<i32>().unwrap_or(-1),
                            local_address: val.local_address.to_owned(),
                            remote_address: val.remote_address.to_owned(),
                            local_port: val.local_port.parse::<i32>().unwrap_or(-1),
                            remote_port: val.remote_port.parse::<i32>().unwrap_or(-1),
                            path: val.path.to_owned(),
                            state: val.state.to_owned(),
                            net_namespace: val.net_namespace.to_owned(),
                        }
                    );
                }
            }
        }
        table
    }
}

impl InternalProcNamespaces {

    fn new () -> InternalProcNamespaces {
        InternalProcNamespaces {
            cgroup_namespace: "".to_string(),
            ipc_namespace: "".to_string(),
            mnt_namespace: "".to_string(),
            net_namespace: "".to_string(),
            pid_namespace: "".to_string(),
            user_namespace: "".to_string(),
            uts_namespace: "".to_string(),
        }
    }

    fn read_full_proc_namespace(pid: &str) -> Option<InternalProcNamespaces> {
        let mut namespaces = InternalProcNamespaces::new();
        let attr = ProcessesRow::get_proc_attr("ns", pid);
        let ns_dir = read_dir(attr).ok();

        if let Some(files) = ns_dir {
            for file in files {
                let file_path = file.ok()?.path();
                let buff = read_link(&file_path).ok()?;
                let content = buff.to_str()?.to_string();
                if content.contains(":[") {
                    let v: Vec<_> = content.replace("]", "")
                        .split(":[")
                        .map(|s| s.to_string())
                        .collect();
                    if v.len() == 2 {
                        match v[0].as_str() {
                            "cgroup" => namespaces.cgroup_namespace = v.get(1)?.to_owned(),
                            "ipc" => namespaces.ipc_namespace = v.get(1)?.to_owned(),
                            "mnt" => namespaces.mnt_namespace = v.get(1)?.to_owned(),
                            "net" => namespaces.net_namespace = v.get(1)?.to_owned(),
                            "pid" => namespaces.pid_namespace = v.get(1)?.to_owned(),
                            "user" => namespaces.user_namespace = v.get(1)?.to_owned(),
                            "uts" => namespaces.uts_namespace = v.get(1)?.to_owned(),
                            &_ => (),
                        }
                    }
                }
            }
        }
        Some(namespaces)
    }
}

