use plist::Plist;
use std::{
    fs::File,
    io::Read,
    borrow::Borrow,
    collections::HashMap,
    hash::BuildHasherDefault
};
use fnv::FnvHasher;

use tables::{ProxiesRow,ProxiesIface};

pub struct Reader {}
impl ProxiesIface for Reader {
    fn get_proxies_file(&self) -> Option<String> {
        let mut s = String::new();
        File::open("/Library/Preferences/SystemConfiguration/preferences.plist").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }
}

impl ProxiesRow {
    pub(crate) fn get_specific_ex(reader: &ProxiesIface) -> Vec<ProxiesRow> {
        let mut out : Vec<ProxiesRow> = Vec::new();

        if let Some(Plist::Dict(dict)) = reader.get_proxies_file()
            .and_then(|s| Plist::from_xml_reader(&mut s.as_bytes()).ok()){
            if let Some(Plist::Dict(network_services)) = dict.get("NetworkServices") {
                for (interface_key,_v) in network_services.iter() {
                    if let Some(Plist::Dict(interface)) = network_services.get(interface_key) {
                        if let Some(Plist::Dict(proxy)) = interface.get("Proxies") {
                            if proxy.get("HTTPEnable") == Some(&Plist::Integer(1)) {
                                out.push(
                                    get_proxy_info("HTTP".to_string(),proxy,interface)
                                )
                            } else if proxy.get("HTTPSEnable") == Some(&Plist::Integer(1)) {
                                out.push(
                                    get_proxy_info("HTTPS".to_string(),proxy,interface)
                                )
                            } else if proxy.get("SOCKSEnable") == Some(&Plist::Integer(1)) {
                                out.push(
                                    get_proxy_info("SOCKS".to_string(),proxy,interface)
                                )
                            }
                        }
                    }
                }
            }
        }
        out
    }
    pub(crate) fn get_specific() -> Vec<ProxiesRow> {
        let reader: Box<ProxiesIface> = Box::new(Reader{});
        let out = ProxiesRow::get_specific_ex(reader.borrow());
        out
    }
}

pub fn get_string(s:Option<&Plist>) -> Option<String> {
    match s {
        Some(Plist::String(v)) => Some(v.to_owned()),
        _ => None,
    }
}

pub fn get_int(i:Option<&Plist>) -> Option<i64> {
    match i {
        Some(Plist::Integer(v)) => Some(*v),
        _ => None,
    }
}

pub fn get_proxy_info(
    protocol:String,
    proxy:&HashMap<String,Plist,BuildHasherDefault<FnvHasher>>,
    interface:&HashMap<String,Plist,BuildHasherDefault<FnvHasher>>
) -> ProxiesRow{
    ProxiesRow {
        address: get_string(proxy.get(&(protocol.clone()+"Proxy"))).unwrap_or("".to_string()),
        port: get_int(proxy.get(&(protocol.clone()+"Port"))).unwrap_or(-1),
        protocol,
        interface: get_string(interface.get("UserDefinedName")).unwrap_or("".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl ProxiesIface for Test {
        fn get_proxies_file(&self) -> Option<String> {
            Some(String::from(include_str!("../../test_data/preferences.plist")))
        }
    }
    #[test]
    fn get_proxies_file () {
        let reader: Box<ProxiesIface> = Box::new(Test{});
        // TODO
        let http_proxy = &ProxiesRow::get_specific_ex(reader.borrow())[0];
        assert_eq!(http_proxy.address, "111.111.111");
        assert_eq!(http_proxy.port, 1111);
        assert_eq!(http_proxy.protocol, "HTTP");
        assert_eq!(http_proxy.interface, "USB 10/100/1000 LAN");
        let https_proxy = &ProxiesRow::get_specific_ex(reader.borrow())[1];
        assert_eq!(https_proxy.address, "222.222.222");
        assert_eq!(https_proxy.port, 2222);
        assert_eq!(https_proxy.protocol, "HTTPS");
        assert_eq!(https_proxy.interface, "Wi-Fi");
        let socks_proxy = &ProxiesRow::get_specific_ex(reader.borrow())[2];
        assert_eq!(socks_proxy.address, "333.333.333");
        assert_eq!(socks_proxy.port, 3333);
        assert_eq!(socks_proxy.protocol, "SOCKS");
        assert_eq!(socks_proxy.interface, "Thunderbolt Bridge");

        assert_eq!(ProxiesRow::get_specific_ex(reader.borrow()).len(), 3);
    }
}