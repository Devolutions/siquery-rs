use std::process::Command;
use std::borrow::Borrow;

use tables::{WmiLocalAccounts,WmiLocalAccountsIface};
use utils;

pub struct Reader {}
impl WmiLocalAccountsIface for Reader {
    fn get_wmi_local_accounts_info(&self) -> Option<String> {
        let output = Command::new("wmic")
            .args(&["useraccount", "get", "/format:list"]).output().ok()?;
        String::from_utf8(output.stdout).ok()
    }
}

impl WmiLocalAccounts {
    pub(crate) fn new() -> WmiLocalAccounts {
        WmiLocalAccounts {
            account_type: String::new(),
            caption: String::new(),
            description: String::new(),
            _domain: String::new(),
            local_account: String::new(),
            name: String::new(),
            sid: String::new(),
            sid_type: String::new(),
            status: String::new(),
        }
    }

    pub(crate) fn get_specific_ex(reader: &WmiLocalAccountsIface) -> Vec<WmiLocalAccounts> {

        let mut local_accounts: Vec<WmiLocalAccounts> = Vec::new();

        if let Some(local_account_info) = reader.get_wmi_local_accounts_info() {
            let mut local_account = WmiLocalAccounts::new();
            let lines = local_account_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if local_account.name != "" {
                        local_accounts.push(local_account);
                    }
                    local_account = WmiLocalAccounts::new();
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
                    "AccountType" => {

                        match v.as_str(){
                            "256" => {
                                local_account.account_type = "Temporary duplicate account".to_string();
                            },
                            "512" => {
                                local_account.account_type = "Normal account".to_string();
                            },
                            "2048" => {
                                local_account.account_type = "Interdomain trust account".to_string();
                            },
                            "4096" => {
                                local_account.account_type = "Workstation trust account".to_string();
                            },
                            "8192" => {
                                local_account.account_type = "Server trust account".to_string();
                            },
                            _ => ()
                        }
                    },
                    "Caption" => {
                        local_account.caption = v;
                    },
                    "Description" => {
                        local_account.description = v;
                    },
                    "Domain" => {
                        local_account._domain = v;
                    },
                    "LocalAccount" => {
                        local_account.local_account = v;
                    },
                    "Name" => {
                        local_account.name = v;
                    },
                    "SID" => {
                        local_account.sid = v;
                    },
                    "SIDType" => {
                        local_account.sid_type = v;
                    },
                    "Status" => {
                        local_account.status = v;
                    },
                    _ => ()
                }
            }
        }

        local_accounts
    }

    pub(crate) fn get_specific() -> Vec<WmiLocalAccounts> {
        let reader: Box<WmiLocalAccountsIface> = Box::new(Reader{});
        let out = WmiLocalAccounts::get_specific_ex(reader.borrow());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl WmiLocalAccountsIface for Test {
        fn get_wmi_local_accounts_info(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/wmi-local-accounts.txt")))
        }
    }
    #[test]
    fn test_wmi_local_account () {
        let reader: Box<WmiLocalAccountsIface> = Box::new(Test{});
        let wmi_local_accounts = &WmiLocalAccounts::get_specific_ex(reader.borrow())[0];
        assert_eq!(wmi_local_accounts.account_type, "Server trust account");
        assert_eq!(wmi_local_accounts.caption, "bipbip\\Acc");
        assert_eq!(wmi_local_accounts.description, "A server account");
        assert_eq!(wmi_local_accounts._domain, "bipbip1010");
        assert_eq!(wmi_local_accounts.local_account, "TRUE");
        assert_eq!(wmi_local_accounts.name, "UtilityAccount");
        assert_eq!(wmi_local_accounts.sid, "S-0-0-11-1111111111-111111111-111111111-111");
        assert_eq!(wmi_local_accounts.sid_type, "1");
        assert_eq!(wmi_local_accounts.status, "Degraded");
        assert_eq!(WmiLocalAccounts::get_specific_ex(reader.borrow()).len(), 2);
    }
}