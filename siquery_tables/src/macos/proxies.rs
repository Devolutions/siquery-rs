use proxy_config::macos::{get_proxy_config_ex};

use tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        let proxies = get_proxy_config_ex().ok().unwrap_or(Vec::new());
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