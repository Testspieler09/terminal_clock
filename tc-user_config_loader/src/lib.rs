pub mod clock_face_loader;
pub mod color_theme_loader;
pub(crate) mod configs;
pub(crate) mod default_themes;
pub mod quote_loader;

use std::{fmt, io, path::PathBuf};

pub type LoaderResult<T> = Result<T, AssetsLoadError>;

#[derive(Debug)]
pub enum AssetsLoadError {
    Io(io::Error),
    ConfigPath(String),
    Toml(toml::de::Error),
}

impl fmt::Display for AssetsLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::ConfigPath(e) => write!(f, "Invalid config path: {e}"),
            Self::Toml(e) => write!(f, "TOML parse error: {e}"),
        }
    }
}

impl std::error::Error for AssetsLoadError {}

impl From<io::Error> for AssetsLoadError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for AssetsLoadError {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}

pub(crate) fn get_user_config_path() -> LoaderResult<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata =
            std::env::var("APPDATA").map_err(|e| AssetsLoadError::ConfigPath(e.to_string()))?;
        Ok(PathBuf::from(appdata).join("terminal_clock"))
    }

    // NOTE: on MacOS .config is not the default
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let home = std::env::var("HOME").map_err(|e| AssetsLoadError::ConfigPath(e.to_string()))?;
        Ok(PathBuf::from(home).join(".config").join("terminal_clock"))
    }
}
