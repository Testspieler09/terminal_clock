use std::sync::Arc;
use tc_models::quote::Quote;

pub(crate) fn init_default_quotes() -> Vec<Arc<Quote>> {
    vec![
        Quote::from_string("\"The only limit to our realization of tomorrow is our doubts of today.\" ― Franklin D. Roosevelt").into(),
        Quote::from_string("\"The future belongs to those who believe in the beauty of their dreams.\" ― Eleanor Roosevelt").into(),
        Quote::from_string("\"Do not watch the clock; do what it does. Keep going.\" ― Sam Levenson").into(),
        Quote::from_string("\"Time is what we want most, but what we use worst.\" ― William Penn").into(),
        Quote::from_string("\"Success is not the key to happiness. Happiness is the key to success.\" ― Albert Schweitzer").into(),
        Quote::from_string("\"Fall down seven times, stand up eight.\"").into(),
        Quote::from_string("\"Time you enjoy wasting is not wasted time.\" ― Marthe Troly-Curtin").into(),
        Quote::from_string("\"Yesterday is gone. Tomorrow has not yet come. We have only today. Let us begin.\" ― Mother Teresa").into(),
        Quote::from_string("\"Don't spend time beating on a wall, hoping to transform it into a door.\" ― Coco Chanel").into(),
        Quote::from_string("\"They always say time changes things, but you actually have to change them yourself.\" ― Andy Warhol").into(),
    ]
}
