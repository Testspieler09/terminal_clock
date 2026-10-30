use std::str::FromStr;

use ratatui::style::Color;

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
    let theme: ColorTheme = make_config("t", Some("not-a-color"), None, None, None, None).into();

    assert_eq!(
        theme.get(&ThemeColor::Foreground),
        ThemeColor::Foreground.fallback()
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
        theme.get(&ThemeColor::Foreground),
        ThemeColor::Foreground.fallback()
    );
    assert_eq!(
        theme.get(&ThemeColor::Selection),
        ThemeColor::Selection.fallback()
    );
    assert_eq!(
        theme.get(&ThemeColor::Accent),
        ThemeColor::Accent.fallback()
    );
    assert_eq!(
        theme.get(&ThemeColor::Borders),
        ThemeColor::Borders.fallback()
    );
}
