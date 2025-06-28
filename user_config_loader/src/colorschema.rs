use ratatui::style::Color;
use std::str::FromStr;

pub struct ColorScheme {
    foreground: Color,
    selection: Color,
    comment: Color,
    red: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    purple: Color,
    cyan: Color,
    pink: Color,
}

impl From<Vec<&str>> for ColorScheme {
    fn from(mut colors: Vec<&str>) -> ColorScheme {
        fn parse_color(value: Option<&str>, fallback: &str) -> Color {
            Color::from_str(value.unwrap_or(fallback)).unwrap_or(Color::White)
        }

        ColorScheme {
            foreground: parse_color(colors.pop(), "#c0caf5"),
            selection: parse_color(colors.pop(), "#283457"),
            comment: parse_color(colors.pop(), "#565f89"),
            red: parse_color(colors.pop(), "#f7768e"),
            orange: parse_color(colors.pop(), "#ff9e64"),
            yellow: parse_color(colors.pop(), "#e0af68"),
            green: parse_color(colors.pop(), "#9ece6a"),
            purple: parse_color(colors.pop(), "#9d7cd8"),
            cyan: parse_color(colors.pop(), "#7dcfff"),
            pink: parse_color(colors.pop(), "#bb9af7"),
        }
    }
}
