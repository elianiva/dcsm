use super::common::StringOrNumber;

pub struct PortConfig {
    pub mode: String,
    pub host_ip: String,
    pub target: u8,
    pub published: StringOrNumber<i32>,
    pub protocol: String,
}

pub enum Port {
    Number(u8),
    String(String),
}
