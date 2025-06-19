use cdumay_core::ErrorConverter;
/// YAML configuration file manager implementing the `Manager` trait.
///
/// This struct handles reading and writing configuration data
/// in YAML format using the `serde_yaml` crate.
pub struct YamlManager {
    /// Path to the YAML configuration file.
    path: String,
}

impl crate::Manager for YamlManager {
    /// Creates a new `YamlManager` with the given file path.
    ///
    /// # Parameters
    /// - `path`: A string representing the path to the YAML file.
    ///
    /// # Returns
    /// A new instance of `YamlManager`.
    fn new(path: String) -> YamlManager {
        YamlManager { path }
    }

    /// Returns the file path associated with this manager.
    ///
    /// # Returns
    /// The file path as a `String`.
    fn path(&self) -> String {
        self.path.clone()
    }

    /// Reads YAML content from a `Read` stream and deserializes it into the target type.
    ///
    /// # Type Parameters
    /// - `R`: Reader implementing `std::io::Read`.
    /// - `C`: Type to deserialize into, must implement `DeserializeOwned`.
    ///
    /// # Parameters
    /// - `reader`: Input stream containing YAML data.
    /// - `context`: Contextual information for error reporting.
    ///
    /// # Returns
    /// Deserialized object or an error.
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        let mut ctx = context.clone();
        ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
        cdumay_yaml::convert_yaml_result!(serde_yaml::from_reader(reader), ctx)
    }

    /// Serializes data to YAML and writes it to the specified output stream.
    ///
    /// # Type Parameters
    /// - `D`: Data type implementing `Serialize`.
    /// - `W`: Output stream implementing `std::io::Write`.
    ///
    /// # Parameters
    /// - `writer`: Output stream to write YAML content.
    /// - `data`: The data to serialize.
    /// - `context`: Contextual information for error reporting.
    ///
    /// # Returns
    /// A success result or an error.
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<()> {
        let mut ctx = context.clone();
        ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
        cdumay_yaml::convert_yaml_result!(serde_yaml::to_writer(writer, &data), ctx)
    }

    /// Deserializes a YAML string into the target type.
    ///
    /// # Type Parameters
    /// - `C`: Type to deserialize into, must implement `DeserializeOwned`.
    ///
    /// # Parameters
    /// - `content`: YAML content as a string.
    /// - `context`: Contextual information for error reporting.
    ///
    /// # Returns
    /// Deserialized object or an error.
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        cdumay_yaml::convert_yaml_result!(serde_yaml::from_str(content), context.clone())
    }
}
