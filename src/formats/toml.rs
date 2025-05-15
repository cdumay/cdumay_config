use cdumay_error::ErrorConverter;
/// TOML configuration file manager implementing the `Manager` trait.
///
/// This struct handles reading from and writing to TOML configuration files,
/// using the `toml` crate for serialization and deserialization.
pub struct TomlManager {
    /// Path to the TOML configuration file.
    path: String,
}

impl crate::Manager for TomlManager {
    /// Creates a new `TomlManager` with the specified file path.
    ///
    /// # Parameters
    /// - `path`: Path to the TOML configuration file.
    ///
    /// # Returns
    /// A new instance of `TomlManager`.
    fn new(path: String) -> TomlManager {
        TomlManager { path }
    }
    /// Returns the path to the TOML configuration file.
    fn path(&self) -> String {
        self.path.clone()
    }

    /// Reads TOML content from a `Read` stream, deserializing it into the specified type.
    ///
    /// The entire stream is first read into a `String`, then parsed as TOML.
    ///
    /// # Type Parameters
    /// - `R`: A type implementing `Read`.
    /// - `C`: The type into which the data will be deserialized.
    ///
    /// # Parameters
    /// - `reader`: A readable stream containing TOML data.
    /// - `context`: Context used for error reporting.
    ///
    /// # Returns
    /// The deserialized configuration object or an error.
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        mut reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write TOML file: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx
                })
        })?;
        Self::read_str(&buffer, context)
    }

    /// Serializes and writes data as pretty-printed TOML to a `Write` stream.
    ///
    /// # Type Parameters
    /// - `D`: The data type to serialize.
    /// - `W`: A type implementing `Write`.
    ///
    /// # Parameters
    /// - `writer`: A writable stream for output.
    /// - `data`: The data to serialize.
    /// - `context`: Context used for error reporting.
    ///
    /// # Returns
    /// Empty result on success, or an error on failure.
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        mut writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<()> {
        let mut ctx = context.clone();
        ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
        let content = cdumay_error_toml::convert_serialize_result!(toml::to_string_pretty(&data), ctx.clone())?;
        Ok(writer.write_all(content.as_bytes()).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write TOML file: {}", err))
                .set_details(ctx)
        })?)
    }

    /// Deserializes TOML content from a string slice.
    ///
    /// # Type Parameters
    /// - `C`: The type into which the content will be deserialized.
    ///
    /// # Parameters
    /// - `content`: The TOML string to parse.
    /// - `context`: Context used for error reporting.
    ///
    /// # Returns
    /// The deserialized object or an error if the content is invalid.
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(cdumay_error_toml::convert_deserialize_result!(toml::from_str(content), context.clone())?)
    }
}
