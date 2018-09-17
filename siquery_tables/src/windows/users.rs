extern crate winreg;

use windows::products::winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

use tables::Users;

impl Users {
    pub(crate) fn new() -> Users {
        Users {
            uid: 0,
            gid: 0,
            uid_signed: 0,
            gid_signed: 0,
            username: String::new(),
            description: String::new(),
            directory: String::new(),
            shell: String::new(),
            uuid: String::new(),
            type_: String::new(),
        }
    }

    pub fn get_specific() -> Vec<Users> {
        let mut users: Vec<Users> = Vec::new();
        let mut user = Users::new();

        users
    }


}

fn process_local_acounts(){

}

fn process_roaming_profiles(){

}

fn get_user_home_dir()->String {
    "".to_string()
}