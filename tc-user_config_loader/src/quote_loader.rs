use std::str::FromStr;

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::quote::Quote;

use crate::{
    LoaderResult, default_themes::default_quotes::init_default_quotes, get_user_config_path,
};

#[derive(Deserialize)]
pub struct QuoteConfig {
    pub author: Option<String>,
    pub text: String,
    pub accent_color: Option<String>,
}

impl From<QuoteConfig> for Quote {
    fn from(conf: QuoteConfig) -> Self {
        let accent_color = match conf.accent_color {
            Some(c) => Color::from_str(&c).ok(),
            None => None,
        };

        Self {
            author: conf.author,
            text: conf.text,
            accent_color,
        }
    }
}

pub struct QuoteLoader;

impl QuoteLoader {
    fn load_user_quotes() -> LoaderResult<Vec<Quote>> {
        let conf_path = get_user_config_path()?.join("quotes");

        let toml_count = std::fs::read_dir(&conf_path)?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext_str| ext_str.eq_ignore_ascii_case("toml"))
                    .unwrap_or(false)
            })
            .count();
        let mut quotes = Vec::with_capacity(toml_count);

        if !conf_path.exists() {
            return Ok(quotes);
        }

        for entry in std::fs::read_dir(conf_path)? {
            let path = entry?.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = std::fs::read_to_string(&path)?;
            let parsed_quote: QuoteConfig = toml::from_str(&content)?;

            if let Some(ref color) = parsed_quote.accent_color
                && Color::from_str(&color).is_err()
            {
                println!("Accent color could not be parsed: {}", color);
                continue;
            }

            quotes.push(parsed_quote.into());
        }

        Ok(quotes)
    }

    pub fn load_quotes() -> LoaderResult<Vec<Quote>> {
        let mut quotes = init_default_quotes();

        if let Ok(user_quotes) = Self::load_user_quotes() {
            quotes.extend(user_quotes);
        }

        Ok(quotes)
    }
}
