use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror:Error, Debug)]
pub enum Error {
    #[error("Couldn't find config in 'config.toml'.")]
    NoConfigFound,

    #[error(transparent)]
    TomlSerializationError(#[from] toml::ser::Error),

    #[error(transparent)]
    TomlDeserializationError(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}
