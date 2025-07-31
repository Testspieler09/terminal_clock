use serde::Deserialize;
use tc_models::quote::Quote;

use crate::LoaderResult;

#[derive(Deserialize)]
pub struct QuoteConfig {
    pub text: Option<String>,
    pub accent_color: Option<String>,
}

pub struct QuoteLoader;

impl QuoteLoader {
    fn load_default_quotes() -> LoaderResult<Vec<Quote>> {
        todo!()
    }

    fn load_users_quotes() -> LoaderResult<Vec<Quote>> {
        todo!()
    }

    pub fn load_quotes(&self) -> LoaderResult<Vec<Quote>> {
        todo!()
    }
}
