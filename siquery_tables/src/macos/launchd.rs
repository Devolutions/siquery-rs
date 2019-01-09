use walkdir::{WalkDir,DirEntry,Error};
use std::{path::Path,fs::File};
use plist::Plist;

use tables::{LaunchdRow,Users};

const LAUNCHD_SEARCH_PATHS: [&'static str;4] = [
    "/System/Library/LaunchDaemons",
    "/Library/LaunchDaemons",
    "/System/Library/LaunchAgents",
    "/Library/LaunchAgents"
];

impl LaunchdRow {

    pub fn new() -> LaunchdRow {
        LaunchdRow {
            path : String::new(),
            name : String::new(),
            label : String::new(),
            program : String::new(),
            run_at_load : String::new(),
            keep_alive : String::new(),
            on_demand : String::new(),
            disabled : String::new(),
            username : String::new(),
            groupname : String::new(),
            stdout_path : String::new(),
            stderr_path : String::new(),
            start_interval : String::new(),
            program_arguments : String::new(),
            watch_paths : String::new(),
            queue_directories : String::new(),
            inetd_compatibility : String::new(),
            start_on_mount : String::new(),
            root_directory : String::new(),
            working_directory : String::new(),
            process_type : String::new(),
        }
    }

    pub fn gen_launchd_row(dir_entry:Result<DirEntry,Error>) -> Result<LaunchdRow, String> {
        let mut row = LaunchdRow::new();

        let path = dir_entry.map_err(|_e| "Error occurred trying to read plist.")?;

        row.path = path.path()
            .to_str().ok_or("Could not get path string.")?
            .to_owned();
        row.name = path.path()
            .file_name().ok_or("Error occurred trying to get filename.")?
            .to_str().ok_or("Could not get filename string.")?
            .to_owned();

        if let Some(Plist::Dictionary(dictionary)) = File::open(&row.path).ok()
            .and_then(|file| Plist::read(file).ok()){
            for (k,v) in dictionary.iter() {
                if let Some(value) = v.as_string(){
                    let string_value = value.to_string();
                    match k.as_str() {
                        "Label" => { row.label = string_value }
                        "StandardOutPath" => { row.stdout_path = string_value }
                        "StandardErrorPath" => { row.stderr_path = string_value }
                        "inetdCompatibility" => { row.inetd_compatibility = string_value }
                        "Program" => { row.program = string_value }
                        "UserName" => { row.username = string_value }
                        "GroupName" => { row.groupname = string_value }
                        "RootDirectory" => { row.root_directory = string_value }
                        "WorkingDirectory" => { row.working_directory = string_value }
                        "ProcessType" => { row.process_type = string_value }
                        &_ => { continue }
                    }
                } else if let Some(array_value) = v.as_array() {
                    let string_value = array_value.iter()
                        .map(|arr_element| arr_element.as_string().unwrap_or(""))
                        .collect::<Vec<&str>>()
                        .join(" ");
                    match k.as_str() {
                        "ProgramArguments" => { row.program_arguments = string_value }
                        "WatchPaths" => { row.watch_paths = string_value }
                        "QueueDirectories" => { row.queue_directories = string_value }
                        &_ => { continue }
                    }
                } else if let Some(bool_value) = v.as_boolean() {
                    let string = match bool_value.to_string().as_str() {
                        "true" => { "1".to_string() }
                        "false" => { "0".to_string() }
                        &_ => { "".to_string() }
                    };
                    match k.as_str() {
                        "RunAtLoad" => { row.run_at_load = string }
                        "KeepAlive" => { row.keep_alive = string }
                        "OnDemand" => { row.on_demand = string }
                        "Disabled" => { row.disabled = string }
                        "StartOnMount" => { row.start_on_mount = string }
                        &_ => { continue }
                    }
                } else if let Some(number_value) = v.as_integer() {
                    match k.as_str() {
                        "StartInterval" => { row.start_interval = number_value.to_string() }
                        &_ => { continue }
                    }
                }
            }
            return Ok(row)
        }
        Err("Not a plist.".to_string())
    }

    pub fn get_specific() -> Vec<LaunchdRow>{
        let mut out: Vec<LaunchdRow> = Vec::new();

        for search_path in LAUNCHD_SEARCH_PATHS.iter() {
            for entry in WalkDir::new(search_path) {
                if let Ok (row) = LaunchdRow::gen_launchd_row(entry) {
                    out.push(row);
                }
            }
        }
        for user in Users::get_specific().iter(){
            for entry in WalkDir::new(
                Path::new(&format!("{}/Library/LaunchAgents",user.directory))
            ) {
                if let Ok (row) = LaunchdRow::gen_launchd_row(entry) {
                    out.push(row);
                }
            }
        }
        out
    }
}
