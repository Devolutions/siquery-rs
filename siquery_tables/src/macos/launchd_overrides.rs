use glob::glob;
use std::{
    path::{
        Path,
        PathBuf
    },
    fs::File
};
use plist::Plist;

use tables::LaunchdOverridesRow;

impl LaunchdOverridesRow {
    pub fn get_specific() -> Vec<LaunchdOverridesRow> {
        LaunchdOverridesRow::get_specific_ex().unwrap_or(Vec::new())
    }
    pub fn get_specific_ex() -> Result<Vec<LaunchdOverridesRow>, String>  {
        let mut out = Vec::new();
        for entry in glob("/var/db/launchd.db/*/overrides.plist").unwrap() {
            match entry {
                Ok(path) => {
                    if let Some(Plist::Dictionary(overrides)) = File::open(&path).ok()
                        .and_then(|file| Plist::read(file).ok()){
                        for (k,v) in overrides {
                            out.push(LaunchdOverridesRow{
                                path:path.to_string_lossy().to_string(),
                                uid:LaunchdOverridesRow::uid_from_parent_path(&path)?,
                                label:k.as_str().to_string(),
                                key:v.as_string().unwrap_or("").to_string(),
                                value:v.as_boolean()
                                    .expect("Vould not read value from overrides.plist").to_string()
                            });
                        }
                    }
                },
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(out)
    }
    /// Returns Option<uid> from the folder name.
    pub fn uid_from_parent_path(path: &PathBuf) -> Result<i64,String> {
        let uid = Path::new(path).parent()
            .and_then(|parent| parent.file_name())
            .and_then(|file_name| file_name.to_str())
            .and_then(|str| Some(str.split('.').collect::<Vec<_>>()))
            .filter(|vector| vector.len() == 5)
            .and_then(|mut vector| Some(vector.remove(4)));
        uid.map_or(Ok(0), |uid| uid.parse::<i64>())
            .or(Err("Could not parse uid from parent path.".to_string()))
    }
}
