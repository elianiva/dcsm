use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NetworkConfig {
    pub aliases: Vec<String>,
    pub ipv4_address: String,
    pub ipv6_address: String,
    pub link_local_ips: Vec<String>,
    pub priority: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Network {
    Simple(serde_yaml::Value), // list or dict
    Config(HashMap<String, Option<NetworkConfig>>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_network_config() {
        let network = Network::Config(HashMap::from([
            (
                "default".into(),
                Some(NetworkConfig {
                    aliases: vec!["default".into(), "something".into()],
                    ipv4_address: "127.0.0.1".into(),
                    ipv6_address: "::1".into(),
                    link_local_ips: vec!["::1".into()],
                    priority: 0,
                }),
            ),
            (
                "test".into(),
                Some(NetworkConfig {
                    aliases: vec!["test".into(), "something".into()],
                    ipv4_address: "127.0.0.1".into(),
                    ipv6_address: "::1".into(),
                    link_local_ips: vec!["::1".into()],
                    priority: 1,
                }),
            ),
        ]));

        let serialised = serde_yaml::to_string(&network).unwrap();
        assert_eq!(
            serialised,
            r#"default:
  aliases:
  - default
  ipv4_address: 127.0.0.1
  ipv6_address: ::1
  link_local_ips:
  - ::1
  priority: 0
test:
  aliases:
    - test
  ipv4_address: 127.0.0.1
  ipv6_address: ::1
  link_local_ips:
  - ::1
  priority: 1
  "#
        );
    }
}
