use std::{fmt::Display, io};

use super::validate::ValidateError;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IO(io::Error),
    TomlDecode(toml::de::Error),
    Validate(ValidateError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TomlDecode(err) => format!("could not decode TOML syntax: {err}"),
                Self::IO(err) => format!("could not perform IO operation: {err}"),
                Self::Validate(err) => format!("configuration invalid: {err}"),
            }
        )
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDecode(err)
    }
}

impl From<ValidateError> for Error {
    fn from(err: ValidateError) -> Self {
        Self::Validate(err)
    }
}
