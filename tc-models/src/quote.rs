use ratatui::style::Color;

pub struct Quote {
    pub author: Option<String>,
    pub text: String,

    /// None will use the default accent color of the color theme
    pub accent_color: Option<Color>,
}

impl Quote {
    pub fn new(author: Option<String>, text: impl Into<String>, color: Option<Color>) -> Self {
        Self {
            author: author,
            text: text.into(),
            accent_color: color,
        }
    }

    pub fn from_string(text: impl Into<String>) -> Self {
        Self {
            author: None,
            text: text.into(),
            accent_color: None,
        }
    }

    pub fn final_quote_string(&self) -> String {
        let mut final_text = "\"".to_owned() + &self.text.clone() + "\"";
        if let Some(author) = &self.author {
            final_text = final_text + " ― " + &author;
        }

        final_text
    }

    // /// Returns a Paragraph widget to render the quote
    // pub fn render(&self, theme: &ColorTheme) -> Paragraph<'_> {
    //     let mut final_text = "\"".to_owned() + &self.text.clone() + "\"";
    //     if let Some(author) = &self.author {
    //         final_text = final_text + " ― " + &author;
    //     }
    //
    //     if let Some(color) = self.accent_color {
    //         Paragraph::new(Span::styled(
    //             final_text,
    //             Style::default().fg(color).add_modifier(Modifier::BOLD),
    //         ))
    //     } else {
    //         Paragraph::new(Span::styled(
    //             final_text,
    //             Style::default()
    //                 .fg(*theme.get(&ThemeColor::Accent))
    //                 .add_modifier(Modifier::BOLD),
    //         ))
    //     }
    // }
}
