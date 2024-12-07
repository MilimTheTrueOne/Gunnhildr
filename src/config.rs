use figment::{
    providers::{Env, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    /// Ip address that the gunnhildr should bind to
    pub binding_ip: IpAddr,
    /// Port that gunnhildr should listen on
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            binding_ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 6767,
        }
    }
}

/// Parse and merge all config sources
pub fn parse_config() -> Config {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Env::prefixed("HILDR"))
        .extract()
        .expect("Error parsing config")
}
