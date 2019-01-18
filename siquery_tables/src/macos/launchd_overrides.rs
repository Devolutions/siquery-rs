use glob::{
    glob,
};
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
        for entry in glob("/var/db/launchd.db/*/overrides.plist")
            // Return any pattern parsing errors.
            .map_err(|e| e.to_string())? {
            match entry {
                Ok(path) => {
                    if let Some(Plist::Dictionary(overrides)) = File::open(&path).ok()
                        .and_then(|file| Plist::read(file).ok()){
                        /* Example:
                        <key>com.apple.AppleFileServer</key>    -> label
                        <dict>
                            <key>Disabled</key>                 -> key
                            <true/>                             -> val
                        </dict>
                        */
                        for (label,bool_key_value) in overrides {
                            for (key,value) in bool_key_value.as_dictionary()
                                .ok_or("Could not read key_value pair from overrides.plist")? {
                                out.push(LaunchdOverridesRow{
                                    path:path.to_string_lossy().to_string(),
                                    uid:LaunchdOverridesRow::uid_from_parent_path(&path)?,
                                    label:label.to_string(),
                                    key:key.to_string(),
                                    value:value.as_boolean()
                                        .ok_or("Could not read value from overrides.plist")?
                                        .to_string()
                                });
                            }
                        }
                    }
                },
                // Report any IoErrors.
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(out)
    }
    /// Returns Option<uid> from the parent path name.
    pub fn uid_from_parent_path(path: &PathBuf) -> Result<i64,String> {
        // Extract Some<uid> if appended to the path name.
        let uid = Path::new(path).parent()
            .and_then(|parent| parent.file_name())
            .and_then(|file_name| file_name.to_str())
            .and_then(|str| Some(str.split('.').collect::<Vec<_>>()))
            .filter(|vector| vector.len() == 5)
            .and_then(|mut vector| Some(vector.remove(4)));
        // Parse Some<uid> as i64 or return 0 if uid = None.
        uid.map_or(Ok(0), |uid| uid.parse::<i64>())
            // Return Err() if parse::<i64>() failed.
            .or(Err("Could not parse uid from parent path.".to_string()))
    }
}
