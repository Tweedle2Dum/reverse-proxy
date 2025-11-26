use crate::config::Config;
use std::net::IpAddr;

pub fn is_allowed(config: &Config, address: IpAddr) -> bool {
    config.access_control_list.contains(&address.to_string())
}
