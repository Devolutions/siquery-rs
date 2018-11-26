use plist::Plist;
use std::{
    fs::File,
};
use serde_json;
use url::Url;
use std::borrow::Borrow;

use tables::ProxiesRow;
use errors::ProxyError;
use errors::ProxyError::*;

trait ProxyReaderIface {
    fn read_plist(&self) -> Result<Plist, ProxyError>;
}

struct Reader {}

impl ProxyReaderIface for Reader {
    fn read_plist(&self) -> Result<Plist, ProxyError> {
        File::open("/Library/Preferences/SystemConfiguration/preferences.plist").ok()
            .and_then(|file| Plist::read(file).ok()).ok_or(OsError)
    }
}

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let reader: Box<ProxyReaderIface> = Box::new(Reader{});
        get_specific_ex(reader.borrow()).unwrap_or(Vec::new())
        // TODO: store log here using wrapped result
    }
}

fn get_specific_ex(reader: &ProxyReaderIface) -> Result<Vec<ProxiesRow>, ProxyError> {

    let plist = reader.read_plist()?;

    if let Some(Plist::Dictionary(network_services)) = plist.as_dictionary()
        .and_then(|decoded_data| decoded_data.get("NetworkServices")) {

        let mut proxies = Vec::new();

        // Extract proxy settings for all network interfaces.
        for (_k,v) in network_services.iter() {

            let proxy = v.as_dictionary().ok_or(InvalidConfigError)?
                .get("Proxies").ok_or(InvalidConfigError)?
                .as_dictionary().ok_or(InvalidConfigError)?;

            for entry in proxy.keys() {
                if entry.contains("Proxy") {
                    // Ex: entry = "HTTPSProxy".
                    let protocol = entry.replace("Proxy","");
                    let scheme;
                    match protocol.as_ref() {
                        "HTTPS" => {
                            scheme = "https"
                        },
                        _ => {
                            scheme = "http"
                        }
                    };
                    if proxy.get(&format!("{}{}",protocol,"Enable"))
                        == Some(&Plist::Integer(1)) {
                        let mut interface = String::new();
                        let mut whitelist = Vec::new();
                        if let Some(Plist::Array(exceptions)) = proxy.get("ExceptionsList") {
                            // Proxy exceptions can be different for different network interfaces on MacOs.
                            if let Some(Plist::String(user_defined_name)) = v
                                .as_dictionary().ok_or(InvalidConfigError)?
                                .get("UserDefinedName"){
                                interface = user_defined_name.to_string();
                                for exception in exceptions {
                                    whitelist.push(exception.as_string().ok_or(InvalidConfigError)?)
                                }
                            }
                        }

                        proxies.push(
                            ProxiesRow {
                                proxy: parse_addr_default_scheme(
                                    scheme,
                                    &format!(
                                        "{}:{}",
                                        proxy.get(entry).ok_or(InvalidConfigError)?.as_string().ok_or(InvalidConfigError)?,
                                        proxy.get(&format!("{}{}", protocol, "Port")).ok_or(InvalidConfigError)?.as_integer().ok_or(InvalidConfigError)?
                                    )
                                )?,
                                port: proxy.get(&format!("{}{}", protocol, "Port")).ok_or(InvalidConfigError)?.as_integer().ok_or(InvalidConfigError)?,
                                protocol: protocol.to_lowercase(),
                                interface,
                                whitelist:serde_json::to_string(&whitelist).ok().unwrap_or(String::new()),
                            }
                        );

                    } else {
                        // Proxy for protocol is not enabled.
                        continue
                    }
                }
            }
        }
        return Ok(proxies);
    }
    Err(NoProxyConfiguredError)
}

fn parse_addr_default_scheme(scheme: &str, addr: &str) -> Result<String, ProxyError> {
    let split: Vec<&str> = addr.split("://").collect();
    if split.len() == 2 {
        Ok(Url::parse(addr)?.to_string())
    } else if split.len() == 1 {
        Ok(Url::parse(&format!("{}://{}", scheme, addr))?.to_string())
    } else {
        Err(InvalidConfigError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct Test {}
    impl ProxyReaderIface for Test {
        fn read_plist(&self) -> Result<Plist, ProxyError> {
            File::open("../siquery_tables/test_data/preferences.plist").ok()
                .and_then(|file| Plist::read(file).ok()).ok_or(OsError)
        }
    }
    #[test]
    fn proxy_configs () {
        let reader: Box<ProxyReaderIface> = Box::new(Test{});
        let proxy_settings = &get_specific_ex(reader.borrow()).unwrap()[0];
        assert_eq!(proxy_settings.proxy, "https://127.0.0.1:50001/");
        assert_eq!(proxy_settings.port, 50001);
        assert_eq!(proxy_settings.protocol, "https");
        assert_eq!(proxy_settings.interface, "Thunderbolt Bridge");
        assert_eq!(proxy_settings.whitelist, r#"["*.local","169.254/16","123.0.0.1/15"]"#);

    }
}