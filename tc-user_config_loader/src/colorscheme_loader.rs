use crate::{LoaderResult, default_themes::COLORSCHEMES, get_user_config_path};
use ratatui::style::Color;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};
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
        let folder_path = get_user_config_path()?;

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

    pub fn load_colorschemes() -> LoaderResult<Vec<ColorScheme>> {
        let mut schemes = Vec::new();

        for scheme in COLORSCHEMES {
            let colorscheme: SchemeConfig = toml::from_str(scheme)?;
            schemes.push(colorscheme.into());
        }
        if let Ok(user_theme) = Self::load_user_themes() {
            schemes.extend(user_theme);
        }

        Ok(schemes)
    }
}
