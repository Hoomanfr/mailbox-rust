use serde::{Deserialize, Serialize};
use std::{env, error::Error};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

pub struct ConfigManager {
    environment: String,
    domain: String,
    service: String,
    client: VaultClient,
}

#[derive(Serialize, Deserialize)]
struct SecretValue {
    value: String,
}

impl ConfigManager {
    pub fn new() -> ConfigManager {
        let environment = env::var("ENVIRONMENT").unwrap();
        let domain = env::var("DOMAIN").unwrap();
        let service = env::var("SERVICE").unwrap();
        let vault_address = env::var("VAULT_ADDR").unwrap();
        let vault_token = env::var("VAULT_TOKEN").unwrap();
        let vault_client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .verify(false)
                .address(vault_address)
                .token(vault_token)
                .build()
                .unwrap(),
        )
        .unwrap();
        let cfg = ConfigManager {
            environment,
            domain,
            service,
            client: vault_client,
        };
        return cfg;
    }
    pub async fn get_value(&self, key: String) -> Result<String, Box<dyn Error>> {
        return self
            .get_service_value(&self.domain, &self.service, &key)
            .await;
    }
    pub async fn get_service_value(
        &self,
        domain: &String,
        service: &String,
        key: &String,
    ) -> Result<String, Box<dyn Error>> {
        let path = format!("{}/{}/{}/{}", self.environment, domain, service, key);
        let secret: SecretValue = kv2::read(&self.client, "secrets", path.as_str()).await?;
        Ok(secret.value)
    }
}
