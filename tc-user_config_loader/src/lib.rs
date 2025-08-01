pub mod clock_face_loader;
pub mod colorscheme_loader;
pub(crate) mod default_themes;
pub mod quote_loader;

use std::{fmt, io, path::PathBuf};

type LoaderResult<T> = Result<T, ColorSchemeLoadError>;

#[derive(Debug)]
pub enum ColorSchemeLoadError {
    Io(io::Error),
    ConfigPath(String),
    Toml(toml::de::Error),
}

impl fmt::Display for ColorSchemeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::ConfigPath(e) => write!(f, "Invalid config path: {e}"),
            Self::Toml(e) => write!(f, "TOML parse error: {e}"),
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

// TODO: adjust for general use
pub(crate) fn get_user_config_path() -> LoaderResult<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
        Ok(PathBuf::from(appdata).join("terminal_clock").join("themes"))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let home =
            std::env::var("HOME").map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
        Ok(PathBuf::from(home)
            .join(".config")
            .join("terminal_clock")
            .join("themes"))
    }
}
