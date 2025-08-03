use std::sync::Arc;
use tc_models::quote::Quote;

pub(crate) fn init_default_quotes() -> Vec<Arc<Quote>> {
    vec![
        Quote::from_string("The only limit to our realization of tomorrow is our doubts of today.")
            .into(),
        Quote::from_string(
            "The future belongs to those who believe in the beauty of their dreams.",
        )
        .into(),
        Quote::from_string("Do not watch the clock; do what it does. Keep going.").into(),
        Quote::from_string("Time is what we want most, but what we use worst.").into(),
        Quote::from_string("Success is not the key to happiness. Happiness is the key to success.")
            .into(),
        Quote::from_string("Fall down seven times, stand up eight").into(),
    ]
}
