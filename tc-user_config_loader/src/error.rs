use std::{fmt, io};

#[derive(Debug)]
pub enum ColorSchemeLoadError {
    Io(io::Error),
    ConfigPath(String),
    Toml(toml::de::Error),
}

impl fmt::Display for ColorSchemeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::ConfigPath(e) => write!(f, "Invalid config path: {}", e),
            Self::Toml(e) => write!(f, "TOML parse error: {}", e),
        }
    }
}

impl std::error::Error for ColorSchemeLoadError {}

impl From<io::Error> for ColorSchemeLoadError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for ColorSchemeLoadError {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}
