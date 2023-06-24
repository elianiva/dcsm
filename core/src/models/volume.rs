use super::common::StringOrNumber;

pub enum SELinux {
    /// same as using :z (lowercase)
    Shared,
    /// same as using :Z (uppercase)
    Private,
}

pub struct Bind {
    pub propagation: String,
    pub create_host_path: bool,
    pub selinux: SELinux,
}

pub struct VolumeField {
    pub nocopy: bool,
}

pub struct TmpFs {
    pub size: StringOrNumber<u64>,
}

pub struct ServiceVolumeConfig {
    pub r#type: String,
    pub source: String,
    pub target: String,
    pub read_only: bool,
    pub consistency: String,
    pub bind: Bind,
    pub volume: VolumeField,
    pub tmpfs: TmpFs,
}

pub enum ServiceVolume {
    Name(String),
    Config(ServiceVolumeConfig),
}
