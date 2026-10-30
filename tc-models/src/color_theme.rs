use std::collections::{HashMap, HashSet};

use ratatui::style::{Color, Style, palette::tailwind};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ThemeColor {
    Foreground,
    Background,
    Selection,
    Accent,
    Borders,
}

impl ThemeColor {
    pub fn fallback(self) -> &'static Color {
        match self {
            ThemeColor::Foreground => &tailwind::SLATE.c300,
            ThemeColor::Background => &tailwind::GRAY.c500,
            ThemeColor::Selection => &tailwind::CYAN.c500,
            ThemeColor::Accent => &tailwind::RED.c500,
            ThemeColor::Borders => &tailwind::PURPLE.c500,
        }
    }
}

#[derive(Clone)]
pub struct ColorTheme {
    pub name: String,
    pub colors: HashMap<ThemeColor, Color>,
    pub transparent_colors: HashSet<ThemeColor>,
}

impl ColorTheme {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get(&self, key: &ThemeColor) -> &Color {
        self.colors.get(key).unwrap_or(key.fallback())
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
