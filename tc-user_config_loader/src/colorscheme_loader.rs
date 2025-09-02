use crate::{LoaderResult, default_themes::COLORSCHEMES, get_user_config_path};
use ratatui::style::Color;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};
use tc_models::colorscheme::{ColorScheme, FALLBACK_COLORSCHEME, SchemeColor};

#[derive(Deserialize)]
pub(crate) struct SchemeConfig {
    pub name: Option<String>,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub selection: Option<String>,
    pub accent: Option<String>,
    pub borders: Option<String>,
}

impl From<SchemeConfig> for ColorScheme {
    fn from(config: SchemeConfig) -> Self {
        fn parse_color(opt: Option<String>, fallback: Color) -> Color {
            opt.as_deref()
                .and_then(|s| Color::from_str(s).ok())
                .unwrap_or(fallback)
        }

        let mut colors = HashMap::new();
        let mut transparent_colors = HashSet::new();

        colors.insert(
            SchemeColor::Foreground,
            parse_color(
                config.foreground,
                FALLBACK_COLORSCHEME[SchemeColor::Foreground as usize],
            ),
        );

        let background_color = config
            .background
            .as_deref()
            .and_then(|s| Color::from_str(s).ok());
        if let Some(bg_color) = background_color {
            colors.insert(SchemeColor::Background, bg_color);
        } else {
            transparent_colors.insert(SchemeColor::Background);
        }

        colors.insert(
            SchemeColor::Selection,
            parse_color(
                config.selection,
                FALLBACK_COLORSCHEME[SchemeColor::Selection as usize],
            ),
        );
        colors.insert(
            SchemeColor::Accent,
            parse_color(
                config.accent,
                FALLBACK_COLORSCHEME[SchemeColor::Accent as usize],
            ),
        );
        colors.insert(
            SchemeColor::Borders,
            parse_color(
                config.borders,
                FALLBACK_COLORSCHEME[SchemeColor::Borders as usize],
            ),
        );

        ColorScheme {
            name: config.name.expect("Expected the theme to have a name."),
            colors,
            transparent_colors,
        }
    }
}

pub struct ColorSchemeLoader;

impl ColorSchemeLoader {
    fn load_user_themes() -> LoaderResult<Vec<Arc<ColorScheme>>> {
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
            let mut parsed_theme: SchemeConfig = toml::from_str(&content)?;

            if parsed_theme.name.is_none() {
                let filename = path.file_name().and_then(|f| f.to_str());
                parsed_theme.name = filename.map(|s| s.to_string());
            }

            themes.push(Arc::new(parsed_theme.into()));
        }

        Ok(themes)
    }

    pub fn load_colorschemes() -> LoaderResult<Vec<Arc<ColorScheme>>> {
        let mut schemes = Vec::new();

        for scheme in COLORSCHEMES {
            let colorscheme: SchemeConfig = toml::from_str(scheme)?;
            schemes.push(Arc::new(colorscheme.into()));
        }
        if let Ok(user_theme) = Self::load_user_themes() {
            schemes.extend(user_theme);
        }

        Ok(schemes)
    }
}
