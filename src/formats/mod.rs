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

pub enum ContentFormat {
    JSON,
    #[cfg(feature = "yaml")]
    YAML,
    #[cfg(feature = "xml")]
    XML,
    #[cfg(feature = "toml")]
    TOML,
}
impl Default for ContentFormat {
    fn default() -> ContentFormat {
        ContentFormat::JSON
    }
}

pub fn read_config<C: serde::de::DeserializeOwned>(
    path: &str,
    format: Option<ContentFormat>,
    context: &std::collections::BTreeMap<String, serde_value::Value>,
) -> cdumay_error::Result<C> {
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

pub fn write_config<C: serde::Serialize>(
    path: &str,
    format: Option<ContentFormat>,
    data: C,
    context: &std::collections::BTreeMap<String, serde_value::Value>,
) -> cdumay_error::Result<std::path::PathBuf> {
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

pub trait Manager {
    fn new(path: String) -> Self;
    fn path(&self) -> String;
    fn open_file(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_error::Result<std::fs::File> {
        Ok(std::fs::File::open(self.path()).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to open file: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }
    fn create_file(&self, context: &std::collections::BTreeMap<String, serde_value::Value>) -> cdumay_error::Result<std::fs::File> {
        Ok(std::fs::File::create(self.path()).map_err(|err| {
            crate::ConfigurationFileError::new()
                .set_message(format!("Failed to create file: {}", err))
                .set_details({
                    let mut ctx = context.clone();
                    ctx.insert("path".to_string(), serde_value::Value::String(self.path()));
                    ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                    ctx
                })
        })?)
    }
    fn read<R: std::io::Read, C: serde::de::DeserializeOwned>(
        &self,
        reader: R,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C>;
    fn write<D: serde::Serialize, W: std::io::Write>(
        &self,
        writer: W,
        data: D,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<()>;
    fn read_config<C: serde::de::DeserializeOwned>(
        &self,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C> {
        self.read(self.open_file(context)?, context)
    }
    fn write_config<C: serde::Serialize>(
        &self,
        data: &C,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<std::path::PathBuf> {
        let _ = self.write(self.create_file(context)?, data, context)?;
        Ok(std::path::PathBuf::from(self.path()))
    }
    fn read_str<C: serde::de::DeserializeOwned>(
        content: &str,
        context: &std::collections::BTreeMap<String, serde_value::Value>,
    ) -> cdumay_error::Result<C>;
}
