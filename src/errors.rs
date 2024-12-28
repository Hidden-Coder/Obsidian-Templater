use std::io;
use confy::ConfyError;


pub enum Errors {
    ConfigError(ConfyError),
    IOError(io::Error)
}
