pub struct BuildConfig {
    pub additional_contexts: serde_yaml::Value, // list or dict
    pub args: serde_yaml::Value,                // list or dict
    pub cache_from: Vec<String>,
    pub cache_to: Vec<String>,
    pub context: String,
    pub dockerfile: String,
    pub extra_hosts: serde_yaml::Value, // list or dict
    pub isolation: String,
    pub labels: serde_yaml::Value, // list or dict
    pub network: String,
    pub no_cache: bool,
    pub platform: Vec<String>,
    pub privileged: bool,
    pub pull: bool,
    pub shm_size: serde_yaml::Value, // string or number
    pub ssh: serde_yaml::Value,      // list or dict
    pub tags: Vec<String>,
    pub target: String,
}

pub enum Build {
    Path(String),
    Config(BuildConfig),
}
