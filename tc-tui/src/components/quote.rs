use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};

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

    /// Returns a Paragraph widget to render the quote
    pub fn render(&self) -> Paragraph {
        Paragraph::new(Span::styled(
            self.text.clone(),
            Style::default().fg(self.color),
        ))
    }
}
