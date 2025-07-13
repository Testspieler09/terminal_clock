use crate::ColorScheme;
use ratatui::style::Color;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn neo_tokyo() -> ColorScheme {
    let mut colors = HashMap::new();
    colors.insert(
        "foreground".to_string(),
        Color::from_str("#c0caf5").unwrap(),
    );
    colors.insert("selection".to_string(), Color::from_str("#283457").unwrap());
    colors.insert("comment".to_string(), Color::from_str("#565f89").unwrap());
    colors.insert("red".to_string(), Color::from_str("#f7768e").unwrap());
    colors.insert("orange".to_string(), Color::from_str("#ff9e64").unwrap());
    colors.insert("yellow".to_string(), Color::from_str("#e0af68").unwrap());
    colors.insert("green".to_string(), Color::from_str("#9ece6a").unwrap());
    colors.insert("purple".to_string(), Color::from_str("#9d7cd8").unwrap());
    colors.insert("cyan".to_string(), Color::from_str("#7dcfff").unwrap());
    colors.insert("pink".to_string(), Color::from_str("#bb9af7").unwrap());

    ColorScheme {
        name: "Neo Tokyo".to_string(),
        colors,
    }
}
