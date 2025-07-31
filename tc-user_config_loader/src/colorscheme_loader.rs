use crate::{ColorSchemeLoadError, LoaderResult};
use ratatui::style::Color;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr};
use tc_models::colorscheme::ColorScheme;

#[derive(Deserialize)]
pub(crate) struct SchemeConfig {
    pub name: Option<String>,
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
        fn parse_color(opt: Option<String>, fallback: Color) -> Color {
            opt.as_deref()
                .and_then(|s| Color::from_str(s).ok())
                .unwrap_or(fallback)
        }

        let mut colors = HashMap::new();

        colors.insert(
            "foreground".to_string(),
            parse_color(config.foreground, Color::White),
        );
        colors.insert(
            "selection".to_string(),
            parse_color(config.selection, Color::DarkGray),
        );
        colors.insert(
            "comment".to_string(),
            parse_color(config.comment, Color::Gray),
        );
        colors.insert("red".to_string(), parse_color(config.red, Color::Red));
        colors.insert(
            "orange".to_string(),
            parse_color(config.orange, Color::LightRed),
        );
        colors.insert(
            "yellow".to_string(),
            parse_color(config.yellow, Color::Yellow),
        );
        colors.insert("green".to_string(), parse_color(config.green, Color::Green));
        colors.insert(
            "purple".to_string(),
            parse_color(config.purple, Color::Magenta),
        );
        colors.insert("cyan".to_string(), parse_color(config.cyan, Color::Cyan));
        colors.insert(
            "pink".to_string(),
            parse_color(config.pink, Color::LightMagenta),
        );

        ColorScheme {
            name: config.name.expect("Expected the theme to have a name."),
            colors,
        }
    }
}

pub struct ColorSchemeLoader;

impl ColorSchemeLoader {
    fn load_user_themes() -> LoaderResult<Vec<ColorScheme>> {
        let folder_path = Self::get_user_config_path()?;

        let toml_count = std::fs::read_dir(&folder_path)?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext_str| ext_str.eq_ignore_ascii_case("toml"))
                    .unwrap_or(false)
            })
            .count();
        let mut themes = Vec::with_capacity(toml_count);

        if !folder_path.exists() {
            return Ok(themes);
        }

        for entry in std::fs::read_dir(folder_path)? {
            let path = entry?.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = std::fs::read_to_string(&path)?;
            let mut parsed_theme: SchemeConfig = toml::from_str(&content)?;

            if parsed_theme.name.is_none() {
                let filename = path.file_name().and_then(|f| f.to_str());
                parsed_theme.name = filename.map(|s| s.to_string());
            }

            themes.push(parsed_theme.into());
        }

        Ok(themes)
    }

    fn get_user_config_path() -> LoaderResult<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")
                .map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
            Ok(PathBuf::from(appdata).join("terminal_clock").join("themes"))
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let home = std::env::var("HOME")
                .map_err(|e| ColorSchemeLoadError::ConfigPath(e.to_string()))?;
            Ok(PathBuf::from(home)
                .join(".config")
                .join("terminal_clock")
                .join("themes"))
        }
    }

    pub fn load_colorschemes() -> LoaderResult<Vec<ColorScheme>> {
        let mut themes = Vec::new();
        // TODO: load default themes within this crate
        if let Ok(user_theme) = Self::load_user_themes() {
            themes.extend(user_theme);
        }
        Ok(themes)
    }
}
