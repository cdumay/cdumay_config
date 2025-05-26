/// XML configuration file manager implementing the `Manager` trait.
///
/// This struct provides methods to read and write XML-formatted configuration
/// files using the `serde_xml_rs` crate.
pub struct XmlManager {
    /// Path to the XML configuration file.
    path: String,
}

impl crate::Manager for XmlManager {
    /// Creates a new `XmlManager` with the specified file path.
    ///
    /// # Parameters
    /// - `path`: The path to the XML file.
    ///
    /// # Returns
    /// A new instance of `XmlManager`.
    fn new(path: String) -> XmlManager {
        XmlManager { path }
    }

    /// Returns the path to the XML configuration file.
    ///
    /// # Returns
    /// The file path as a `String`.
    fn path(&self) -> String {
        self.path.clone()
    }

    /// Reads XML content from a `Read` stream and deserializes it into the target type.
    ///
    /// # Type Parameters
    /// - `R`: A reader implementing `std::io::Read`.
    /// - `C`: The type to deserialize into, must implement `DeserializeOwned`.
    ///
    /// # Parameters
    /// - `reader`: A reader containing XML data.
    /// - `context`: Error context metadata.
    ///
    /// # Returns
    /// The deserialized configuration object or an error.
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        Ok(serde_xml_rs::from_reader(reader).map_err(|err| {
            crate::ConfigurationFileError::new()
                .with_message(format!("Invalid XML file content: {}", err))
                .with_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }

    /// Serializes data into XML format and writes it to the given `Write` stream.
    ///
    /// # Type Parameters
    /// - `D`: The data type to serialize (must implement `Serialize`).
    /// - `W`: A writable stream implementing `std::io::Write`.
    ///
    /// # Parameters
    /// - `writer`: The output stream.
    /// - `data`: The data to serialize and write.
    /// - `context`: Error context metadata.
    ///
    /// # Returns
    /// An empty result on success or an error on failure.
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<()> {
        Ok(serde_xml_rs::to_writer(writer, &data).map_err(|err| {
            crate::ConfigurationFileError::new()
                .with_message(format!("Failed to write XML file: {}", err))
                .with_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }

    /// Deserializes a string of XML content into the target type.
    ///
    /// # Type Parameters
    /// - `C`: The target type, must implement `DeserializeOwned`.
    ///
    /// # Parameters
    /// - `content`: A string slice containing XML data.
    /// - `context`: Error context metadata.
    ///
    /// # Returns
    /// The deserialized object or an error.
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        Ok(serde_xml_rs::from_str(content).map_err(|err| {
            crate::ConfigurationFileError::new()
                .with_message(format!("Invalid XML content: {}", err))
                .with_details(context.clone())
        })?)
    }
}
