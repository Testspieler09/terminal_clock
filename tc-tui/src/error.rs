use std::{fmt, io};

use tc_user_config_loader::AssetsLoadError;

pub enum AppError {
    Config(AssetsLoadError),
    Io(io::Error),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => match e {
                AssetsLoadError::InvalidConfig(msg) => write!(
                    f,
                    "Your configuration appears to be invalid:\n\n  {msg}\n\nPlease check your config files in your terminal_clock config directory."
                ),
                AssetsLoadError::Toml(e) => write!(
                    f,
                    "A config file could not be parsed:\n\n  {e}\n\nPlease check the file for syntax errors."
                ),
                AssetsLoadError::Io(e) => write!(f, "A config file could not be read:\n\n  {e}"),
            },
            AppError::Io(e) => write!(f, "Terminal error: {e}"),
            AppError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for AppError {}

impl From<AssetsLoadError> for AppError {
    fn from(e: AssetsLoadError) -> Self {
        AppError::Config(e)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Other(s)
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        AppError::Other(e.to_string())
    }
}
