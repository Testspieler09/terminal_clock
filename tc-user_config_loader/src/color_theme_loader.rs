use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
};

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::color_theme::{ColorTheme, FALLBACK_COLOR_THEME, ThemeColor};

use crate::{LoaderResult, bundled::COLOR_THEMES};

#[derive(Deserialize)]
pub struct ThemeConfig {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(
        name: &str,
        foreground: Option<&str>,
        background: Option<&str>,
        selection: Option<&str>,
        accent: Option<&str>,
        borders: Option<&str>,
    ) -> ThemeConfig {
        ThemeConfig {
            name: Some(name.to_string()),
            foreground: foreground.map(str::to_string),
            background: background.map(str::to_string),
            selection: selection.map(str::to_string),
            accent: accent.map(str::to_string),
            borders: borders.map(str::to_string),
        }
    }

    #[test]
    fn valid_colors_are_parsed() {
        let theme: ColorTheme = make_config(
            "t",
            Some("#ff0000"),
            Some("#00ff00"),
            Some("#0000ff"),
            Some("#ffffff"),
            Some("#000000"),
        )
        .into();

        assert_eq!(
            *theme.get(&ThemeColor::Foreground),
            Color::from_str("#ff0000").unwrap()
        );
        assert_eq!(
            *theme.get(&ThemeColor::Selection),
            Color::from_str("#0000ff").unwrap()
        );
    }

    #[test]
    fn invalid_color_falls_back_to_default() {
        let theme: ColorTheme =
            make_config("t", Some("not-a-color"), None, None, None, None).into();

        assert_eq!(
            *theme.get(&ThemeColor::Foreground),
            FALLBACK_COLOR_THEME[ThemeColor::Foreground as usize]
        );
    }

    #[test]
    fn missing_background_is_transparent() {
        let theme: ColorTheme = make_config("t", None, None, None, None, None).into();

        assert!(theme.transparent_colors.contains(&ThemeColor::Background));
        assert!(theme.try_get(&ThemeColor::Background).is_none());
    }

    #[test]
    fn valid_background_is_not_transparent() {
        let theme: ColorTheme = make_config("t", None, Some("#112233"), None, None, None).into();

        assert!(!theme.transparent_colors.contains(&ThemeColor::Background));
        assert!(theme.try_get(&ThemeColor::Background).is_some());
    }

    #[test]
    fn all_none_fields_use_fallbacks() {
        let theme: ColorTheme = make_config("t", None, None, None, None, None).into();

        assert_eq!(
            *theme.get(&ThemeColor::Foreground),
            FALLBACK_COLOR_THEME[ThemeColor::Foreground as usize]
        );
        assert_eq!(
            *theme.get(&ThemeColor::Selection),
            FALLBACK_COLOR_THEME[ThemeColor::Selection as usize]
        );
        assert_eq!(
            *theme.get(&ThemeColor::Accent),
            FALLBACK_COLOR_THEME[ThemeColor::Accent as usize]
        );
        assert_eq!(
            *theme.get(&ThemeColor::Borders),
            FALLBACK_COLOR_THEME[ThemeColor::Borders as usize]
        );
    }
}

pub struct ColorThemeLoader;

impl ColorThemeLoader {
    fn load_user_themes(config_path: &Path) -> LoaderResult<Vec<ColorTheme>> {
        let folder_path = config_path.join("themes");

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

            themes.push(parsed_theme.into());
        }

        Ok(themes)
    }

    pub fn load_color_themes(config_path: &Path) -> LoaderResult<Vec<ColorTheme>> {
        let mut schemes = COLOR_THEMES
            .iter()
            .map(|scheme| {
                let colorscheme: ThemeConfig = toml::from_str(scheme)?;
                Ok(colorscheme.into())
            })
            .collect::<LoaderResult<Vec<_>>>()?;

        if let Ok(user_theme) = Self::load_user_themes(config_path) {
            schemes.extend(user_theme);
        }

        Ok(schemes)
    }
}
