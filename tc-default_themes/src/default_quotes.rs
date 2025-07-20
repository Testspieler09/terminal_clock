use tc_user_config_loader::quote::Quote;

pub(crate) fn init_default_quotes() -> Vec<Quote> {
    vec![
        Quote::from_string("The only limit to our realization of tomorrow is our doubts of today."),
        Quote::from_string(
            "The future belongs to those who believe in the beauty of their dreams.",
        ),
        Quote::from_string("Do not watch the clock; do what it does. Keep going."),
        Quote::from_string("Time is what we want most, but what we use worst."),
        Quote::from_string("Success is not the key to happiness. Happiness is the key to success."),
    ]
}
