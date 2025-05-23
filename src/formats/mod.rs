mod json;
pub use json::JsonManager;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::XmlManager;
#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "yaml")]
pub use yaml::YamlManager;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "toml")]
pub use toml::TomlManager;

/// Enum representing the supported content formats for configuration files.
///
/// Each variant corresponds to a specific data serialization format.
/// Additional formats (YAML, XML, TOML) are enabled via Cargo features.
pub enum ContentFormat {
    /// JSON format (always available).
    JSON,

    /// YAML format (available only if the `yaml` feature is enabled).
    #[cfg(feature = "yaml")]
    YAML,

    /// XML format (available only if the `xml` feature is enabled).
    #[cfg(feature = "xml")]
    XML,

    /// TOML format (available only if the `toml` feature is enabled).
    #[cfg(feature = "toml")]
    TOML,
}
impl Default for ContentFormat {
    /// Provides the default format used when none is explicitly specified.
    ///
    /// Defaults to `ContentFormat::JSON`.
    fn default() -> ContentFormat {
        ContentFormat::JSON
    }
}
/// Reads a configuration file and deserializes its content into a strongly typed Rust value.
///
/// # Type Parameters
/// - `C`: The type to deserialize the configuration into. Must implement `DeserializeOwned`.
///
/// # Parameters
/// - `path`: Path to the configuration file. Tilde `~` expansion is supported.
/// - `format`: Optional format specifier. Defaults to `JSON` if not provided.
/// - `context`: A templating context used to resolve variables inside the configuration.
///
/// # Returns
/// The deserialized configuration of type `C`, or an error if reading or parsing fails.
///
/// # Example
/// ```rust
/// fn load() -> Result<String, cdumay_core::Error> {
///     let mut context = std::collections::BTreeMap::new();
///     cdumay_config::read_config("~/.config/app.json", None, &context)
/// }
/// ```
pub fn read_config<C: serde::de::DeserializeOwned>(
    path: &str,
    format: Option<ContentFormat>,
    context: &std::collections::BTreeMap<String, serde_value::Value>,
) -> Result<C, cdumay_core::Error> {
    let path = shellexpand::tilde(path);
    log::info!("Reading config file '{}'", path.as_ref());
    match format.unwrap_or(ContentFormat::JSON) {
        ContentFormat::JSON => JsonManager::new(path.to_string()).read_config(context),
        #[cfg(feature = "yaml")]
        ContentFormat::YAML => YamlManager::new(path.to_string()).read_config(context),
        #[cfg(feature = "xml")]
        ContentFormat::XML => XmlManager::new(path.to_string()).read_config(context),
        #[cfg(feature = "toml")]
        ContentFormat::TOML => TomlManager::new(path.to_string()).read_config(context),
    }
}

/// Serializes and writes a Rust value to a configuration file in a specified format.
///
/// # Type Parameters
/// - `C`: The data type to serialize. Must implement `Serialize`.
///
/// # Parameters
/// - `path`: The file path to write to. Tilde `~` expansion is supported.
/// - `format`: Optional output format. Defaults to `JSON` if not provided.
/// - `data`: The data to serialize and write to the file.
/// - `context`: Templating context for value substitution, if applicable.
///
/// # Returns
/// The path to the written file if successful, or an error otherwise.
///
/// # Example
/// ```
/// fn write<S: serde::Serialize>(config: S) -> Result<std::path::PathBuf, cdumay_core::Error> {
///     let mut context = std::collections::BTreeMap::new();
///     cdumay_config::write_config("~/.config/app.json", Some(cdumay_config::ContentFormat::JSON), &config, &context)
/// }
/// ```
pub fn write_config<C: serde::Serialize>(
    path: &str,
    format: Option<ContentFormat>,
    data: C,
    context: &std::collections::BTreeMap<String, serde_value::Value>,
) -> Result<std::path::PathBuf, cdumay_core::Error> {
    let path = shellexpand::tilde(path);
    log::info!("Saving config file '{}'", path.as_ref());
    match format.unwrap_or(ContentFormat::JSON) {
        ContentFormat::JSON => JsonManager::new(path.to_string()).write_config(&data, context),
        #[cfg(feature = "yaml")]
        ContentFormat::YAML => YamlManager::new(path.to_string()).write_config(&data, context),
        #[cfg(feature = "xml")]
        ContentFormat::XML => XmlManager::new(path.to_string()).write_config(&data, context),
        #[cfg(feature = "toml")]
        ContentFormat::TOML => TomlManager::new(path.to_string()).write_config(&data, context),
    }
}

/// A trait defining common operations for configuration file managers.
///
/// This abstraction allows handling different formats (e.g. JSON, YAML, etc.)
/// through a unified interface for reading and writing configurations.
pub trait Manager {
    /// Constructs a new instance of the manager with the given file path.
    ///
    /// # Parameters
    /// - `path`: Path to the configuration file.
    ///
    /// # Returns
    /// A new instance of the implementing manager.
    fn new(path: String) -> Self;
    
    /// Returns the file path associated with the manager.
    fn path(&self) -> String;
    
    /// Opens the configuration file for reading.
    ///
    /// # Parameters
    /// - `context`: A context used for error details if the operation fails.
    ///
    /// # Returns
    /// A readable `File` handle or an error if the file cannot be opened.
    fn open_file(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> Result<std::fs::File, cdumay_core::Error> {
        Ok(std::fs::File::open(self.path()).map_err(|err| {
            crate::ConfigurationFileError::new()
                .with_message(format!("Failed to open file: {}", err))
                .with_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }
    
    /// Creates (or overwrites) the configuration file for writing.
    ///
    /// # Parameters
    /// - `context`: A context used for error details if the operation fails.
    ///
    /// # Returns
    /// A writable `File` handle or an error if the file cannot be created.
    fn create_file(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> Result<std::fs::File, cdumay_core::Error> {
        Ok(std::fs::File::create(self.path()).map_err(|err| {
            crate::ConfigurationFileError::new()
                .with_message(format!("Failed to create file: {}", err))
                .with_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }
    
    /// Reads and deserializes configuration data from a readable input stream.
    ///
    /// # Type Parameters
    /// - `R`: A type implementing `Read` for input.
    /// - `C`: The type into which the data will be deserialized.
    ///
    /// # Parameters
    /// - `reader`: A readable input stream.
    /// - `context`: A context used for error reporting and template substitution.
    ///
    /// # Returns
    /// The deserialized configuration object.
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> Result<C, cdumay_core::Error>;
    
    /// Serializes and writes configuration data to a writable output stream.
    ///
    /// # Type Parameters
    /// - `D`: The data type to serialize.
    /// - `W`: A type implementing `Write` for output.
    ///
    /// # Parameters
    /// - `writer`: A writable output stream.
    /// - `data`: The data to serialize.
    /// - `context`: A context used for template substitution or error reporting.
    ///
    /// # Returns
    /// An empty result on success, or an error if writing fails.
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> Result<(), cdumay_core::Error>;
    
    /// Reads configuration directly from the file path managed by this instance.
    ///
    /// Internally calls `open_file` and then `read`.
    ///
    /// # Type Parameters
    /// - `C`: The target deserialization type.
    ///
    /// # Parameters
    /// - `context`: A context for error handling and templating.
    ///
    /// # Returns
    /// The deserialized configuration object.
    fn read_config<C: serde::de::DeserializeOwned>(
        &self,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> Result<C, cdumay_core::Error> {
        self.read(self.open_file(context)?, context)
    }
    
    /// Writes configuration data directly to the file path managed by this instance.
    ///
    /// Internally calls `create_file` and then `write`.
    ///
    /// # Type Parameters
    /// - `C`: The type of the configuration data to serialize.
    ///
    /// # Parameters
    /// - `data`: A reference to the configuration data.
    /// - `context`: A context used for error details and templating.
    ///
    /// # Returns
    /// The path to the file where the configuration was written.
    fn write_config<C: serde::Serialize>(
        &self,
        data: &C,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> Result<std::path::PathBuf, cdumay_core::Error> {
        let _ = self.write(self.create_file(context)?, data, context)?;
        Ok(std::path::PathBuf::from(self.path()))
    }
    
    /// Reads configuration data from a raw string and deserializes it.
    ///
    /// This method is static and typically used to parse embedded or in-memory content.
    ///
    /// # Type Parameters
    /// - `C`: The type into which the string will be deserialized.
    ///
    /// # Parameters
    /// - `content`: The string content containing the serialized configuration.
    /// - `context`: A context for templating and error reporting.
    ///
    /// # Returns
    /// The deserialized configuration object or an error.
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> Result<C, cdumay_core::Error>;
}
