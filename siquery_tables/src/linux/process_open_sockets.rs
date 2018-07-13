use std::io::Read;
use std::fs::{read_dir, File, read_link};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

use tables::{ProcessOpenSocketsRow, ProcessesRow};
use linux::SystemReaderInterface;

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
        let dir_entries = read_dir("/proc").ok()?;
        let mut open_sockets: Vec<InternalPidSockets> = Vec::new();
        for dir_entry in dir_entries {
            let pid = dir_entry.ok()?.file_name().into_string().ok()?;
            let attr = ProcessesRow::get_proc_attr("fd", &pid);
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
                                    pid: pid.to_owned(),
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

    #[allow(unused_must_use)]
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
        if let Ok(_bytes_read) = _buff.read_to_string(&mut file_contents) {
            let mut lines = file_contents.lines();

            if lines.nth(0)?.contains(expected_format) {
                for line in lines.skip(1) {
                    let v: Vec<_> = line.split_whitespace().collect();
                    if family == "2" || family == "10" {
                        if v.len() == expected_columns as usize {
                            let local_address_and_port: Vec<_> = v[1].split(':').collect();
                            let remote_address_and_port: Vec<_> = v[2].split(':').collect();
                            if local_address_and_port.len() == 2 && remote_address_and_port.len() == 2 {
                                let mut local_address: String;
                                match local_address_and_port[0].len() {
                                    8 => {
                                        // IPv4 is represented as 8 bytes in the tcp6 file in linux.
                                        local_address = Ipv4Addr::from(u32::from_be(u32::from_str_radix(local_address_and_port[0], 16).unwrap_or(1))).to_string();
                                    }
                                    32 => {
                                        /* As explained here:
                                    https://serverfault.com/questions/592574/why-does-proc-net-tcp6-represents-1-as-1000
                                    In the tcp6 file, the address is handled as four
                                    little-endian words consisting of 4 bytes each.
                                    Then, for proper network byte-order,
                                    adjacent words must also be swapped. */

                                        // Split the address in words of 4 bytes.
                                        let (a, _buf) = local_address_and_port[0].split_at(4);
                                        let (b, _buf) = _buf.split_at(4);
                                        let (c, _buf) = _buf.split_at(4);
                                        let (d, _buf) = _buf.split_at(4);
                                        let (e, _buf) = _buf.split_at(4);
                                        let (f, _buf) = _buf.split_at(4);
                                        let (g, _buf) = _buf.split_at(4);
                                        let (h, _buf) = _buf.split_at(4);

                                        /* Parse every word as u16,
                                    convert to big-endian and swap adjacent words. */
                                        let ip = [
                                            u16::from_be(u16::from_str_radix(b, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(a, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(d, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(c, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(f, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(e, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(h, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(g, 16).unwrap_or(1)),
                                        ];
                                        local_address = Ipv6Addr::from(ip).to_string();
                                    }
                                    _ => {
                                        local_address = "An error occurred while parsing the local address: the address does not have an expected format".to_string();
                                    }
                                }
                                let mut remote_address: String;
                                match local_address_and_port[0].len() {
                                    8 => {
                                        // IPv4 is represented as 8 bytes in the tcp6 file in linux.
                                        remote_address = Ipv4Addr::from(u32::from_be(u32::from_str_radix(local_address_and_port[0], 16).unwrap_or(1))).to_string();
                                    }
                                    32 => {
                                        /* As explained here:
                                    https://serverfault.com/questions/592574/why-does-proc-net-tcp6-represents-1-as-1000
                                    In the tcp6 file, the address is handled as four
                                    little-endian words consisting of 4 bytes each.
                                    Then, for proper network byte-order,
                                    adjacent words must also be swapped. */

                                        // Split the address in words of 4 bytes.
                                        let (a, _buf) = local_address_and_port[0].split_at(4);
                                        let (b, _buf) = _buf.split_at(4);
                                        let (c, _buf) = _buf.split_at(4);
                                        let (d, _buf) = _buf.split_at(4);
                                        let (e, _buf) = _buf.split_at(4);
                                        let (f, _buf) = _buf.split_at(4);
                                        let (g, _buf) = _buf.split_at(4);
                                        let (h, _buf) = _buf.split_at(4);

                                        /* Parse every word as u16,
                                    convert to big-endian and swap adjacent words. */
                                        let ip = [
                                            u16::from_be(u16::from_str_radix(b, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(a, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(d, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(c, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(f, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(e, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(h, 16).unwrap_or(1)),
                                            u16::from_be(u16::from_str_radix(g, 16).unwrap_or(1)),
                                        ];
                                        remote_address = Ipv6Addr::from(ip).to_string();
                                    }
                                    _ => {
                                        remote_address = "An error occurred while parsing the local address: the address does not have an expected format".to_string();
                                    }
                                }
                                socket_info.push(InternalBasicSocketInfo {
                                    family: family.to_owned(),
                                    protocol: protocol.to_owned(),
                                    local_address,
                                    local_port: local_address_and_port[1].to_owned(),
                                    remote_address,
                                    remote_port: remote_address_and_port[1].to_owned(),
                                    inode: v[9].to_owned(),
                                    path: "".to_owned(),
                                    state: v[3].to_owned(),
                                    net_namespace: ns.to_owned(),
                                });
                            }
                        }
                    }
                        else if family == "1" {
                            if v.len() >= expected_columns as usize {

                                let mut path = String::new();
                                if v.len() == 8 {
                                    path = v[7].to_owned();
                                } else if v.len() == 7 {
                                    path = "".to_owned()
                                }
                                socket_info.push(InternalBasicSocketInfo {
                                    family: family.to_owned(),
                                    protocol: protocol.to_owned(),
                                    local_address: "".to_owned(),
                                    local_port: "".to_owned(),
                                    remote_address: "".to_owned(),
                                    remote_port: "".to_owned(),
                                    inode: v[6].to_owned(),
                                    path,
                                    state: v[5].to_owned(),
                                    net_namespace: ns.to_owned(),
                                });

                            }
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
                    out.append(&mut ProcessOpenSocketsRow::internal_get_basic_sockets_info (file_path, "Num", 7, "1", "0", ns)?);
                }
                &_ => ()
            }
        }
        Some(out)
    }

    pub fn get_all_network_namespaces () -> Option<HashMap<String, HashMap<String, InternalBasicSocketInfo>>> {
        // Group all sockets by inodes and group inodes by network namespaces.
        let mut net_namespaces: HashMap <
            String, // Network namespace.
            HashMap <
                String, // Inode.
                InternalBasicSocketInfo // Socket information.
            >
        > = HashMap::new();
        let dir_entries = read_dir("/proc").ok()?;
        for dir_entry in dir_entries {
            let pid = dir_entry.ok()?.file_name().into_string().ok()?;
            let pid_ns = InternalProcNamespaces::read_full_proc_namespace(&pid).unwrap_or(InternalProcNamespaces::new());
            let mut net_ns : String;
            /* When net_namespace is not available, assign the current pid to a net_namespace of 0.
            Basic socket information will be collect for the first pid in the list.*/
            if pid_ns.net_namespace.is_empty() {
                net_ns = "0".to_string()
            } else {
                net_ns = pid_ns.net_namespace
            }
            /* To be executed every time a new net_namespace is found.
            Collects basic socket information for the namespace.*/
            if !net_namespaces.contains_key(&net_ns) {
                let mut net_namespace: HashMap<String, InternalBasicSocketInfo> = HashMap::new();
                let socket_info = ProcessOpenSocketsRow::get_basic_sockets_info(&pid, &net_ns).unwrap_or(Vec::new());
                for entry in socket_info {
                    let inode = entry.inode.clone();
                    net_namespace.insert(inode, entry);
                }
                net_namespaces.insert(net_ns, net_namespace);
            }
        }
        Some(net_namespaces)
    }

    pub fn get_specific (_system_reader: &SystemReaderInterface) -> Vec<ProcessOpenSocketsRow>{
        let mut table: Vec<ProcessOpenSocketsRow> = Vec::new();
        let mut all_namespaces = ProcessOpenSocketsRow::get_all_network_namespaces().unwrap_or(HashMap::new());
        let all_pid_ino = ProcessOpenSocketsRow::get_all_open_sockets().unwrap_or(Vec::new());
        for (_namespace_key, val) in all_namespaces.iter_mut() {
            for entry in &all_pid_ino {
                if let Some(ns) = val.remove(&entry.inode) {
                    let mut state_string : String;
                    match &ns.state as &str {
                        "01" => state_string = "ESTABLISHED".to_owned(),
                        "02" => state_string = "SYN_SENT".to_owned(),
                        "03" => state_string = "SYN_RECV".to_owned(),
                        "04" => state_string = "FIN_WAIT1".to_owned(),
                        "05" => state_string = "FIN_WAIT2".to_owned(),
                        "06" => state_string = "TIME_WAIT".to_owned(),
                        "07" => state_string = "CLOSE".to_owned(),
                        "08" => state_string = "CLOSE_WAIT".to_owned(),
                        "09" => state_string = "LAST_ACK".to_owned(),
                        "0A" => state_string = "LISTEN".to_owned(),
                        "0B" => state_string = "CLOSING".to_owned(),
                        &_ => state_string = "UNKNOWN".to_owned(),
                    }
                    table.push(
                        ProcessOpenSocketsRow {
                            pid: entry.pid.parse::<i64>().unwrap_or(-1),
                            fd: entry.fd.parse::<i64>().unwrap_or(-1),
                            socket: ns.inode.parse::<i64>().unwrap_or(-1),
                            family: ns.family.parse::<i32>().unwrap_or(-1),
                            protocol: ns.protocol.parse::<i32>().unwrap_or(-1),
                            local_address: ns.local_address.to_owned(),
                            remote_address: ns.remote_address.to_owned(),
                            local_port: ns.local_port.parse::<i32>().unwrap_or(0),
                            remote_port: ns.remote_port.parse::<i32>().unwrap_or(0),
                            path: ns.path.to_owned(),
                            state: state_string,
                            net_namespace: ns.net_namespace.to_owned(),
                        }
                    );
                }
            }
            // Set the pid and fd of all the orphan net_namespace inodes to -1.
            for (_key, ns) in val.drain() {
                let mut state_string : String;
                match &ns.state as &str {
                    "01" => state_string = "ESTABLISHED".to_owned(),
                    "02" => state_string = "SYN_SENT".to_owned(),
                    "03" => state_string = "SYN_RECV".to_owned(),
                    "04" => state_string = "FIN_WAIT1".to_owned(),
                    "05" => state_string = "FIN_WAIT2".to_owned(),
                    "06" => state_string = "TIME_WAIT".to_owned(),
                    "07" => state_string = "CLOSE".to_owned(),
                    "08" => state_string = "CLOSE_WAIT".to_owned(),
                    "09" => state_string = "LAST_ACK".to_owned(),
                    "0A" => state_string = "LISTEN".to_owned(),
                    "0B" => state_string = "CLOSING".to_owned(),
                    &_ => state_string = "UNKNOWN".to_owned(),
                }
                table.push(
                    ProcessOpenSocketsRow {
                        pid: -1,
                        fd: -1,
                        socket: ns.inode.parse::<i64>().unwrap_or(-1),
                        family: ns.family.parse::<i32>().unwrap_or(-1),
                        protocol: ns.protocol.parse::<i32>().unwrap_or(-1),
                        local_address: ns.local_address.to_owned(),
                        remote_address: ns.remote_address.to_owned(),
                        local_port: ns.local_port.parse::<i32>().unwrap_or(0),
                        remote_port: ns.remote_port.parse::<i32>().unwrap_or(0),
                        path: ns.path.to_owned(),
                        state: state_string,
                        net_namespace: ns.net_namespace.to_owned(),
                    }
                );
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
        let ns_dir_entries = read_dir(attr).ok();

        if let Some(files) = ns_dir_entries {
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

