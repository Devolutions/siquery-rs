use tables::WmiNetworkAdapters;
use utils;
use windows::SystemReaderInterface;

impl WmiNetworkAdapters {
    pub(crate) fn new() -> WmiNetworkAdapters {
        WmiNetworkAdapters {
            caption: String::new(),
            description: String::new(),
            setting_id: String::new(),
            arp_always_source_route: String::new(),
            arp_use_ether_snap: String::new(),
            database_path: String::new(),
            dead_gw_detect_enabled: String::new(),
            default_ip_gateway: Vec::new(),
            default_tos: String::new(),
            default_ttl: String::new(),
            dhcp_enabled: String::new(),
            dhcp_lease_expires: String::new(),
            dhcp_lease_obtained: String::new(),
            dhcp_server: String::new(),
            dns_domain: String::new(),
            dns_domain_suffix_search_order: Vec::new(),
            dns_enabled_for_wins_resolution: String::new(),
            dns_host_name: String::new(),
            dns_server_search_order: Vec::new(),
            domain_dns_registration_enabled: String::new(),
            forward_buffer_memory: String::new(),
            full_dns_registration_enabled: String::new(),
            gateway_cost_metric: Vec::new(),
            igmp_level: String::new(),
            index: String::new(),
            interface_index: String::new(),
            ip_address: Vec::new(),
            ip_connection_metric: String::new(),
            ip_enabled: String::new(),
            ip_filter_security_enabled: String::new(),
            ip_port_security_enabled: String::new(),
            ip_sec_permit_ip_protocols: Vec::new(),
            ip_sec_permit_tcp_ports: Vec::new(),
            ip_sec_permit_udp_ports: Vec::new(),
            ip_subnet: Vec::new(),
            ip_use_zero_broadcast: String::new(),
            ip_x_address: String::new(),
            ip_x_enabled: String::new(),
            ip_x_frame_type: Vec::new(),
            ip_x_media_type: String::new(),
            ip_x_network_number: Vec::new(),
            ip_x_virtual_net_number: String::new(),
            keep_alive_interval: String::new(),
            keep_alive_time: String::new(),
            mac_address: String::new(),
            mtu: String::new(),
            num_forward_packets: String::new(),
            pmtu_bh_detect_enabled: String::new(),
            pmtu_discovery_enabled: String::new(),
            service_name: String::new(),
            tcp_ip_netbios_options: String::new(),
            tcp_max_connect_retransmissions: String::new(),
            tcp_max_data_retransmissions: String::new(),
            tcp_num_connections: String::new(),
            tcp_use_rfc1122_urgent_pointer: String::new(),
            tcp_window_size: String::new(),
            wins_enable_lm_hosts_lookup: String::new(),
            wins_host_lookup_file: String::new(),
            wins_primary_server: String::new(),
            wins_scope_id: String::new(),
            wins_secondary_server: String::new(),
        }
    }

    pub(crate) fn get_netwok_adapters_info(system_reader: &SystemReaderInterface) -> Vec<WmiNetworkAdapters> {

        let mut network_adapters: Vec<WmiNetworkAdapters> = Vec::new();

        if let Some(network_adapter_info) = system_reader.get_wmi_network_adapters_info() {
            let mut network_adapter = WmiNetworkAdapters::new();
            let lines = network_adapter_info.split('\n');

            for line in lines {
                if line.len() <= 2 {
                    if network_adapter.caption != "" {
                        network_adapters.push(network_adapter);
                    }
                    network_adapter = WmiNetworkAdapters::new();
                }
                let v: Vec<_> = line.split('=').collect();
                if v.len() != 2 {
                    continue
                }

                let mut k = String::from(v[0]);
                let mut v = String::from(v[1]);
                utils::trim_string(&mut k);
                utils::trim_string(&mut v);

                match k.as_str() {
                    "Caption" => {
                        network_adapter.caption = v;
                    },
                    "Description" => {
                        network_adapter.description = v;
                    },
                    "SettingID" => {
                        network_adapter.setting_id = v;
                    },
                    "ArpAlwaysSourceRoute" => {
                        network_adapter.arp_always_source_route = v;
                    },
                    "ArpUseEtherSNAP" => {
                        network_adapter.arp_use_ether_snap = v;
                    },
                    "DatabasePath" => {
                        network_adapter.database_path = v;
                    },
                    "DeadGWDetectEnabled" => {
                        network_adapter.dead_gw_detect_enabled = v;
                    },
                    "DefaultIPGateway" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.default_ip_gateway);
                    },
                    "DefaultTOS" => {
                        network_adapter.default_tos = v;
                    },
                    "DefaultTTL" => {
                        network_adapter.default_ttl = v;
                    },
                    "DHCPEnabled" => {
                        network_adapter.dhcp_enabled = v;
                    },
                    "DHCPLeaseExpires" => {
                        network_adapter.dhcp_lease_expires = v;
                    },
                    "DHCPLeaseObtained" => {
                        network_adapter.dhcp_lease_obtained = v;
                    },
                    "DHCPServer" => {
                        network_adapter.dhcp_server = v;
                    },
                    "DNSDomain" => {
                        network_adapter.dns_domain = v;
                    },
                    "DNSDomainSuffixSearchOrder" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.dns_domain_suffix_search_order);
                    },
                    "DNSEnabledForWINSResolution" => {
                        network_adapter.dns_enabled_for_wins_resolution = v;
                    },
                    "DNSHostName" => {
                        network_adapter.dns_host_name = v;
                    },
                    "DNSServerSearchOrder" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.dns_server_search_order);
                    },
                    "DomainDNSRegistrationEnabled" => {
                        network_adapter.domain_dns_registration_enabled = v;
                    },
                    "ForwardBufferMemory" => {
                        network_adapter.forward_buffer_memory = v;
                    },
                    "FullDNSRegistrationEnabled" => {
                        network_adapter.full_dns_registration_enabled = v;
                    },
                    "GatewayCostMetric" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.gateway_cost_metric);
                    },
                    "IGMPLevel" => {
                        network_adapter.igmp_level = v;
                    },
                    "Index" => {
                        network_adapter.index = v;
                    },
                    "InterfaceIndex" => {
                        network_adapter.interface_index = v;
                    },
                    "IPAddress" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_address);
                    },
                    "IPConnectionMetric" => {
                        network_adapter.ip_connection_metric = v;
                    },
                    "IPEnabled" => {
                        network_adapter.ip_enabled = v;
                    },
                    "IPFilterSecurityEnabled" => {
                        network_adapter.ip_filter_security_enabled = v;
                    },
                    "IPPortSecurityEnabled" => {
                        network_adapter.ip_port_security_enabled = v;
                    },
                    "IPSecPermitIPProtocols" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_sec_permit_ip_protocols);
                    },
                    "IPSecPermitTCPPorts" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_sec_permit_tcp_ports);
                    },
                    "IPSecPermitUDPPorts" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_sec_permit_udp_ports);
                    },
                    "IPSubnet" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_subnet);
                    },
                    "IPUseZeroBroadcast" => {
                        network_adapter.ip_use_zero_broadcast = v;
                    },
                    "IPXAddress" => {
                        network_adapter.ip_x_address = v;
                    },
                    "IPXEnabled" => {
                        network_adapter.ip_x_enabled = v;
                    },
                    "IPXFrameType" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_x_frame_type);
                    },
                    "IPXMediaType" => {
                        network_adapter.ip_x_media_type = v;
                    },
                    "IPXNetworkNumber" => {
                        add_formatted_entry(&mut v,  &mut network_adapter.ip_x_network_number);
                    },
                    "IPXVirtualNetNumber" => {
                        network_adapter.ip_x_virtual_net_number = v;
                    },
                    "KeepAliveInterval" => {
                        network_adapter.keep_alive_interval = v;
                    },
                    "KeepAliveTime" => {
                        network_adapter.keep_alive_time = v;
                    },
                    "MACAddress" => {
                        network_adapter.mac_address = v;
                    },
                    "MTU" => {
                        network_adapter.mtu = v;
                    },
                    "NumForwardPackets" => {
                        network_adapter.num_forward_packets = v;
                    },
                    "PMTUBHDetectEnabled" => {
                        network_adapter.pmtu_bh_detect_enabled = v;
                    },
                    "PMTUDiscoveryEnabled" => {
                        network_adapter.pmtu_discovery_enabled = v;
                    },
                    "ServiceName" => {
                        network_adapter.service_name = v;
                    },
                    "TcpipNetbiosOptions" => {
                        network_adapter.tcp_ip_netbios_options = v;
                    },
                    "TcpMaxConnectRetransmissions" => {
                        network_adapter.tcp_max_connect_retransmissions = v;
                    },
                    "TcpMaxDataRetransmissions" => {
                        network_adapter.tcp_max_data_retransmissions = v;
                    },
                    "TcpNumConnections" => {
                        network_adapter.tcp_num_connections = v;
                    },
                    "TcpUseRFC1122UrgentPointer" => {
                        network_adapter.tcp_use_rfc1122_urgent_pointer = v;
                    },
                    "TcpWindowSize" => {
                        network_adapter.tcp_window_size = v;
                    },
                    "WINSEnableLMHostsLookup" => {
                        network_adapter.wins_enable_lm_hosts_lookup = v;
                    },
                    "WINSHostLookupFile" => {
                        network_adapter.wins_host_lookup_file = v;
                    },
                    "WINSPrimaryServer" => {
                        network_adapter.wins_primary_server = v;
                    },
                    "WINSScopeID" => {
                        network_adapter.wins_scope_id = v;
                    },
                    "WINSSecondaryServer" => {
                        network_adapter.wins_secondary_server = v;
                    },
                    _ => ()
                }
            }
        }

        network_adapters
    }
}

fn add_formatted_entry(s: &mut String, v: &mut Vec<String>){
    s.retain(|c| c != '\"');
    s.retain(|c| c != '{');
    s.retain(|c| c != '}');

    let p: Vec<_> = s.split(',').collect();

    for x in 0..p.len() {
        v.push(String::from(p[x]));
    }
}