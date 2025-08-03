use ratatui::style::Color;
use std::collections::HashMap;

pub const FALLBACK_COLORSCHEME: [Color; 10] = [
    Color::White,
    Color::DarkGray,
    Color::Gray,
    Color::Red,
    Color::LightRed,
    Color::Yellow,
    Color::Green,
    Color::Magenta,
    Color::Cyan,
    Color::LightMagenta,
];

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SchemeColor {
    Foreground,
    Selection,
    Comment,
    Red,
    Orange,
    Yellow,
    Green,
    Purple,
    Cyan,
    Pink,
}

#[derive(Clone)]
pub struct ColorScheme {
    pub name: String,
    pub colors: HashMap<SchemeColor, Color>,
}

impl ColorScheme {
    pub fn get(&self, key: &SchemeColor) -> Option<&Color> {
        self.colors.get(key)
    }
}
