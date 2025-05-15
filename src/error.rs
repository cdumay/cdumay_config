use cdumay_error::AsError;

cdumay_error::define_kinds! {
    InvalidConfiguration=("CONFIG-00001", 400, "Invalid Configuration"),
}

cdumay_error::define_errors! {
    ConfigurationFileError = InvalidConfiguration
}

impl From<ConfigurationFileError> for std::io::Error {
    fn from(e: ConfigurationFileError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}
