use crate::{LoaderResult, default_themes::COLOR_THEMES, get_user_config_path};
use ratatui::style::Color;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::{Arc, Mutex},
};
use tc_models::color_theme::{ColorTheme, FALLBACK_COLOR_THEME, ThemeColor};

#[derive(Deserialize)]
pub(crate) struct ThemeConfig {
    pub name: Option<String>,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub selection: Option<String>,
    pub accent: Option<String>,
    pub borders: Option<String>,
}

impl From<ThemeConfig> for ColorTheme {
    fn from(config: ThemeConfig) -> Self {
        fn parse_color(opt: Option<String>, fallback: Color) -> Color {
            opt.as_deref()
                .and_then(|s| Color::from_str(s).ok())
                .unwrap_or(fallback)
        }

        let mut colors = HashMap::new();
        let mut transparent_colors = HashSet::new();

        colors.insert(
            ThemeColor::Foreground,
            parse_color(
                config.foreground,
                FALLBACK_COLOR_THEME[ThemeColor::Foreground as usize],
            ),
        );

        let background_color = config
            .background
            .as_deref()
            .and_then(|s| Color::from_str(s).ok());
        if let Some(bg_color) = background_color {
            colors.insert(ThemeColor::Background, bg_color);
        } else {
            transparent_colors.insert(ThemeColor::Background);
        }

        colors.insert(
            ThemeColor::Selection,
            parse_color(
                config.selection,
                FALLBACK_COLOR_THEME[ThemeColor::Selection as usize],
            ),
        );
        colors.insert(
            ThemeColor::Accent,
            parse_color(
                config.accent,
                FALLBACK_COLOR_THEME[ThemeColor::Accent as usize],
            ),
        );
        colors.insert(
            ThemeColor::Borders,
            parse_color(
                config.borders,
                FALLBACK_COLOR_THEME[ThemeColor::Borders as usize],
            ),
        );

        ColorTheme {
            name: config.name.expect("Expected the theme to have a name."),
            colors,
            transparent_colors,
        }
    }
}

pub struct ColorThemeLoader;

impl ColorThemeLoader {
    fn load_user_themes() -> LoaderResult<Vec<Arc<Mutex<ColorTheme>>>> {
        let folder_path = get_user_config_path()?.join("themes");

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
            let mut parsed_theme: ThemeConfig = toml::from_str(&content)?;

            if parsed_theme.name.is_none() {
                let filename = path.file_name().and_then(|f| f.to_str());
                parsed_theme.name = filename.map(|s| s.to_string());
            }

            themes.push(Arc::new(Mutex::new(parsed_theme.into())));
        }

        Ok(themes)
    }

    pub fn load_color_themes() -> LoaderResult<Vec<Arc<Mutex<ColorTheme>>>> {
        let mut schemes = COLOR_THEMES
            .iter()
            .map(|scheme| {
                let colorscheme: ThemeConfig = toml::from_str(scheme)?;
                Ok(Arc::new(Mutex::new(colorscheme.into())))
            })
            .collect::<LoaderResult<Vec<_>>>()?;

        if let Ok(user_theme) = Self::load_user_themes() {
            schemes.extend(user_theme);
        }

        Ok(schemes)
    }
}
