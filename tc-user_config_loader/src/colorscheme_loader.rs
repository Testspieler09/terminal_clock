use crate::colorscheme::ColorScheme;
use crate::error::ColorSchemeLoadError;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct ThemeFile {
    colorscheme: SchemeConfig,
}

#[derive(Deserialize)]
pub(crate) struct SchemeConfig {
    pub name: String, // TODO: defaults to filename
    pub foreground: Option<String>,
    pub selection: Option<String>,
    pub comment: Option<String>,
    pub red: Option<String>,
    pub orange: Option<String>,
    pub yellow: Option<String>,
    pub green: Option<String>,
    pub purple: Option<String>,
    pub cyan: Option<String>,
    pub pink: Option<String>,
}

pub struct ColorSchemeLoader;

impl ColorSchemeLoader {
    fn load_user_themes() -> Result<Option<ColorScheme>, ColorSchemeLoadError> {
        let path = Self::get_user_config_path()?;

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(path)?;
        let parsed: ThemeFile = toml::from_str(&content)?;
        Ok(Some(parsed.colorscheme.into()))
    }

    fn get_user_config_path() -> Result<PathBuf, ColorSchemeLoadError> {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")
                .map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
            Ok(PathBuf::from(appdata)
                .join("terminal_clock")
                .join("theme.toml"))
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let home = std::env::var("HOME")
                .map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
            Ok(PathBuf::from(home)
                .join(".config")
                .join("terminal_clock")
                .join("theme.toml"))
        }
    }

    pub fn load_colorschemes() -> Result<Vec<ColorScheme>, ColorSchemeLoadError> {
        let mut themes = Vec::new();
        if let Some(user_theme) = Self::load_user_themes()? {
            themes.push(user_theme);
        }
        Ok(themes)
    }
}
