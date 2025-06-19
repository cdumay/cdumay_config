use cdumay_core::define_errors;
use cdumay_error::InvalidConfiguration;

define_errors! {
    ConfigurationFileError = InvalidConfiguration,
}

impl From<ConfigurationFileError> for std::io::Error {
    fn from(e: ConfigurationFileError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}
