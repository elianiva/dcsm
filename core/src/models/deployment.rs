use super::common::{ListOrDict};

pub enum RollbackOrder {
    StartFirst,
    StopFirst,
}

pub struct UpdateAndRollbackConfig {
    pub parallelism: i32,
    pub delay: String,
    pub failure_action: String,
    pub monitor: String,
    pub max_failure_ratio: i32,
    pub order: RollbackOrder,
}

pub struct ResourceLimit {
    pub cpus: serde_yaml::Value, // string or number
    pub memory: String,
    pub pids: u8,
}

pub struct DiscreteResourceSpec {
    pub kind: String,
    pub value: i32,
}

pub struct GenericResource {
    pub discrete_resource_spec: DiscreteResourceSpec,
}

pub struct Device {
    pub capabilities: Vec<String>,
    pub device_ids: Vec<String>,
    pub count: serde_yaml::Value, // string or number
    pub driver: String,
    pub options: ListOrDict,
}

pub struct Reservation {
    pub cpus: serde_yaml::Value, // string or number
    pub memory: String,
    pub generic_resource: Vec<GenericResource>,
    // TODO(elianiva): create the struct
    // pub devices: None,
}

pub struct Resource {
    pub limits: ResourceLimit,
    pub reservations: Reservation,
}

pub struct RestartPolicy {
    pub condition: String,
    pub delay: String,
    pub max_attempts: u8,
    pub window: String,
}

pub struct Preference {
    pub spread: String,
}

pub struct Placement {
    pub constraints: Vec<String>,
    pub preferences: Vec<Preference>,
}

pub struct Deployment {
    pub mode: String,
    pub endpoint_mode: String,
    pub replicas: i32,
    pub labels: ListOrDict,
    pub rollback_config: UpdateAndRollbackConfig,
    pub update_config: UpdateAndRollbackConfig,
    pub resources: Resource,
    pub restart_policy: RestartPolicy,
    pub placement: Placement,
}
