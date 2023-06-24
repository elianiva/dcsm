pub struct DockerCompose {
    version: String,
    services: Vec<Service>,
    volumes: Vec<String>,
}
