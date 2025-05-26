//! This module defines a structure for managing secrets (like credentials or API keys)
//! retrieved from a vault-like configuration. It supports dynamic format parsing (e.g., JSON, YAML)
//! and deserialization into typed Rust values using context-aware templating.

use crate::InvalidConfiguration;
use crate::formats::Manager;
use cdumay_core::define_errors;

define_errors! {
    VaultSecretError = InvalidConfiguration
}

/// Represents a single secret stored in the vault.
///
/// Each secret has a user-defined alias, an internal key, and a string value
/// which can be deserialized later using a specific format.
#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultSecret {
    /// A human-readable name or identifier for the secret.
    alias: String,
    /// A technical or symbolic key identifier for the secret.
    key: String,
    /// The actual string value of the secret (e.g., a password or API key).
    value: String,
}

impl VaultSecret {
    /// Creates a new `VaultSecret` instance with the given alias, key, and value.
    ///
    /// # Parameters
    /// - `alias`: A human-readable identifier for the secret.
    /// - `key`: A technical or symbolic key representing the secret.
    /// - `value`: The actual string value of the secret (e.g., a token or password).
    ///
    /// # Returns
    /// A new `VaultSecret` instance.
    pub fn new(alias: &str, key: &str, value: &str) -> Self {
        Self {
            alias: alias.to_string(),
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

/// A collection of multiple secrets loaded from a configuration source.
///
/// Provides utility methods for accessing secrets by alias and deserializing
/// them into strongly typed values.
#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultSecrets {
    data: Vec<VaultSecret>,
}

impl VaultSecrets {
    /// Creates a new `VaultSecrets` instance from a given list of secrets.
    ///
    /// # Parameters
    /// - `data`: A vector of `VaultSecret` items representing the stored secrets.
    ///
    /// # Returns
    /// A `VaultSecrets` instance containing the provided secrets.
    ///
    /// # Example
    /// ```rust
    /// use cdumay_config::{VaultSecrets, VaultSecret};
    ///
    /// let secrets = vec![
    ///     VaultSecret::new("api", "api_key", "1234")
    /// ];
    /// let vault = VaultSecrets::new(secrets);
    /// ```
    pub fn new(data: Vec<VaultSecret>) -> Self {
        Self { data }
    }
    /// Retrieves and deserializes a secret value by its alias.
    ///
    /// # Type Parameters
    /// - `C`: The target deserialization type.
    ///
    /// # Parameters
    /// - `name`: The alias of the secret to retrieve.
    /// - `format`: The format used to deserialize the secret's value (e.g. JSON, YAML).
    /// - `context`: A templating context used for value substitution (e.g. variables).
    ///
    /// # Returns    
    /// The deserialized secret as type `C` if successful, or an error
    /// if the alias doesn't exist or deserialization fails.
    ///
    /// # Errors
    /// Returns a [`VaultSecretError`] if the alias is not found or deserialization fails.
    pub fn alias<C: serde::de::DeserializeOwned>(
        &self,
        name: String,
        format: crate::ContentFormat,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
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
                .with_message(format!("Invalid alias: {}", name))
                .with_details(context.clone())
                .into()),
        }
    }
}

/// Configuration structure for loading secrets from an external file.
///
/// Wraps the underlying list of secrets and provides initialization and access methods.
///
/// # Example
/// ```rust
/// fn load() -> cdumay_core::Result<String> {
///     let mut context = std::collections::BTreeMap::new();
///     let config = cdumay_config::VaultConfig::init("vault.json", &context)?;
///     context.insert("env".to_string(), serde_value::Value::String("prod".to_string()));
///     
///     let secrets = config.secrets(&context)?;
///     secrets.alias("my_alias".to_string(), cdumay_config::ContentFormat::JSON, &context)
/// }
/// ```
#[derive(serde::Deserialize, Clone, Debug)]
pub struct VaultConfig {
    pub secrets: Option<VaultSecrets>,
}

impl VaultConfig {
    /// Initializes a new `VaultConfig` instance from a JSON configuration file.
    ///
    /// # Parameters
    /// - `path`: The file path to the JSON configuration containing the secrets.
    /// - `context`: A context used to resolve templated values in the configuration.
    ///
    /// # Returns
    /// A `VaultConfig` populated with secrets if successful.
    ///
    /// # Errors
    /// Returns a deserialization or file read error if the JSON cannot be parsed.
    pub fn init(path: &str, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_core::Result<VaultConfig> {
        Ok(VaultConfig {
            secrets: Some(VaultSecrets {
                data: crate::JsonManager::new(path.to_string()).read_config(context)?,
            }),
        })
    }
    /// Returns the list of secrets if they have been loaded.
    ///
    /// # Parameters
    /// - `context`: Context passed to generate a detailed error if secrets are missing.
    ///
    /// # Returns
    /// The `VaultSecrets` if available, or an error otherwise.
    pub fn secrets(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_core::Result<VaultSecrets> {
        match self.secrets.clone() {
            None => Err(VaultSecretError::new()
                .with_message("Failed to read vault data".to_string())
                .with_details(context.clone())
                .into()),
            Some(secrets) => Ok(secrets),
        }
    }
}
