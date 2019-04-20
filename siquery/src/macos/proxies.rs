use proxy_cfg::macos::{get_proxy_config_ex,Reader,ProxyConfigReader};
use std::borrow::Borrow;

use crate::tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        let reader: Box<ProxyConfigReader> = Box::new(Reader{});

        let proxies = get_proxy_config_ex(reader.borrow()).ok().unwrap_or(Vec::new());
        for entry in proxies {
            out.push(
                ProxiesRow{
                    url : entry.url.to_string().trim_end_matches('/').to_string(),
                    protocol : entry.url.scheme().to_string(),
                    host : entry.url.host_str().unwrap_or("").to_string(),
                    port : entry.url.port().unwrap_or(0),
                    interface : entry.interface,
                    exceptions : entry.whitelist,
                }
            )
        }
        out
    }
}