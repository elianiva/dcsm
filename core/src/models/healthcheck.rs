use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Healthcheck {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    pub interval: String,
    pub retries: u8,
    pub test: serde_yaml::Value, // string or list
    pub timeout: String,
    pub start_period: String,
}

#[test]
fn serialise_healthcheck_config() {
    let healthcheck = Healthcheck {
        disable: Some(false),
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
    assert_eq!(healthcheck.disable, Some(true));
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

#[test]
fn parse_optional_field() {
    let healthcheck_yaml = r#"interval: 10s
retries: 5
test: "curl -f http://localhost:9000/minio/health/live"
timeout: 10s
start_period: 10s
"#;
    let healthcheck: Healthcheck = serde_yaml::from_str(healthcheck_yaml).unwrap();
    assert_eq!(healthcheck.disable, None);
    assert_eq!(healthcheck.interval, "10s");
    assert_eq!(healthcheck.retries, 5);
    assert_eq!(
        healthcheck.test,
        serde_yaml::Value::String("curl -f http://localhost:9000/minio/health/live".into())
    );
    assert_eq!(healthcheck.timeout, "10s");
    assert_eq!(healthcheck.start_period, "10s");
}
