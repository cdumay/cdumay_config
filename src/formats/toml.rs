pub struct TomlManager {
    path: String,
}

impl crate::Manager for TomlManager {
    fn new(path: String) -> TomlManager {
        TomlManager { path }
    }
    fn path(&self) -> String {
        self.path.clone()
    }
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
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<()> {
        Ok(serde_json::to_writer_pretty(writer, &data).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write TOML file: {}", err))
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
        Ok(toml::from_str(content).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to read TOML content: {}", err))
                .set_details(context.clone())
        })?)
    }
}
