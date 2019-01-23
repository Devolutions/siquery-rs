use proxy_config;

use tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        if let Ok(proxy_config::ProxyConfig { proxies, whitelist, .. }) = proxy_config::get_proxy_config() {
            for (protocol,url) in proxies {
                out.push(
                    ProxiesRow{
                        url:url.to_string().trim_right_matches('/').to_string(),
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