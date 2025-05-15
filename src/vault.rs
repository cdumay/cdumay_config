use crate::formats::Manager;
use cdumay_error::AsError;

cdumay_error::define_errors! {
    VaultSecretError = crate::error::InvalidConfiguration
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultSecret {
    pub alias: String,
    pub key: String,
    pub value: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultSecrets {
    data: Vec<VaultSecret>,
}

impl VaultSecrets {
    pub fn alias<C: serde::de::DeserializeOwned>(
        &self,
        name: String,
        format: crate::ContentFormat,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        let aliases: std::collections::BTreeMap<String, String> = self.data.clone().into_iter().map(|item| (item.alias, item.value)).collect();
        match aliases.get(&name) {
            Some(value) => match format {
                crate::ContentFormat::JSON => crate::JsonManager::read_str(value, &context),
                #[cfg(feature = "yaml")]
                crate::ContentFormat::YAML => crate::YamlManager::read_str(value, &context),
                #[cfg(feature = "xml")]
                crate::ContentFormat::XML => crate::XmlManager::read_str(value, &context),
                #[cfg(feature = "toml")]
                crate::ContentFormat::TOML => crate::TomlManager::read_str(value, &context),
            },
            None => Err(VaultSecretError::new()
                .set_message(format!("Invalid alias: {}", name))
                .set_details(context.clone())
                .into()),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultConfig {
    secrets: Option<VaultSecrets>,
}

impl VaultConfig {
    pub fn init(path: &str, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_error::Result<VaultConfig> {
        Ok(VaultConfig {
            secrets: Some(VaultSecrets {
                data: crate::JsonManager::new(path.to_string()).read_config(context)?,
            }),
        })
    }
    pub fn secrets(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_error::Result<VaultSecrets> {
        match self.secrets.clone() {
            None => Err(VaultSecretError::new()
                .set_message("Failed to read vault data".to_string())
                .set_details(context.clone())
                .into()),
            Some(secrets) => Ok(secrets),
        }
    }
}
