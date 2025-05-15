pub struct XmlManager {
    path: String,
}

impl crate::Manager for XmlManager {
    fn new(path: String) -> XmlManager {
        XmlManager { path }
    }
    fn path(&self) -> String {
        self.path.clone()
    }
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(serde_xml_rs::from_reader(reader).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Invalid XML file content: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
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
        Ok(serde_xml_rs::to_writer(writer, &data).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to write XML file: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        Ok(serde_xml_rs::from_str(content).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Invalid XML content: {}", err))
                .set_details(context.clone())
        })?)
    }
}
