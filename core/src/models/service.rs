use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::healthcheck::Healthcheck;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub image: String,
    pub restart: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
    pub command: String,
    pub healthcheck: Healthcheck,
}

#[test]
fn parse_simple_minio_servive() {
    let minio_config = r#"---
image: quay.io/minio/minio:RELEASE.2022-02-05T04-40-59Z
command: server /data --console-address ":9001"
restart: unless-stopped
ports:
    - 9001:9001
    - 9000:9000
environment:
    MINIO_ROOT_USER: diPj59zJzm2kwUZxcg5QRAUtpbVx5Uxd
    MINIO_ROOT_PASSWORD: xLxBHSp2vAdX2TJSy6EptamrNk5ZXzXo
healthcheck:
    test: "curl -f http://localhost:9000/minio/health/live"
    interval: 30s
    timeout: 10s
    retries: 5
    start_period: 60s
volumes:
    - ./data/minio:/data:z"#;
    let service: Service = serde_yaml::from_str(minio_config).unwrap();
    assert_eq!(
        service.image,
        "quay.io/minio/minio:RELEASE.2022-02-05T04-40-59Z"
    );
    assert_eq!(service.command, "server /data --console-address \":9001\"");
    assert_eq!(service.restart, "unless-stopped");
    assert_eq!(service.ports, vec!["9001:9001", "9000:9000"]);
    assert_eq!(
        service.healthcheck.test,
        "curl -f http://localhost:9000/minio/health/live"
    );
    assert_eq!(service.healthcheck.interval, "30s");
    assert_eq!(service.healthcheck.timeout, "10s");
    assert_eq!(service.healthcheck.retries, 5);
    assert_eq!(service.healthcheck.start_period, "60s");
}
