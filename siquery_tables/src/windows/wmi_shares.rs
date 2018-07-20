use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiShares,WmiSharesIface};
use utils;

pub struct Reader {}
impl WmiSharesIface for Reader {
    fn get_wmi_shares_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["share",
                "get",
                "Caption,Description,Name,Path,Status,Type,AllowMaximum",
                "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiShares {
    pub(crate) fn new() -> WmiShares {
        WmiShares {
            caption: String::new(),
            description: String::new(),
            name: String::new(),
            path: String::new(),
            status: String::new(),
            _type: String::new(),
            allow_maximum: String::new(),
        }
    }

    pub(crate) fn get_specific_ex(reader: &WmiSharesIface) -> Vec<WmiShares> {

        let mut shares: Vec<WmiShares> = Vec::new();

        if let Some(share_info) = reader.get_wmi_shares_info() {
            let mut share = WmiShares::new();
            let lines = share_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if share.allow_maximum != "" {
                        shares.push(share);
                    }
                    share = WmiShares::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Caption" => {
                        share.caption = v;
                    },
                    "Description" => {
                        share.description = v;
                    },
                    "Name" => {
                        share.name = v;
                    },
                    "Path" => {
                        share.path = v;
                    },
                    "Status" => {
                        share.status = v;
                    },
                    //https://msdn.microsoft.com/en-us/library/aa394435(v=vs.85).aspx
                    "Type" => {
                        match v.as_str(){
                            "0"=> {
                                share._type = "Disk Drive".to_string();
                            },
                            "1"=> {
                                share._type = "Print Queue".to_string();
                            },
                            "2"=> {
                                share._type = "Device".to_string();
                            },
                            "3"=> {
                                share._type = "IPC".to_string();
                            },
                            "2147483648"=> {
                                share._type = "Disk Drive Admin".to_string();
                            },
                            "2147483649"=> {
                                share._type = "Print Queue Admin".to_string();
                            },
                            "2147483650"=> {
                                share._type = "Device Admin".to_string();
                            },
                            "2147483651"=> {
                                share._type = "IPC Admin".to_string();
                            },
                            _=>()
                        }
                    },
                    "AllowMaximum" => {
                        share.allow_maximum = v;
                    },
                    _ => ()
                }
            }
        }

        shares
    }

    pub(crate) fn get_specific() -> Vec<WmiShares> {
        let reader: Box<WmiSharesIface> = Box::new(Reader{});
        let out = WmiShares::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiSharesIface for Test {
        fn get_wmi_shares_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-shares.txt")))
        }
    }
    #[test]
    fn test_wmi_shares () {
        let reader: Box<WmiSharesIface> = Box::new(Test{});
        let test_shares = &WmiShares::get_specific_ex(reader.borrow())[0];
        assert_eq!(test_shares.name, "print$");
        assert_eq!(test_shares.caption, "Printer Drivers");
        assert_eq!(test_shares.description, "Printer Drivers");
        assert_eq!(test_shares.path, "C:\\WINDOWS\\system32\\spool\\drivers");
        assert_eq!(test_shares.status, "OK");
        assert_eq!(test_shares._type, "Device Admin");
        assert_eq!(test_shares.allow_maximum, "TRUE");
    }
}