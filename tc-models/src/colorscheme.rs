use ratatui::style::{Color, palette::tailwind};
use std::collections::HashMap;

pub const FALLBACK_COLORSCHEME: [Color; 10] = [
    tailwind::SLATE.c500,
    tailwind::SLATE.c300,
    tailwind::GRAY.c500,
    tailwind::RED.c500,
    tailwind::ROSE.c400,
    tailwind::YELLOW.c500,
    tailwind::GREEN.c500,
    tailwind::PURPLE.c500,
    tailwind::CYAN.c500,
    tailwind::PINK.c500,
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

pub struct ColorScheme {
    pub name: String,
    pub colors: HashMap<SchemeColor, Color>,
}

impl ColorScheme {
    pub fn get(&self, key: &SchemeColor) -> Option<&Color> {
        self.colors.get(key)
    }
}
