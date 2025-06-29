use crate::colorscheme_loader::SchemeConfig;
use ratatui::style::Color;
use std::collections::HashMap;
use std::str::FromStr;

pub struct ColorScheme {
    pub name: String,
    pub colors: HashMap<String, Color>,
}

impl ColorScheme {
    pub fn get(&self, key: &str) -> Option<&Color> {
        self.colors.get(key)
    }
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
