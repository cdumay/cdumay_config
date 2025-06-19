use cdumay_core::ErrorConverter;
/// JSON configuration file manager implementing the `Manager` trait.
///
/// This struct handles reading and writing JSON configuration files,
/// and integrates with the error and context handling system.
pub struct JsonManager {
    /// Path to the JSON configuration file.
    path: String,
}

impl crate::Manager for JsonManager {
    /// Creates a new `JsonManager` with the specified file path.
    ///
    /// # Parameters
    /// - `path`: Path to the JSON configuration file.
    ///
    /// # Returns
    /// A new instance of `JsonManager`.
    fn new(path: String) -> JsonManager {
        JsonManager { path }
    }

    /// Returns the path to the JSON configuration file.
    fn path(&self) -> String {
        self.path.clone()
    }

    /// Reads and deserializes JSON content from a `Read` stream.
    ///
    /// # Type Parameters
    /// - `R`: A type implementing `Read`.
    /// - `C`: The type into which the data will be deserialized.
    ///
    /// # Parameters
    /// - `reader`: A readable stream containing JSON data.
    /// - `context`: Context used for error reporting.
    ///
    /// # Returns
    /// The deserialized configuration object or an error.
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        let mut ctx = context.clone();
        ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
        cdumay_json::convert_json_result!(serde_json::from_reader(reader), ctx)
    }

    /// Serializes and writes data as pretty-printed JSON to a `Write` stream.
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
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<()> {
        let mut ctx = context.clone();
        ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
        cdumay_json::convert_json_result!(serde_json::to_writer_pretty(writer, &data), ctx)
    }

    /// Deserializes JSON content from a string slice.
    ///
    /// # Type Parameters
    /// - `C`: The type into which the content will be deserialized.
    ///
    /// # Parameters
    /// - `content`: The JSON string to parse.
    /// - `context`: Context used for error reporting.
    ///
    /// # Returns
    /// The deserialized object or an error if the content is invalid.
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_core::Result<C> {
        cdumay_json::convert_json_result!(serde_json::from_str(content), context.clone())
    }
}
