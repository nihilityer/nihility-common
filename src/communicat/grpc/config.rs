use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

use local_ip_address::{local_ip, local_ipv6};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::error::NihilityCommonError;

const BIND_PORT: u32 = 5050;
const BIND_IP: &str = "127.0.0.1";
const DEFAULT_NAME: &str = "nihility-submodule";
const DEFAULT_TERMINAL_ADDR: &str = "http://127.0.0.1:5050";

const SERVER_ADDR_FIELD: &str = "server_addr";
const SUBMODULE_NAME_FIELD: &str = "submodule_name";

/// Grpc相关配置
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GrpcServerConfig {
    pub bind_ip: IpAddr,
    pub bind_port: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GrpcClientConfig {
    pub server_address: String,
    pub submodule_name: String,
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
                        IpAddr::from_str(BIND_IP).unwrap()
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
            server_address: DEFAULT_TERMINAL_ADDR.to_string(),
            submodule_name: DEFAULT_NAME.to_string(),
        }
    }
}

impl GrpcServerConfig {
    pub fn create_connection_params(&self, submodule_name: &String) -> HashMap<String, String> {
        let mut result = HashMap::<String, String>::new();
        result.insert(SUBMODULE_NAME_FIELD.to_string(), submodule_name.to_string());
        let server_addr = match self.bind_ip {
            IpAddr::V4(ip) => format!("http://{}:{}", ip, self.bind_port),
            IpAddr::V6(ip) => format!("http://[{}]:{}", ip, self.bind_port),
        };
        result.insert(SERVER_ADDR_FIELD.to_string(), server_addr);
        result
    }
}

impl TryFrom<HashMap<String, String>> for GrpcClientConfig {
    type Error = NihilityCommonError;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        if let (Some(server_address), Some(submodule_name)) = (
            value.get(SERVER_ADDR_FIELD),
            value.get(SUBMODULE_NAME_FIELD),
        ) {
            return Ok(GrpcClientConfig {
                server_address: server_address.to_string(),
                submodule_name: submodule_name.to_string(),
            });
        }
        Err(NihilityCommonError::ConfigFieldMissing)
    }
}
