use std::{path::Path, str::FromStr};

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::quote::Quote;

use crate::{LoaderResult, bundled::DEFAULT_QUOTES};

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
        let config: QuotesConfig =
            toml::from_str(DEFAULT_QUOTES).expect("bundled default_quotes.toml is invalid");
        let mut quotes = config.into_quotes();

        if let Ok(user_quotes) = Self::load_user_quotes(config_path) {
            quotes.extend(user_quotes);
        }

        Ok(quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(quotes: Vec<(&str, bool, Option<&str>)>) -> QuotesConfig {
        QuotesConfig {
            quote: quotes
                .into_iter()
                .map(|(text, active, color)| QuoteConfig {
                    author: None,
                    text: text.to_string(),
                    accent_color: color.map(str::to_string),
                    active,
                })
                .collect(),
        }
    }

    #[test]
    fn no_active_flag_returns_all_quotes() {
        let config = make_config(vec![("a", false, None), ("b", false, None)]);
        let quotes = config.into_quotes();
        assert_eq!(quotes.len(), 2);
    }

    #[test]
    fn active_flag_filters_to_only_active() {
        let config = make_config(vec![
            ("a", false, None),
            ("b", true, None),
            ("c", false, None),
        ]);
        let quotes = config.into_quotes();
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0].text, "b");
    }

    #[test]
    fn multiple_active_flags_returns_all_active() {
        let config = make_config(vec![
            ("a", true, None),
            ("b", false, None),
            ("c", true, None),
        ]);
        let quotes = config.into_quotes();
        assert_eq!(quotes.len(), 2);
    }

    #[test]
    fn invalid_accent_color_skips_quote() {
        let config = make_config(vec![("a", false, Some("not-a-color")), ("b", false, None)]);
        let quotes = config.into_quotes();
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0].text, "b");
    }

    #[test]
    fn valid_accent_color_is_parsed() {
        let config = make_config(vec![("a", false, Some("#ff0000"))]);
        let quotes = config.into_quotes();
        assert_eq!(quotes.len(), 1);
        assert!(quotes[0].accent_color.is_some());
    }

    #[test]
    fn empty_config_returns_empty_vec() {
        let config = QuotesConfig { quote: vec![] };
        assert!(config.into_quotes().is_empty());
    }
}
