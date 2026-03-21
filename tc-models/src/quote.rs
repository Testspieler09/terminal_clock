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
            author,
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
            final_text = final_text + " ― " + author;
        }

        final_text
    }
}
