use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

#[derive(Clone)]
pub struct Quote {
    pub text: String,
    pub color: Color,
}

impl Quote {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }

    pub fn from_string(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::Blue,
        }
    }

    /// Returns a Paragraph widget to render the quote
    pub fn render(&self) -> Paragraph {
        Paragraph::new(Span::styled(
            self.text.clone(),
            Style::default().fg(self.color).add_modifier(Modifier::BOLD),
        ))
    }
}
