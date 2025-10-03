use ratatui::style::{Color, Style, palette::tailwind};
use std::collections::{HashMap, HashSet};

pub const FALLBACK_COLOR_THEME: [Color; 5] = [
    tailwind::SLATE.c300,  // SchemeColor::Forground
    tailwind::GRAY.c500,   // SchemeColor::Background
    tailwind::CYAN.c500,   // SchemeColor::Selection
    tailwind::RED.c500,    // SchemeColor::Accent
    tailwind::PURPLE.c500, // SchemeColor::Borders
];

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ThemeColor {
    Foreground,
    Background,
    Selection,
    Accent,
    Borders,
}

#[derive(Clone)]
pub struct ColorTheme {
    pub name: String,
    pub colors: HashMap<ThemeColor, Color>,
    pub transparent_colors: HashSet<ThemeColor>,
}

impl ColorTheme {
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get(&self, key: &ThemeColor) -> &Color {
        self.colors
            .get(key)
            .unwrap_or(&FALLBACK_COLOR_THEME[key.clone() as usize])
    }

    pub fn update(&mut self, key: ThemeColor, new_value: Color) {
        self.colors.insert(key, new_value);
    }

    pub fn try_get(&self, key: &ThemeColor) -> Option<&Color> {
        if self.transparent_colors.contains(key) {
            None
        } else {
            Some(self.get(key))
        }
    }

    pub fn default_style(&self) -> Style {
        let fg = *self.get(&ThemeColor::Foreground);
        let bg = self.try_get(&ThemeColor::Background);

        if let Some(bg) = bg {
            Style::default().fg(fg).bg(*bg)
        } else {
            Style::default().fg(fg)
        }
    }
}
