use proxy_cfg;
use url::Url;

use crate::tables::ProxiesRow;

impl ProxiesRow {
    pub fn get_specific() -> Vec<ProxiesRow> {
        let mut out = Vec::new();
        if let Ok(Some(config)) = proxy_cfg::get_proxy_config() {
            let proxies = &config.proxies;
            let whitelist = config.whitelist.iter().map(|e| e.to_string()).collect::<Vec<String>>();
            for (protocol, proxy_url) in proxies {
                if let Ok(url) = Url::parse(proxy_url) {
                    out.push(
                        ProxiesRow{
                            url: url.to_string().trim_end_matches('/').to_string(),
                            protocol: protocol.to_string(),
                            host: url.host_str().unwrap_or("").to_string(),
                            port: url.port().unwrap_or(0),
                            interface: "".to_string(),
                            exceptions: whitelist.join(","),
                        }
                    );
                }
            }
        }
        out
    }
}