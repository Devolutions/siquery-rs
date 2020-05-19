use proxy_cfg;

use crate::tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        if let Ok(proxy_cfg::ProxyConfig { proxies, whitelist, .. }) = proxy_cfg::get_proxy_config() {
            for (protocol,url) in proxies {
                out.push(
                    ProxiesRow{
                        url:url.to_string().trim_end_matches('/').to_string(),
                        protocol,
                        host:url.host_str().unwrap_or("").to_string(),
                        port:url.port().unwrap_or(0),
                        interface: "".to_string(),
                        exceptions: whitelist.join(","),
                    }
                );
            }
        }
        out
    }
}
