use crate::config::Config;
use std::net::IpAddr;

pub fn is_allowed(config: &Config, address: IpAddr) -> bool {
    config.ACCESS_CONTROL_LIST.contains(&address.to_string())
}
