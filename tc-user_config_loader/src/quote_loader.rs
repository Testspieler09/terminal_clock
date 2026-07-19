use std::{path::Path, str::FromStr};

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::quote::Quote;

use crate::{AssetsLoadError, LoaderResult, bundled::DEFAULT_QUOTES};

#[cfg(test)]
#[path = "quote_loader_test.rs"]
mod tests;

#[derive(Deserialize)]
pub struct QuoteConfig {
    pub author: Option<String>,
    pub text: String,
    pub accent_color: Option<String>,
    #[serde(default)]
    pub initial: bool,
}

#[derive(Deserialize)]
pub struct QuotesConfig {
    pub quote: Vec<QuoteConfig>,
}

impl QuotesConfig {
    /// Returns all quotes and the index of the initially shown one.
    ///
    /// Errors if more than one quote is marked `initial = true`.
    pub fn into_quotes_with_initial(self) -> Result<(Vec<Quote>, Option<usize>), String> {
        let initial_indices: Vec<usize> = self
            .quote
            .iter()
            .enumerate()
            .filter(|(_, q)| q.initial)
            .map(|(i, _)| i)
            .collect();

        if initial_indices.len() > 1 {
            return Err(format!(
                "only one quote may be marked initial, but {} are",
                initial_indices.len()
            ));
        }

        let mut initial_idx: Option<usize> = None;
        let mut quotes = Vec::with_capacity(self.quote.len());

        for (i, q) in self.quote.into_iter().enumerate() {
            let accent_color = match q.accent_color {
                Some(ref c) => match Color::from_str(c) {
                    Ok(color) => Some(color),
                    Err(_) => {
                        return Err(format!(
                            "accent_color \"{c}\" for quote \"{}\" is not a valid color — use a hex value like #ff0000 or a named color like \"red\"",
                            q.text
                        ));
                    }
                },
                None => None,
            };
            if q.initial {
                initial_idx = Some(quotes.len());
            }
            let _ = i; // position in config; quotes vec position tracked above
            quotes.push(Quote::new(q.author, q.text, accent_color));
        }

        Ok((quotes, initial_idx))
    }
}

pub struct QuoteLoader;

impl QuoteLoader {
    fn load_user_quotes(config_path: &Path) -> LoaderResult<(Vec<Quote>, Option<usize>)> {
        let path = config_path.join("quotes.toml");

        if !path.exists() {
            return Ok((vec![], None));
        }

        let content = std::fs::read_to_string(&path)?;
        let config: QuotesConfig = toml::from_str(&content)?;
        config
            .into_quotes_with_initial()
            .map_err(AssetsLoadError::InvalidConfig)
    }

    /// Returns all quotes (bundled + user) and the index of the initially shown
    /// quote. The user config takes precedence for the initial index; if neither
    /// specifies one, `None` is returned (caller picks index 0).
    pub fn load_quotes(config_path: &Path) -> LoaderResult<(Vec<Quote>, Option<usize>)> {
        let bundled: QuotesConfig =
            toml::from_str(DEFAULT_QUOTES).expect("bundled default_quotes.toml is invalid");
        let (mut quotes, _) = bundled
            .into_quotes_with_initial()
            .map_err(AssetsLoadError::InvalidConfig)?;

        let user_initial = match Self::load_user_quotes(config_path) {
            Ok((user_quotes, idx)) => {
                let offset = quotes.len();
                quotes.extend(user_quotes);
                idx.map(|i| i + offset)
            }
            Err(_) => None,
        };

        // User config initial takes precedence; bundled quotes never set one
        Ok((quotes, user_initial))
    }
}
