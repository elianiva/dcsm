use serde::{Deserialize, Serialize};

use super::service::Service;

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerCompose {
    pub version: f32,
    pub name: Option<String>,
    pub services: Vec<Service>,
    pub volumes: Vec<String>,
}

#[test]
fn parse_simple_postgre_config() {
    let config = r#"version: 3.7
services:
    db:
        image: mysql:8
        restart: unless-stopped
        volumes:
            - ./data/mysql:/var/lib/mysql:z
        ports:
            - 3306:3306
        environment:
            MYSQL_DATABASE: deeznuts
            MYSQL_ROOT_PASSWORD: password
    minio:
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
    let docker_compose: DockerCompose = serde_yaml::from_str(config).unwrap();
    assert_eq!(docker_compose.version, 3.7);
    assert_eq!(docker_compose.name, None);
    assert_eq!(docker_compose.services.len(), 2);
    assert_eq!(docker_compose.volumes.len(), 2);
    assert_eq!(docker_compose.services[0].image, "mysql:8");
    assert_eq!(docker_compose.services[0].restart, "unless-stopped");
    assert_eq!(
        docker_compose.services[0].volumes,
        vec!["./data/mysql:/var/lib/mysql:z"]
    );
    assert_eq!(docker_compose.services[0].ports, vec!["3306:3306"]);
    assert_eq!(
        docker_compose.services[0].environment.get("MYSQL_DATABASE"),
        Some(&"deeznuts".to_string())
    );
    assert_eq!(
        docker_compose.services[0]
            .environment
            .get("MYSQL_ROOT_PASSWORD"),
        Some(&"password".to_string())
    );
    assert_eq!(
        docker_compose.services[1].image,
        "quay.io/minio/minio:RELEASE.2022-02-05T04-40-59Z"
    );
    assert_eq!(
        docker_compose.services[1].command,
        "server /data --console-address \":9001\""
    );
    assert_eq!(docker_compose.services[1].restart, "unless-stopped");
    assert_eq!(
        docker_compose.services[1].ports,
        vec!["9001:9001", "9000:9000"]
    );
    assert_eq!(docker_compose.volumes[0], "./data/mysql:/var/lib/mysql:z");
}
