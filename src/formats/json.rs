pub struct JsonManager {
    path: String,
}

impl crate::Manager for JsonManager {
    fn new(path: String) -> JsonManager {
        JsonManager { path }
    }
    fn path(&self) -> String {
        self.path.clone()
    }
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(serde_json::from_reader(reader).map_err(|err| {
            let mut ctx = context.clone();
            crate::ConfigurationFileError::new()
                .set_message(format!("Invalid JSON file content: {}", err))
                .set_details({
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx
                })
        })?)
    }
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<()> {
        Ok(serde_json::to_writer_pretty(writer, &data).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write JSON file: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx
                })
        })?)
    }
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(serde_json::from_str(content).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to read JSON content: {}", err))
                .set_details(context.clone())
        })?)
    }
}
