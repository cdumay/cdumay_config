pub struct YamlManager {
    path: String,
}

impl crate::Manager for YamlManager {
    fn new(path: String) -> YamlManager {
        YamlManager { path }
    }
    fn path(&self) -> String {
        self.path.clone()
    }
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(serde_yaml::from_reader(reader).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Invalid YAML file content: {}", err))
                .set_details({
                    let mut ctx = context.clone();
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
        Ok(serde_yaml::to_writer(writer, &data).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write YAML file: {}", err))
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
        Ok(serde_yaml::from_str(content).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Invalid YAML content: {}", err))
                .set_details(context.clone())
        })?)
    }
}
