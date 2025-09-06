use ratatui::style::{Color, Style, palette::tailwind};
use std::collections::{HashMap, HashSet};

pub const FALLBACK_COLORSCHEME: [Color; 5] = [
    tailwind::SLATE.c300,  // SchemeColor::Forground
    tailwind::GRAY.c500,   // SchemeColor::Background
    tailwind::CYAN.c500,   // SchemeColor::Selection
    tailwind::RED.c500,    // SchemeColor::Accent
    tailwind::PURPLE.c500, // SchemeColor::Borders
];

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SchemeColor {
    Foreground,
    Background,
    Selection,
    Accent,
    Borders,
}

pub struct ColorScheme {
    pub name: String,
    pub colors: HashMap<SchemeColor, Color>,
    pub transparent_colors: HashSet<SchemeColor>,
}

impl ColorScheme {
    pub fn get(&self, key: &SchemeColor) -> &Color {
        self.colors
            .get(key)
            .unwrap_or(&FALLBACK_COLORSCHEME[key.clone() as usize])
    }

    pub fn update(&mut self, key: SchemeColor, new_value: Color) {
        self.colors.insert(key, new_value);
    }

    pub fn try_get(&self, key: &SchemeColor) -> Option<&Color> {
        if self.transparent_colors.contains(key) {
            None
        } else {
            Some(self.get(key))
        }
    }

    pub fn default_style(&self) -> Style {
        let fg = *self.get(&SchemeColor::Foreground);
        let bg = self.try_get(&SchemeColor::Background);

        if let Some(bg) = bg {
            Style::default().fg(fg).bg(*bg)
        } else {
            Style::default().fg(fg)
        }
    }
}
