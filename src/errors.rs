use std::io;
use confy::ConfyError;


#[derive(Debug)]
pub enum Errors {
    ConfigError(ConfyError),
    IOError(io::Error)
}

impl From<ConfyError> for Errors {
    fn from(error: ConfyError) -> Self {
        Errors::ConfigError(error)
    }
}

impl From<io::Error> for Errors {
    fn from(error: io::Error) -> Self {
        Errors::IOError(error)
    }
}
