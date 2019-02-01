use treexml::{Element,Document,XmlVersion::Version10};
use heck::CamelCase;
use rusqlite::{Rows, Row as RusqliteRow, types::{Value, Type}};

use tables::*;

pub fn execute_inventory_query() {
    let mut root = Element::new("InventorySystemInformation");

    get_local_accounts_inv(&mut root);


    let doc = Document {
        root: Some(root),
        version: Version10,
        .. Document::default()
    };

    println!("{}",doc.to_string());
}

pub fn get_local_accounts_inv(ref mut root: &mut Element) {

    let wmi_local_accounts = WmiLocalAccounts::get_specific();

    let mut local_accounts = Element::new("LocalAccounts");

    for local_account in wmi_local_accounts {
        let mut remote_account = Element::new("RemoteAccount");

        let mut child_1 = Element::new("Caption");
        let mut child_2 = Element::new("Domain");
        let mut child_3 = Element::new("LocalAccount");
        let mut child_4 = Element::new("Name");
        let mut child_5 = Element::new("SID");
        let mut child_6 = Element::new("Status");

        child_1.text = Some(local_account.caption);
        child_2.text = Some(local_account._domain);
        child_3.text = Some(local_account.local_account);
        child_4.text = Some(local_account.name);
        child_5.text = Some(local_account.sid);
        child_6.text = Some(local_account.status);

        remote_account.children.push(child_1);
        remote_account.children.push(child_2);
        remote_account.children.push(child_3);
        remote_account.children.push(child_4);
        remote_account.children.push(child_5);
        remote_account.children.push(child_6);

        local_accounts.children.push(remote_account);
    }

    root.children.push(local_accounts);
}

