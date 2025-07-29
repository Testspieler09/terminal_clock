use crate::ColorSchemeLoadError;
use ratatui::style::Color;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr};
use tc_models::colorscheme::ColorScheme;

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

impl From<SchemeConfig> for ColorScheme {
    fn from(config: SchemeConfig) -> Self {
        fn parse_color(opt: Option<String>, fallback: &str) -> Color {
            opt.as_deref()
                .and_then(|s| Color::from_str(s).ok())
                .unwrap_or_else(|| Color::from_str(fallback).unwrap())
        }

        let mut colors = HashMap::new();

        colors.insert(
            "foreground".to_string(),
            parse_color(config.foreground, "#c0caf5"),
        );
        colors.insert(
            "selection".to_string(),
            parse_color(config.selection, "#283457"),
        );
        colors.insert(
            "comment".to_string(),
            parse_color(config.comment, "#565f89"),
        );
        colors.insert("red".to_string(), parse_color(config.red, "#f7768e"));
        colors.insert("orange".to_string(), parse_color(config.orange, "#ff9e64"));
        colors.insert("yellow".to_string(), parse_color(config.yellow, "#e0af68"));
        colors.insert("green".to_string(), parse_color(config.green, "#9ece6a"));
        colors.insert("purple".to_string(), parse_color(config.purple, "#9d7cd8"));
        colors.insert("cyan".to_string(), parse_color(config.cyan, "#7dcfff"));
        colors.insert("pink".to_string(), parse_color(config.pink, "#bb9af7"));

        ColorScheme {
            name: config.name,
            colors,
        }
    }
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
