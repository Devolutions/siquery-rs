use proxy_config::macos::{get_proxy_config_ex,Reader,ProxyConfigReader};
use std::borrow::Borrow;

use tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        let reader: Box<ProxyConfigReader> = Box::new(Reader{});

        let proxies = get_proxy_config_ex(reader.borrow()).ok().unwrap_or(Vec::new());
        for entry in proxies {
            out.push(
                ProxiesRow{
                    proxy : entry.proxy.to_string(),
                    port : entry.port,
                    protocol : entry.protocol,
                    interface : entry.interface,
                    whitelist : entry.whitelist,
                }
            )
        }
        out
    }
}