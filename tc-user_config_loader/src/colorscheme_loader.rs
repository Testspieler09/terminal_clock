use crate::{LoaderResult, default_themes::COLORSCHEMES, get_user_config_path};
use ratatui::style::Color;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tc_models::colorscheme::{ColorScheme, FALLBACK_COLORSCHEME, SchemeColor};

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
            SchemeColor::Foreground,
            parse_color(
                config.foreground,
                FALLBACK_COLORSCHEME[SchemeColor::Foreground as usize],
            ),
        );
        colors.insert(
            SchemeColor::Selection,
            parse_color(
                config.selection,
                FALLBACK_COLORSCHEME[SchemeColor::Selection as usize],
            ),
        );
        colors.insert(
            SchemeColor::Comment,
            parse_color(
                config.comment,
                FALLBACK_COLORSCHEME[SchemeColor::Comment as usize],
            ),
        );
        colors.insert(
            SchemeColor::Red,
            parse_color(config.red, FALLBACK_COLORSCHEME[SchemeColor::Red as usize]),
        );
        colors.insert(
            SchemeColor::Orange,
            parse_color(
                config.orange,
                FALLBACK_COLORSCHEME[SchemeColor::Orange as usize],
            ),
        );
        colors.insert(
            SchemeColor::Yellow,
            parse_color(
                config.yellow,
                FALLBACK_COLORSCHEME[SchemeColor::Yellow as usize],
            ),
        );
        colors.insert(
            SchemeColor::Green,
            parse_color(
                config.green,
                FALLBACK_COLORSCHEME[SchemeColor::Green as usize],
            ),
        );
        colors.insert(
            SchemeColor::Purple,
            parse_color(
                config.purple,
                FALLBACK_COLORSCHEME[SchemeColor::Purple as usize],
            ),
        );
        colors.insert(
            SchemeColor::Cyan,
            parse_color(
                config.cyan,
                FALLBACK_COLORSCHEME[SchemeColor::Cyan as usize],
            ),
        );
        colors.insert(
            SchemeColor::Pink,
            parse_color(
                config.pink,
                FALLBACK_COLORSCHEME[SchemeColor::Pink as usize],
            ),
        );

        ColorScheme {
            name: config.name.expect("Expected the theme to have a name."),
            colors,
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
