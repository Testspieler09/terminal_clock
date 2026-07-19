pub mod app_config_loader;
pub(crate) mod bundled;
pub mod clock_face_loader;
pub mod color_theme_loader;
pub(crate) mod configs;
pub mod quote_loader;

use std::{fmt, io, path::PathBuf};

pub type LoaderResult<T> = Result<T, AssetsLoadError>;

#[derive(Debug)]
pub enum AssetsLoadError {
    Io(io::Error),
    Toml(toml::de::Error),
    InvalidConfig(String),
}

impl fmt::Display for AssetsLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Toml(e) => write!(f, "TOML parse error: {e}"),
            Self::InvalidConfig(e) => write!(f, "Invalid config: {e}"),
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

pub fn get_user_config_path() -> LoaderResult<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("LOCALAPPDATA")
            .map_err(|e| AssetsLoadError::InvalidConfig(e.to_string()))?;
        Ok(PathBuf::from(appdata).join("terminal_clock"))
    }

    // NOTE: on MacOS .config is not the default
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let base = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg)
        } else {
            let home =
                std::env::var("HOME").map_err(|e| AssetsLoadError::InvalidConfig(e.to_string()))?;
            PathBuf::from(home).join(".config")
        };
        Ok(base.join("terminal_clock"))
    }
}
