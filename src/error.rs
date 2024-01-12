use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("no config file ({{project-name}}) found")]
    NoConfig,

    #[error("could not parse config file: {0}")]
    ConfigParse(#[from] toml::de::Error),
}
