use proxy_config;
use serde_json;

use tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        if let Ok(proxy_config::ProxyConfig { proxies, whitelist, .. }) = proxy_config::get_proxy_config() {
            for (protocol,url) in proxies {
                out.push(
                    ProxiesRow{
                        proxy:url.to_string(),
                        port:url.port().unwrap_or(0),
                        protocol,
                        interface:"".to_string(),
                        whitelist:serde_json::to_string(&whitelist).ok()
                            .unwrap_or(String::new()),
                    }
                );
            }
        }
        out
    }
}
