use super::common::StringOrList;

pub struct Healthcheck {
    pub disable: bool,
    pub interval: String,
    pub retries: u8,
    pub test: StringOrList,
    pub timeout: String,
    pub start_period: String,
}
