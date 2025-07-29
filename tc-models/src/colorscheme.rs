use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ColorScheme {
    pub name: String,
    pub colors: HashMap<String, Color>,
}

impl ColorScheme {
    pub fn get(&self, key: &str) -> Option<&Color> {
        self.colors.get(key)
    }
}
