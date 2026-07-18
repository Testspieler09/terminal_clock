use tc_models::quote::Quote;

use crate::{bundled::DEFAULT_QUOTES, quote_loader::QuotesConfig};

pub(crate) fn init_default_quotes() -> Vec<Quote> {
    let config: QuotesConfig =
        toml::from_str(DEFAULT_QUOTES).expect("bundled default_quotes.toml is invalid");
    config.into_quotes()
}
