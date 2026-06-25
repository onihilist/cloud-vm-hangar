use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::cloud::Provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VM {
    pub id: String,
    pub provider: Provider,
    pub name: String,
    pub region: String,
    pub state: VMState,
    pub instance_type: String,
    pub public_ip: Option<String>,
    pub tags: HashMap<String, String>,
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