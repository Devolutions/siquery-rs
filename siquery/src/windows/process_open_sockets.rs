#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{ptr,net::Ipv4Addr,net::Ipv6Addr};
use crate::tables::ProcessOpenSocketsRow;
use winapi::shared::minwindef::{
    DWORD,
    PDWORD,
    BOOL,
    ULONG,
};
#[allow(unused_imports)]
use winapi::shared::ws2def::{AF_INET,AF_INET6};

#[link(name = "iphlpapi")]
extern "system" {
    fn GetExtendedTcpTable (
        output_table: *mut u8,                  //Out -> Table Struct
        pdwSize: PDWORD,                        //Out -> Estimated table size in bytes
        bOrder: BOOL,                           //In  -> Set true to sort table
        ulAf: i32,                              //In  -> AF_INET for ipv4 or AF_INET6 for ipv6
        TableClass: TCP_TABLE_CLASS,            //In  -> TCP table structure to retrieve
        Reserved: ULONG                         //In  -> Reserved, must be 0
    ) -> DWORD;

    fn GetExtendedUdpTable (
        output_table: *mut u8,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: i32,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG
    ) -> DWORD;
}

fn get_extended_tcp_table_internal(
    out_table: *mut u8,
    out_table_size: PDWORD,
    in_order: BOOL,
    in_ip_version: i32,
    in_table_class: TCP_TABLE_CLASS,
    in_reserved: ULONG
) -> u32 {

    let return_int: u32;
    unsafe {
        return_int = GetExtendedTcpTable(
            out_table,
            out_table_size,
            in_order,
            in_ip_version,
            in_table_class,
            in_reserved
        );
    }
    return_int
}

fn get_extended_udp_table (
    out_table: *mut u8,
    out_table_size: PDWORD,
    in_order: BOOL,
    in_ip_version: i32,
    in_table_class: UDP_TABLE_CLASS,
    in_reserved: ULONG
) -> u32 {
    let return_int: u32;
    unsafe {
        return_int = GetExtendedUdpTable(
            out_table,
            out_table_size,
            in_order,
            in_ip_version,
            in_table_class,
            in_reserved
        );
    }
    return_int
}

//
//TCP
//
#[repr(C)]
STRUCT! {
    struct MIB_TCPTABLE_OWNER_PID {
        dwNumEntries: DWORD,
        table: MIB_TCPROW_OWNER_PID,
    }
}
#[repr(C)]
STRUCT! {
    struct MIB_TCPROW_OWNER_PID {
        dwState: DWORD,
        dwLocalAddr: DWORD,
        dwLocalPort: DWORD,
        dwRemoteAddr: DWORD,
        dwRemotePort: DWORD,
        dwOwningPid: DWORD,
    }
}
#[repr(C)]
#[allow(dead_code)]
enum TCP_TABLE_CLASS{
    TCP_TABLE_BASIC_LISTENER            = 0,
    TCP_TABLE_BASIC_CONNECTIONS         = 1,
    TCP_TABLE_BASIC_ALL                 = 2,
    TCP_TABLE_OWNER_PID_LISTENER        = 3,
    TCP_TABLE_OWNER_PID_CONNECTIONS     = 4,
    TCP_TABLE_OWNER_PID_ALL             = 5,
    TCP_TABLE_OWNER_MODULE_LISTENER     = 6,
    TCP_TABLE_OWNER_MODULE_CONNECTIONS  = 7,
    TCP_TABLE_OWNER_MODULE_ALL          = 8
}

//
//TCP6
//
#[repr(C)]
STRUCT! {
    struct MIB_TCP6TABLE_OWNER_PID {
        dwNumEntries: DWORD,
        table: MIB_TCP6ROW_OWNER_PID,
    }
}
#[repr(C)]
STRUCT! {
    struct MIB_TCP6ROW_OWNER_PID {
        ucLocalAddr: [u8; 16],
        dwLocalScopeId: DWORD,
        dwLocalPort: DWORD,
        ucRemoteAddr: [u8; 16],
        dwRemoteScopeId: DWORD,
        dwRemotePort: DWORD,
        dwState: DWORD,
        dwOwningPid: DWORD,
    }
}

//
//UDP
//
#[repr(c)]
STRUCT! {
    struct MIB_UDPTABLE_OWNER_PID {
        dwNumEntries: DWORD,
        table: MIB_UDPROW_OWNER_PID,
    }
}
#[repr(C)]
STRUCT! {
    struct MIB_UDPROW_OWNER_PID {
        dwLocalAddr: DWORD,
        dwLocalPort: DWORD,
        dwOwningPid: DWORD,
    }
}

//
//UDP6
//
#[repr(c)]
STRUCT! {
    struct MIB_UDP6TABLE_OWNER_PID {
        dwNumEntries: DWORD,
        table: MIB_UDP6ROW_OWNER_PID,
    }
}
#[repr(C)]
STRUCT! {struct MIB_UDP6ROW_OWNER_PID {
  ucLocalAddr: [u8; 16],
  dwLocalScopeId: DWORD,
  dwLocalPort: DWORD,
  dwOwningPid: DWORD,
}}
#[repr(C)]
#[allow(dead_code)]
enum UDP_TABLE_CLASS{
    UDP_TABLE_BASIC             = 0,
    UDP_TABLE_OWNER_PID         = 1,
    UDP_TABLE_OWNER_MODULE      = 2,
}

fn parse_socket_table(socket_type:SocketType, output_table: &mut Vec<ProcessOpenSocketsRow>) {

    let mut table_size: u32 = 0;
    let table_size_ptr: *mut u32 = &mut table_size;

    let mut connection_state: &str;

    match socket_type {

        SocketType::Tcp => {

            let mut buffer: Vec<u8>;
            let tcp_return_val: u32;
            let tcp_table_ptr: *const MIB_TCPTABLE_OWNER_PID;

            get_extended_tcp_table_internal(
                ptr::null_mut(),
                table_size_ptr,
                1,
                AF_INET,
                TCP_TABLE_CLASS::TCP_TABLE_OWNER_PID_ALL,
                0
            );

            buffer = Vec::with_capacity(table_size as usize);
            tcp_return_val = get_extended_tcp_table_internal(
                buffer.as_mut_ptr(),
                table_size_ptr,
                1,
                AF_INET,
                TCP_TABLE_CLASS::TCP_TABLE_OWNER_PID_ALL,
                0
            );
            tcp_table_ptr = buffer.as_mut_ptr() as *const MIB_TCPTABLE_OWNER_PID;

            if tcp_return_val == 0 && table_size > 0 {
                unsafe {
                    let tcp_table = &(*tcp_table_ptr).table as *const MIB_TCPROW_OWNER_PID;
                    let num_entries = (*tcp_table_ptr).dwNumEntries;

                    for i in 0..num_entries {
                        let tcp_row = *tcp_table.offset(i as isize);
                        let connection_state_int = tcp_row.dwState;
                        match connection_state_int {
                            1 => connection_state = "CLOSED",
                            2 => connection_state = "LISTEN",
                            3 => connection_state = "SYN_SENT",
                            4 => connection_state = "SYN_RCVD",
                            5 => connection_state = "ESTAB",
                            6 => connection_state = "FIN_WAIT1",
                            7 => connection_state = "FIN_WAIT2",
                            8 => connection_state = "CLOSE_WAIT",
                            9 => connection_state = "CLOSING",
                            10 => connection_state = "LAST_ACK",
                            11 => connection_state = "TIME_WAIT",
                            12 => connection_state = "DELETE_TCB",
                            _ => connection_state = "UNKNOWN"
                        };
                        &output_table.push(ProcessOpenSocketsRow {
                            pid: tcp_row.dwOwningPid as i64,
                            fd: 0,  // NA for windows
                            socket: 0,  // NA for windows
                            family: 2,  // IPv4
                            protocol: 6,    // TCP
                            local_address: Ipv4Addr::from(u32::from_be(tcp_row.dwLocalAddr)).to_string(),
                            remote_address: Ipv4Addr::from(u32::from_be(tcp_row.dwRemoteAddr)).to_string(),
                            local_port: u16::from_be(tcp_row.dwLocalPort as u16) as i32,
                            remote_port: u16::from_be(tcp_row.dwRemotePort as u16) as i32,
                            path: "".to_string(),   // NA for windows
                            state: connection_state.to_string(),
                            net_namespace: "".to_string(),  // NA for windows
                        });
                    };
                }
            }
        },

        SocketType::Tcp6 => {

            let mut buffer: Vec<u8>;
            let tcp6_return_val: u32;
            let tcp6_table_ptr: *const MIB_TCP6TABLE_OWNER_PID;

            get_extended_tcp_table_internal(ptr::null_mut(), table_size_ptr, 1, AF_INET6, TCP_TABLE_CLASS::TCP_TABLE_OWNER_PID_ALL, 0);

            buffer = Vec::with_capacity(table_size as usize);
            tcp6_return_val = get_extended_tcp_table_internal(buffer.as_mut_ptr(), table_size_ptr, 1, AF_INET6, TCP_TABLE_CLASS::TCP_TABLE_OWNER_PID_ALL, 0);
            tcp6_table_ptr = buffer.as_mut_ptr() as *const MIB_TCP6TABLE_OWNER_PID;

            if tcp6_return_val == 0 && table_size > 0 {
                unsafe {

                    let tcp6_table = &(*tcp6_table_ptr).table as *const MIB_TCP6ROW_OWNER_PID;

                    let num_entries = (*tcp6_table_ptr).dwNumEntries;

                    for i in 0..num_entries {
                        let tcp6_row = *tcp6_table.offset(i as isize);

                        let connection_state_int = tcp6_row.dwState;
                        match connection_state_int {
                            1 => connection_state = "CLOSED",
                            2 => connection_state = "LISTEN",
                            3 => connection_state = "SYN_SENT",
                            4 => connection_state = "SYN_RCVD",
                            5 => connection_state = "ESTAB",
                            6 => connection_state = "FIN_WAIT1",
                            7 => connection_state = "FIN_WAIT2",
                            8 => connection_state = "CLOSE_WAIT",
                            9 => connection_state = "CLOSING",
                            10 => connection_state = "LAST_ACK",
                            11 => connection_state = "TIME_WAIT",
                            12 => connection_state = "DELETE_TCB",
                            _ => connection_state = "UNKNOWN"
                        };

                        let local_ipv6_bytes: [u8; 16] = tcp6_row.ucLocalAddr;
                        let remote_ipv6_bytes: [u8; 16] = tcp6_row.ucRemoteAddr;

                        &output_table.push(ProcessOpenSocketsRow {
                            pid: tcp6_row.dwOwningPid as i64,
                            fd: 0,  // NA for windows
                            socket: 0,  // NA for windows
                            family: 23, // IPv6
                            protocol: 6,    // TCP
                            local_address: Ipv6Addr::from(local_ipv6_bytes).to_string(),
                            remote_address: Ipv6Addr::from(remote_ipv6_bytes).to_string(),
                            local_port: u16::from_be(tcp6_row.dwLocalPort as u16) as i32,
                            remote_port: u16::from_be(tcp6_row.dwRemotePort as u16) as i32,
                            path: "".to_string(),   // NA for windows
                            state: connection_state.to_string(),
                            net_namespace: "".to_string(),  // NA for windows
                        });
                    };
                }
            }
        },

        SocketType::Udp => {

            let mut buffer: Vec<u8>;
            let udp_return_val: u32;
            let udp_table_ptr: *const MIB_UDPTABLE_OWNER_PID;

            get_extended_udp_table(ptr::null_mut(), table_size_ptr, 1, AF_INET, UDP_TABLE_CLASS::UDP_TABLE_OWNER_PID, 0);

            buffer = Vec::with_capacity(table_size as usize);
            udp_return_val = get_extended_udp_table(buffer.as_mut_ptr(), table_size_ptr, 1, AF_INET, UDP_TABLE_CLASS::UDP_TABLE_OWNER_PID, 0);
            udp_table_ptr = buffer.as_mut_ptr() as *const MIB_UDPTABLE_OWNER_PID;

            if udp_return_val == 0 && table_size > 0 {
                unsafe {
                    let udp_table = &(*udp_table_ptr).table as *const MIB_UDPROW_OWNER_PID;

                    let num_entries = (*udp_table_ptr).dwNumEntries;

                    for i in 0..num_entries {
                        let udp_row = *udp_table.offset(i as isize);
                        &output_table.push(ProcessOpenSocketsRow {
                            pid: udp_row.dwOwningPid as i64,
                            fd: 0,  // NA for windows
                            socket: 0,  // NA for windows
                            family: 2,  // IPv4
                            protocol: 17,   // UDP
                            local_address: Ipv4Addr::from(u32::from_be(udp_row.dwLocalAddr)).to_string(),
                            remote_address: "0".to_string(),
                            local_port: u16::from_be(udp_row.dwLocalPort as u16) as i32,
                            remote_port: 0, // NA for UDP
                            path: "".to_string(),   // NA for windows
                            state: "".to_string(),  // NA for UDP
                            net_namespace: "".to_string(),  // NA for windows
                        });
                    };
                }
            }
        },

        SocketType::Udp6 => {

            let mut buffer: Vec<u8>;
            let udp6_return_val: u32;
            let udp6_table_ptr: *const MIB_UDP6TABLE_OWNER_PID;

            get_extended_udp_table(ptr::null_mut(), table_size_ptr, 1, AF_INET6, UDP_TABLE_CLASS::UDP_TABLE_OWNER_PID, 0);

            buffer = Vec::with_capacity(table_size as usize );
            udp6_return_val = get_extended_udp_table(buffer.as_mut_ptr(), table_size_ptr, 1, AF_INET6, UDP_TABLE_CLASS::UDP_TABLE_OWNER_PID, 0);
            udp6_table_ptr = buffer.as_mut_ptr() as * const MIB_UDP6TABLE_OWNER_PID;

            if udp6_return_val == 0 && table_size > 0 {
                unsafe {
                    let udp6_table = &(*udp6_table_ptr).table as * const MIB_UDP6ROW_OWNER_PID;

                    let num_entries = (*udp6_table_ptr).dwNumEntries;

                    for i in 0..num_entries {
                        let udp_row = *udp6_table.offset(i as isize);
                        let local_ipv6_bytes: [u8; 16] = udp_row.ucLocalAddr;
                        &output_table.push(ProcessOpenSocketsRow {
                            pid: udp_row.dwOwningPid as i64,
                            fd: 0,  // NA for windows
                            socket: 0,  // NA for windows
                            family: 23, // IPv6
                            protocol: 17,   // UDP
                            local_address: Ipv6Addr::from(local_ipv6_bytes).to_string(),
                            remote_address: "0".to_string(),
                            local_port: u16::from_be(udp_row.dwLocalPort as u16) as i32,
                            remote_port: 0, // NA for UDP
                            path: "".to_string(),   // NA for windows
                            state: "".to_string(),  // NA for UDP
                            net_namespace: "".to_string(),  // NA for windows
                        });
                    };
                }
            }
        },
    }
}

enum SocketType {
    Tcp,
    Tcp6,
    Udp,
    Udp6,
}

impl ProcessOpenSocketsRow {
    pub fn get_specific () -> Vec<ProcessOpenSocketsRow>{

        let mut open_sockets_table: Vec<ProcessOpenSocketsRow> = Vec::new();
        parse_socket_table(SocketType::Tcp, &mut open_sockets_table);
        parse_socket_table(SocketType::Tcp6, &mut open_sockets_table);
        parse_socket_table(SocketType::Udp, &mut open_sockets_table);
        parse_socket_table(SocketType::Udp6, &mut open_sockets_table);
        open_sockets_table
    }
}