use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use crate::vm::VM;

/// Common operations every cloud backend must support.
/// Add a new cloud by implementing this trait and registering it
/// in `build_aggregator_from_env`.
#[async_trait]
pub trait CloudProvider: Send + Sync {
    fn provider(&self) -> Provider;
    async fn list_vms(&self) -> anyhow::Result<Vec<VM>>;
    async fn start_vm(&self, id: &str) -> anyhow::Result<()>;
    async fn stop_vm(&self, id: &str) -> anyhow::Result<()>;
    async fn delete_vm(&self, id: &str) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    AWS,
    GCP,
    AZURE,
    DIGITAL_OCEAN
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Provider::AWS => "aws",
            Provider::GCP => "gcp",
            Provider::AZURE => "azure",
            Provider::DIGITAL_OCEAN => "digitalocean",
        };
        write!(f, "{s}")
    }
}

impl FromStr for Provider {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aws" => Ok(Provider::AWS),
            "gcp" => Ok(Provider::GCP),
            "azure" => Ok(Provider::AZURE),
            "digitalocean" | "do" => Ok(Provider::DIGITAL_OCEAN),
            other => Err(anyhow::anyhow!("unknown provider: {other}")),
        }
    }
}

pub mod providers {
    use std::collections::HashMap;
    use crate::vm::VMState;
    use super::*;

    /// TODO: wire up `aws_sdk_ec2::Client` here and implement against
    /// describe_instances / start_instances / stop_instances / terminate_instances.
    pub struct AwsProvider;

    #[async_trait]
    impl CloudProvider for AwsProvider {
        fn provider(&self) -> Provider {
            Provider::AWS
        }
        async fn list_vms(&self) -> anyhow::Result<Vec<VM>> {
            Ok(vec![VM{
                id: "id_test-vm-preinit".to_string(),
                provider: Provider::AWS,
                name: "test-vm-preinit".to_string(),
                region: "us".to_string(),
                state: VMState::STOPPED,
                instance_type: "ec2".to_string(),
                public_ip: Some("1.2.3.4".to_string()),
                tags: HashMap::new(),
            }])
        }
        async fn start_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn stop_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn delete_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }

    /// TODO: call the Compute Engine REST API via `reqwest`, authenticating
    /// with a service-account JWT (no official Rust SDK for GCP exists yet).
    pub struct GcpProvider;

    #[async_trait]
    impl CloudProvider for GcpProvider {
        fn provider(&self) -> Provider {
            Provider::GCP
        }
        async fn list_vms(&self) -> anyhow::Result<Vec<VM>> {
            Ok(vec![])
        }
        async fn start_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn stop_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn delete_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }

    /// TODO: wire up `azure_mgmt_compute` here, authenticating with a
    /// service principal (client id / secret / tenant id).
    pub struct AzureProvider;

    #[async_trait]
    impl CloudProvider for AzureProvider {
        fn provider(&self) -> Provider {
            Provider::AZURE
        }
        async fn list_vms(&self) -> anyhow::Result<Vec<VM>> {
            Ok(vec![])
        }
        async fn start_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn stop_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn delete_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }

    /// TODO: call the DigitalOcean REST API via `reqwest` with a bearer token.
    pub struct DigitalOceanProvider;

    #[async_trait]
    impl CloudProvider for DigitalOceanProvider {
        fn provider(&self) -> Provider {
            Provider::DIGITAL_OCEAN
        }
        async fn list_vms(&self) -> anyhow::Result<Vec<VM>> {
            Ok(vec![])
        }
        async fn start_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn stop_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
        async fn delete_vm(&self, _id: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }
}

/// Fans out list/start/stop/delete calls to whichever provider(s) are registered.
pub struct CloudAggregator {
    providers: Vec<Box<dyn CloudProvider>>,
}

impl CloudAggregator {
    pub fn new(providers: Vec<Box<dyn CloudProvider>>) -> Self {
        Self { providers }
    }

    pub async fn add_vm(&self, vm_name: &str) -> bool {
        false
    }

    pub async fn list_vms(&self, filter: Option<&str>) -> anyhow::Result<Vec<VM>> {
        let mut all = Vec::new();
        for p in &self.providers {
            if let Some(f) = filter {
                if p.provider().to_string() != f.to_lowercase() {
                    continue;
                }
            }
            all.extend(p.list_vms().await?);
        }
        Ok(all)
    }

    fn find(&self, provider: &str) -> anyhow::Result<&dyn CloudProvider> {
        let target: Provider = provider.parse()?;
        self.providers
            .iter()
            .find(|p| p.provider() == target)
            .map(|p| p.as_ref())
            .ok_or_else(|| anyhow::anyhow!("provider not configured: {provider}"))
    }

    pub async fn start_vm(&self, provider: &str, id: &str) -> anyhow::Result<()> {
        self.find(provider)?.start_vm(id).await
    }

    pub async fn stop_vm(&self, provider: &str, id: &str) -> anyhow::Result<()> {
        self.find(provider)?.stop_vm(id).await
    }

    pub async fn delete_vm(&self, provider: &str, id: &str) -> anyhow::Result<()> {
        self.find(provider)?.delete_vm(id).await
    }
}

/// Builds an aggregator from whichever provider credentials are present in
/// the environment. All four providers are currently registered as stubs;
/// as each one gets a real implementation, gate registration on the
/// presence of its credentials (e.g. `AWS_ACCESS_KEY_ID`) instead.
pub fn build_aggregator_from_env() -> CloudAggregator {
    use providers::*;
    CloudAggregator::new(vec![
        Box::new(AwsProvider),
        Box::new(GcpProvider),
        Box::new(AzureProvider),
        Box::new(DigitalOceanProvider),
    ])
}