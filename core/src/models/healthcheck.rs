use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Healthcheck {
    pub disable: bool,
    pub interval: String,
    pub retries: u8,
    pub test: serde_yaml::Value, // string or list
    pub timeout: String,
    pub start_period: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_healthcheck_config() {
        let healthcheck = Healthcheck {
            disable: false,
            interval: "10s".to_string(),
            retries: 5,
            test: serde_yaml::Value::Sequence(vec![
                "CMD".into(),
                "curl".into(),
                "-f".into(),
                "http://localhost".into(),
            ]),
            timeout: "10s".to_string(),
            start_period: "10s".to_string(),
        };
        let serialised = serde_yaml::to_string(&healthcheck).unwrap();
        assert_eq!(
            serialised,
            r#"disable: false
interval: 10s
retries: 5
test:
- CMD
- curl
- -f
- http://localhost
timeout: 10s
start_period: 10s
"#
        )
    }

    #[test]
    fn parse_healthcheck_config() {
        let healthcheck_yaml = r#"disable: true
interval: 10s
retries: 5
test:
- CMD
- curl
- -f
- http://localhost
timeout: 10s
start_period: 10s
"#;
        let healthcheck: Healthcheck = serde_yaml::from_str(healthcheck_yaml).unwrap();
        assert_eq!(healthcheck.disable, true);
        assert_eq!(healthcheck.interval, "10s");
        assert_eq!(healthcheck.retries, 5);
        assert_eq!(
            healthcheck.test,
            serde_yaml::Value::Sequence(vec![
                "CMD".into(),
                "curl".into(),
                "-f".into(),
                "http://localhost".into(),
            ])
        );
        assert_eq!(healthcheck.timeout, "10s");
        assert_eq!(healthcheck.start_period, "10s");
    }
}
