

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VM {
    id: String,
    provider: Provider,
    name: String,
    region: String,
    state: VMState,
    instance_type: String,
    public_ip: Option<String>,
    tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VMState {
    RUNNING,
    STOPPED,
    BOOTING,
    SHUTTING_DOWN,
    SUSPENDED,
    TERMINATED,
    CONFIGURING,
    ERROR,
    UNKNOWN,
}