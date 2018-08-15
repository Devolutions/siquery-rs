# Siquery

## Introduction

A rust library for system information analytics and monitoring.

Siquery provides utilities to explore low level operating system data on mac, windows and linux in an efficient and intuitive way by integrating an SQLite powered interface. 
SQLite read-only statements can be used to create custom queries to be outputted into a formatted table, JSON or CSV.

## Usage

```
USAGE: 
       siquery.exe [FLAGS] [OPTIONS] [input]

FLAGS:
             --csv        Sets 'csv' output mode
         -h, --help       Prints help information
             --json       Sets 'json' output mode
             --L          Lists all table names
             --pretty     Sets 'print_pretty' output mode
         -V, --version    Prints version information
         -v               Sets the level of verbosity

OPTIONS:
             --schema <schema>     Prints schema of the given table name
         -q, --query <siquery>     Sqlite command
         -a, --A <table>           Selects all from table

ARGS:
         <input>     Sqlite query command
```
## Examples
```
'SELECT DISTINCT process.name, env.key, process.pid FROM processes AS process JOIN process_envs AS env ON process.pid = env.pid where process.name = 'siquery' AND process.pid > 38000 LIMIT 3' 
+---------+----------------+-------+
| name    | key            | pid   |
+=========+================+=======+
| siquery | CARGO_PKG_NAME | 38798 |
+---------+----------------+-------+
| siquery | SHLVL          | 38798 |
+---------+----------------+-------+
| siquery | PATH.          | 38798 |
+---------+----------------+-------+
```
```
'SELECT DISTINCT process.name, process.pid FROM process_memory_map AS memory JOIN processes as process ON process.path = memory.path where process.pid < 7900 limit 1`

+-----------------+------+
| name            | pid  |
+=================+======+
| dptf_helper.exe | 7800 |
+-----------------+------+
```
```
'Pragma table_info(process_memory_map)'

+-----+-------------+---------+---------+------------+
| cid | name        | type    | notnull | dflt_value |
+=====+=============+=========+=========+============+
| 0   | pid         | INTEGER | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 1   | start       | TEXT    | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 2   | end         | TEXT    | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 3   | permissions | TEXT    | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 4   | offset      | INTEGER | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 5   | device      | TEXT    | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 6   | inode       | INTEGER | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 7   | path        | TEXT    | 0       | 0          |
+-----+-------------+---------+---------+------------+
| 8   | pseudo      | INTEGER | 0       | 0          |
+-----+-------------+---------+---------+------------+
```
```
cargo run -- --pretty -q "SELECT * FROM etc_hosts LIMIT 1"

+-----------+-----------+
| address   | hostnames |
+===========+===========+
| 127.0.0.1 | localhost |
+-----------+-----------+
```
```
cargo run -- --json -q "SELECT * FROM etc_hosts"

[
  {"address":"127.0.0.1","hostnames":"localhost"},
  {"address":"255.255.255.255","hostnames":"broadcasthost"},
  {"address":"::1","hostnames":"localhost"}
]
```
```
cargo run -- --csv -q "SELECT * FROM etc_hosts LIMIT 1"

address|hostnames
127.0.0.1|localhost
255.255.255.255|broadcasthost
::1|localhost
```

## Implemented tables 

Table name | Windows | Linux | MacOS
--- | :---: | :---: | :---: |
etc_hosts | ✔ | ✔ | ✔
etc_protocols | ✔ | ✔ | ✔
etc_services | ✔ | ✔ | ✔
interface_address | ✔ |  | 
interface_details | ✔ |  | 
system_info | ✔ | ✔ | ✔
os_version | ✔ | ✔ | ✔
logical_drives | ✔ |  | 
uptime | ✔ | ✔ | ✔
processes | ✔ | ✔ | ✔
process_open_sockets | ✔ | ✔ | ✔
process_memory_map | ✔ | ✔ | ✔
products | ✔ |  | 
process_envs |  | ✔ | ✔
wmi_computer_info | ✔ |  | 
wmi_os_version  | ✔ |  |
wmi_printers | ✔ |  |
wmi_services | ✔ |  |
wmi_hotfixes | ✔ |  |
wmi_shares | ✔ |  |
wmi_network_adapters | ✔ |  |
wmi_local_accounts | ✔ |  |
wmi_bios | ✔ |  |
wmi_motherboard | ✔ |  |
wmi_processor | ✔ |  |
wmi_physical_memory | ✔ |  |
wmi_sound | ✔ |  |
wmi_video | ✔ |  |
wmi_monitors | ✔ |  |
wmi_keyboard | ✔ |  |
wmi_pointing_device | ✔ |  |





