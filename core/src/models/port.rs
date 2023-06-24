use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Mode {
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "ingress")]
    Ingress,
}

#[derive(Serialize, Deserialize)]
pub struct PortConfig {
    pub mode: Mode,
    pub host_ip: String,
    pub target: u32,
    pub published: serde_yaml::Value,
    pub protocol: Protocol,
}

#[derive(Serialize, Deserialize)]
pub enum Port {
    Number(u8),
    String(String),
}

#[cfg(test)]
mod tests {
    use crate::models::port::{Mode, PortConfig, Protocol};

    #[test]
    fn parse_port_config() {
        let port_config_yaml = r#"mode: host
host_ip: 127.0.0.1
target: 8080
published: 80
protocol: tcp
"#;
        let port_config: PortConfig = serde_yaml::from_str(port_config_yaml).unwrap();
        assert_eq!(port_config.mode, Mode::Host);
        assert_eq!(port_config.host_ip, "127.0.0.1");
        assert_eq!(port_config.target, 8080);
        assert_eq!(port_config.published, 80);
        assert_eq!(port_config.protocol, Protocol::Tcp);
    }

    #[test]
    fn serialize_port_config() {
        let port_config = PortConfig {
            mode: Mode::Host,
            host_ip: "127.0.0.1".to_string(),
            target: 8080,
            published: serde_yaml::Value::Number(80.into()),
            protocol: Protocol::Tcp,
        };
        let serialised_port_config = serde_yaml::to_string(&port_config).unwrap();
        assert_eq!(
            serialised_port_config,
            "mode: host
host_ip: 127.0.0.1
target: 8080
published: 80
protocol: tcp
"
        )
    }
}
