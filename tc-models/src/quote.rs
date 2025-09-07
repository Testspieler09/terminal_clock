use crate::color_theme::{ColorTheme, ThemeColor};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

pub struct Quote {
    pub text: String,

    /// None will use the default accent color of the color theme
    pub accent_color: Option<Color>,
}

impl Quote {
    pub fn new(text: impl Into<String>, color: Option<Color>) -> Self {
        Self {
            text: text.into(),
            accent_color: color,
        }
    }

    pub fn from_string(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            accent_color: None,
        }
    }

    /// Returns a Paragraph widget to render the quote
    pub fn render(&self, theme: &ColorTheme) -> Paragraph<'_> {
        if let Some(color) = self.accent_color {
            Paragraph::new(Span::styled(
                self.text.clone(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
        } else {
            Paragraph::new(Span::styled(
                &self.text,
                Style::default()
                    .fg(*theme.get(&ThemeColor::Accent))
                    .add_modifier(Modifier::BOLD),
            ))
        }
    }
}
