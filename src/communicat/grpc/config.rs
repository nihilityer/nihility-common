use std::net::IpAddr;
use std::str::FromStr;

use local_ip_address::{local_ip, local_ipv6};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

const BIND_PORT: u32 = 5050;
const BIND_IP: &str = "127.0.0.1";
const DEFAULT_NAME: &str = "nihility-submodule";
const DEFAULT_TERMINAL_ADDR: &str = "http://127.0.0.1:5050";
const DEFAULT_SERVER_ADDR: &str = "http://127.0.0.1:1234";

/// Grpc相关配置
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GrpcServerConfig {
    pub bind_ip: IpAddr,
    pub bind_port: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GrpcClientConfig {
    pub terminal_address: String,
    pub submodule_name: String,
    pub server_address: String,
}

impl Default for GrpcServerConfig {
    fn default() -> Self {
        let ip = match local_ipv6() {
            Ok(ipv6) => ipv6,
            Err(e) => {
                debug!("Get Local Ipv6 Addr Error {:?}, Try Get Ipv4 Addr", e);
                match local_ip() {
                    Ok(ipv4) => ipv4,
                    Err(e) => {
                        error!("Get Ipv4 Addr Error: {:?}", e);
                        let ip = IpAddr::from_str(BIND_IP).unwrap();
                        ip
                    }
                }
            }
        };
        GrpcServerConfig {
            bind_ip: ip,
            bind_port: BIND_PORT,
        }
    }
}

impl Default for GrpcClientConfig {
    fn default() -> Self {
        GrpcClientConfig {
            terminal_address: DEFAULT_TERMINAL_ADDR.to_string(),
            submodule_name: DEFAULT_NAME.to_string(),
            server_address: DEFAULT_SERVER_ADDR.to_string(),
        }
    }
}