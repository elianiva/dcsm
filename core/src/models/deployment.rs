use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RollbackOrder {
    #[serde(rename = "start_first")]
    StartFirst,
    #[serde(rename = "stop_first")]
    StopFirst,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAndRollbackConfig {
    pub parallelism: i32,
    pub delay: String,
    pub failure_action: String,
    pub monitor: String,
    pub max_failure_ratio: i32,
    pub order: RollbackOrder,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceLimit {
    pub cpus: serde_yaml::Value, // string or number
    pub memory: String,
    pub pids: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DiscreteResourceSpec {
    pub kind: String,
    pub value: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GenericResource {
    pub discrete_resource_spec: DiscreteResourceSpec,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub capabilities: Vec<String>,
    pub device_ids: Vec<String>,
    pub count: serde_yaml::Value, // string or number
    pub driver: String,
    pub options: serde_yaml::Value, // list or dict
}

#[derive(Serialize, Deserialize)]
pub struct Reservation {
    pub cpus: serde_yaml::Value, // string or number
    pub memory: String,
    pub generic_resource: Vec<GenericResource>,
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub limits: ResourceLimit,
    pub reservations: Reservation,
}

#[derive(Serialize, Deserialize)]
pub struct RestartPolicy {
    pub condition: String,
    pub delay: String,
    pub max_attempts: u8,
    pub window: String,
}

#[derive(Serialize, Deserialize)]
pub struct Preference {
    pub spread: String,
}

#[derive(Serialize, Deserialize)]
pub struct Placement {
    pub constraints: Vec<String>,
    pub preferences: Vec<Preference>,
}

#[derive(Serialize, Deserialize)]
pub struct Deployment {
    pub mode: String,
    pub endpoint_mode: String,
    pub replicas: i32,
    pub labels: serde_yaml::Value, // list or dict
    pub rollback_config: UpdateAndRollbackConfig,
    pub update_config: UpdateAndRollbackConfig,
    pub resources: Resource,
    pub restart_policy: RestartPolicy,
    pub placement: Placement,
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn serialise_update_and_rollback_config() {
        let config = UpdateAndRollbackConfig {
            parallelism: 1,
            delay: "1s".to_string(),
            failure_action: "stop".to_string(),
            monitor: "1s".to_string(),
            max_failure_ratio: 1,
            order: RollbackOrder::StartFirst,
        };
        let serialised = serde_yaml::to_string(&config).unwrap();
        assert_eq!(
            serialised,
            r#"parallelism: 1
delay: "1s"
failure_action: "stop"
monitor: "1s"
max_failure_ratio: 1
order: "start_first""#
        );
    }
}
