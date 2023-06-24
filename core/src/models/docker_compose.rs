use super::service::Service;

pub struct DockerCompose {
    pub version: String,
    pub name: Option<String>,
    pub services: Vec<Service>,
    pub volumes: Vec<String>,
}
