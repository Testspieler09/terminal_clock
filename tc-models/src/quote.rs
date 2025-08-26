use crate::colorscheme::{ColorScheme, FALLBACK_COLORSCHEME, SchemeColor};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

pub struct Quote {
    pub text: String,

    /// None will use the default accent color of the colorscheme
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
    pub fn render(&self, scheme: &ColorScheme) -> Paragraph<'_> {
        if let Some(color) = self.accent_color {
            Paragraph::new(Span::styled(
                self.text.clone(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
        } else {
            Paragraph::new(Span::styled(
                &self.text,
                Style::default()
                    .fg(*scheme
                        .get(&SchemeColor::Cyan)
                        .unwrap_or(&FALLBACK_COLORSCHEME[0]))
                    .add_modifier(Modifier::BOLD),
            ))
        }
    }
}
