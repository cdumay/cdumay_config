cdumay_core::define_kinds! {
    InvalidConfiguration=(400, "Invalid Configuration"),
}

cdumay_core::define_errors! {
    ConfigurationFileError = InvalidConfiguration
}

impl From<ConfigurationFileError> for std::io::Error {
    fn from(e: ConfigurationFileError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}
