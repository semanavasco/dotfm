use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML deserialization error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Environment error: {0}")]
    Env(#[from] shellexpand::LookupError<VarError>),

    #[error("{0}")]
    Msg(String),
}
