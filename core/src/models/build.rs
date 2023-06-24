use super::common::ListOrDict;

pub enum ShmSize {
    String(String),
    Number(u64),
}

pub struct BuildConfig {
    pub additional_contexts: ListOrDict,
    pub args: ListOrDict,
    pub cache_from: Vec<String>,
    pub cache_to: Vec<String>,
    pub context: String,
    pub dockerfile: String,
    pub extra_hosts: ListOrDict,
    pub isolation: String,
    pub labels: ListOrDict,
    pub network: String,
    pub no_cache: bool,
    pub platform: Vec<String>,
    pub privileged: bool,
    pub pull: bool,
    pub shm_size: ShmSize,
    pub ssh: ListOrDict,
    pub tags: Vec<String>,
    pub target: String,
}

pub enum Build {
    Path(String),
    Config(BuildConfig),
}
