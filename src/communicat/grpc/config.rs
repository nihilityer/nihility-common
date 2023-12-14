use std::net::IpAddr;
use std::str::FromStr;

use local_ip_address::{local_ip, local_ipv6};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

const BIND_PORT: u32 = 5050;

/// Grpc相关配置
#[derive(Deserialize, Serialize, Clone)]
pub struct GrpcConfig {
    pub enable: bool,
    pub addr: IpAddr,
    pub port: u32,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        let ip = match local_ipv6() {
            Ok(ipv6) => ipv6,
            Err(e) => {
                debug!("get ipv6 addr error {:?}, try get ipv4 addr", e);
                match local_ip() {
                    Ok(ipv4) => ipv4,
                    Err(e) => {
                        error!("get ipv4 addr error: {:?}", e);
                        let ip = IpAddr::from_str("127.0.0.1").unwrap();
                        ip
                    }
                }
            }
        };
        GrpcConfig {
            enable: true,
            addr: ip,
            port: BIND_PORT,
        }
    }
}