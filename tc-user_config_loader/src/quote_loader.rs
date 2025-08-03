use crate::{LoaderResult, default_themes::default_quotes::init_default_quotes};
use serde::Deserialize;
use std::sync::Arc;
use tc_models::quote::Quote;

#[derive(Deserialize)]
pub struct QuoteConfig {
    pub text: Option<String>,
    pub accent_color: Option<String>,
}

pub struct QuoteLoader;

impl QuoteLoader {
    fn load_users_quotes() -> LoaderResult<Vec<Quote>> {
        todo!()
    }

    pub fn load_quotes() -> LoaderResult<Vec<Arc<Quote>>> {
        let mut quotes = init_default_quotes();
        // quotes.extend(Self::load_users_quotes()?);
        Ok(quotes)
    }
}
