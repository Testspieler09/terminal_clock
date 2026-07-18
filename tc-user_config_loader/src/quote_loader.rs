use std::{path::Path, str::FromStr};

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::quote::Quote;

use crate::{LoaderResult, bundled::default_quotes::init_default_quotes};

#[derive(Deserialize)]
pub struct QuoteConfig {
    pub author: Option<String>,
    pub text: String,
    pub accent_color: Option<String>,
    #[serde(default)]
    pub active: bool,
}

#[derive(Deserialize)]
pub struct QuotesConfig {
    pub quote: Vec<QuoteConfig>,
}

impl QuotesConfig {
    pub fn into_quotes(self) -> Vec<Quote> {
        let has_active = self.quote.iter().any(|q| q.active);

        self.quote
            .into_iter()
            .filter(|q| !has_active || q.active)
            .filter_map(|q| {
                let accent_color = match q.accent_color {
                    Some(ref c) => match Color::from_str(c) {
                        Ok(color) => Some(color),
                        Err(_) => return None,
                    },
                    None => None,
                };
                Some(Quote::new(q.author, q.text, accent_color))
            })
            .collect()
    }
}

pub struct QuoteLoader;

impl QuoteLoader {
    fn load_user_quotes(config_path: &Path) -> LoaderResult<Vec<Quote>> {
        let path = config_path.join("quotes.toml");

        if !path.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&path)?;
        let config: QuotesConfig = toml::from_str(&content)?;
        Ok(config.into_quotes())
    }

    pub fn load_quotes(config_path: &Path) -> LoaderResult<Vec<Quote>> {
        let mut quotes = init_default_quotes();

        if let Ok(user_quotes) = Self::load_user_quotes(config_path) {
            quotes.extend(user_quotes);
        }

        Ok(quotes)
    }
}
