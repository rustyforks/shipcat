/// Kubernetes resource requests
#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceRequest {
    /// CPU request string
    pub cpu: String,
    /// Memory request string
    pub memory: String,
    // TODO: ephemeral-storage + extended-resources
}

/// Kubernetes resource limits
#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceLimit {
    /// CPU limit string
    pub cpu: String,
    /// Memory limit string
    pub memory: String,
    // TODO: ephemeral-storage + extended-resources
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Resources {
    /// Resource requests for k8s
    pub requests: Option<ResourceRequest>,
    /// Resource limits for k8s
    pub limits: Option<ResourceLimit>,
}

// HostAlias support for all pods regardless of network configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HostAlias {
    /// ip address string
    pub ip: String,
    /// add additional entries that resolve the ip address to the hosts file
    pub hostnames: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ConfigMappedFile {
    /// Name of file to template (from service repo paths)
    pub name: String,
    /// Name of file inside container
    pub dest: String,
    /// Config value inlined
    ///
    /// This is usually filled in internally by to help out Helm a bit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ConfigMap {
    /// Optional k8s specific name for the mount (autogenerated if left out)
    pub name: Option<String>,
    /// Container-local directory path where configs are available
    pub mount: String,
    /// Files from the config map to mount at this mountpath
    pub files: Vec<ConfigMappedFile>
}


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HealthCheck {
    /// Where the health check is located
    #[serde(default = "health_check_url_default")]
    pub uri: String,
    /// How long to wait after boot in seconds
    #[serde(default = "health_check_wait_time_default")]
    pub wait: u32,
}
fn health_check_url_default() -> String { "/health".into() }
fn health_check_wait_time_default() -> u32 { 30 }


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VolumeMount {
    pub name: String,
    pub mountPath: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPath: Option<String>,
    #[serde(default = "volume_mount_read_only")]
    pub readOnly: bool,
}
fn volume_mount_read_only() -> bool { false }

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InitContainer {
    pub name: String,
    pub image: String,
    pub command: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VolumeSecretItem {
    #[serde(default = "volume_key")]
    pub key: String,
    pub path: String,
    #[serde(default = "volume_default_mode")]
    pub mode: u32,
}
fn volume_key() -> String { "value".into() }
fn volume_default_mode() -> u32 { 420 } // 0644

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VolumeSecretDetail {
    pub name: String,
    pub items: Vec<VolumeSecretItem>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VolumeSecret {
    pub secret: Option<VolumeSecretDetail>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProjectedVolumeSecret {
    pub sources: Vec<VolumeSecret>,
    // pub default_mode: u32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Volume {
    pub name: String,
    /// A projection combines multiple volume items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected: Option<ProjectedVolumeSecret>,
    /// The secret is fetched  from kube secrets and mounted as a volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<VolumeSecretDetail>,
}
