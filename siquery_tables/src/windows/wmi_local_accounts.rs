use tables::WmiLocalAccounts;
use utils;
use windows::SystemReaderInterface;

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

    pub(crate) fn get_specific(system_reader: &SystemReaderInterface) -> Vec<WmiLocalAccounts> {

        let mut local_accounts: Vec<WmiLocalAccounts> = Vec::new();

        if let Some(local_account_info) = system_reader.get_wmi_local_accounts_info() {
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
}