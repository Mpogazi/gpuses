use openssl::error::ErrorStack;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use vaultrs::api::kv2::responses::SecretVersionMetadata;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    key: String,
    value: String,
}

pub struct Keys {
    vault_client: VaultClient,
}

impl Keys {
    pub fn new() -> Keys {
        Keys {
            vault_client: VaultClient::new(
                VaultClientSettingsBuilder::default()
                    .address("http://127.0.0.1:8200")
                    .token("hvs.dDyruCH0wAjMW2jMlE7P3oOH")
                    .build()
                    .unwrap(),
            )
            .unwrap(),
        }
    }

    fn create_pairs(&self) -> Result<(String, String), ErrorStack> {
        let rsa = Rsa::generate(2048)?;
        let private_key_pem = rsa.private_key_to_pem()?;
        let public_key_pem = rsa.public_key_to_pem()?;

        let public_key = String::from_utf8(public_key_pem).unwrap();
        let private_key = String::from_utf8(private_key_pem).unwrap();
        Ok((private_key, public_key))
    }

    // We will assume the user_id is the unique id of a user in the system
    // Therefore we assume that we are not gonna have any collision
    async fn save_public_key(
        &self,
        user_id: &str,
        public_key: &str,
    ) -> Result<SecretVersionMetadata, ClientError> {
        let secret_mount = "secret";
        let secret_data = Key {
            key: "public_key".to_string(),
            value: public_key.to_string(),
        };
        kv2::set(&self.vault_client, secret_mount, user_id, &secret_data).await
    }

    pub async fn generate(&self, user_id: String) -> Result<String, ErrorStack> {
        let (private_key, public_key) = self.create_pairs().unwrap();
        self.save_public_key(&user_id, &public_key).await.unwrap();
        Ok(private_key)
    }

    pub async fn get_key(&self, user_id: &str) -> Result<String, ClientError> {
        let secret_mount = "secret";
        let public_key: Key = kv2::read(&self.vault_client, secret_mount, user_id)
            .await
            .unwrap();
        Ok(public_key.value.to_string())
    }
}
