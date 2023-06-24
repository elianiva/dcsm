use std::collections::HashMap;

use super::common::ListOrDict;

pub struct NetworkConfig {
    pub aliases: Vec<String>,
    pub ipv4_address: String,
    pub ipv6_address: String,
    pub link_local_ips: Vec<String>,
    pub priority: u8,
}

pub enum Network {
    Simple(ListOrDict),
    Config(HashMap<String, Option<NetworkConfig>>),
}
