use std::collections::HashMap;

use super::{
    build::Build,
    common::{ListOrDict, StringOrList},
    deployment::Deployment,
    network::Network,
    port::Port,
};

pub enum ServiceCondition {
    Started,
    Healthy,
    CompletedSuccessfully,
}

pub enum DependsOn {
    ServiceNames(Vec<String>),
    Services(HashMap<String, ServiceCondition>),
}

pub enum PullPolicy {
    Always,
    IfNotPresent,
    Never,
    Build,
    Missing,
}

pub struct Service {
    pub annotations: ListOrDict,
    pub build: Build,
    pub command: String,
    pub depends_on: DependsOn,
    pub deploy: Option<Deployment>,
    pub dns_search: StringOrList,
    pub dns: StringOrList,
    pub env_file: StringOrList,
    pub environment: ListOrDict,
    pub expose: serde_yaml::Value, // string or number
    pub external_links: Vec<String>,
    pub group_add: serde_yaml::Value, // string or number
    pub hostname: String,
    pub image: String,
    pub init: bool,
    pub ipc: String,
    pub isolation: String,
    pub labels: ListOrDict,
    pub links: Vec<String>,
    pub networks: Network,
    pub platform: String,
    pub ports: Vec<Port>,
    pub privileged: bool,
    pub profiles: Vec<String>,
    pub pull_policy: PullPolicy,
    pub read_only: bool,
    pub restart: String,
    pub runtime: String,
    pub scale: u8,
    pub security_opt: Vec<String>,
    pub shm_size: serde_yaml::Value, // string or number
    pub tty: bool,
    pub volumes_from: Vec<String>,
    pub volumes: Vec<String>,
    pub working_dir: String,
}
