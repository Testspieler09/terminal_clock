use tc_models::quote::Quote;

pub(crate) fn init_default_quotes() -> Vec<Quote> {
    vec![
        Quote::new(
            Some("Franklin D. Roosevelt".to_string()),
            "The only limit to our realization of tomorrow is our doubts of today.",
            None,
        ),
        Quote::new(
            Some("Eleanor Roosevelt".to_string()),
            "The future belongs to those who believe in the beauty of their dreams.",
            None,
        ),
        Quote::new(
            Some(" Sam Levenson".to_string()),
            "Do not watch the clock; do what it does. Keep going.",
            None,
        ),
        Quote::new(
            Some("William Penn".to_string()),
            "Time is what we want most, but what we use worst.",
            None,
        ),
        Quote::new(
            Some("Albert Schweitzer".to_string()),
            "Success is not the key to happiness. Happiness is the key to success.",
            None,
        )
        .into(),
        Quote::from_string("Fall down seven times, stand up eight."),
        Quote::new(
            Some("Marthe Troly-Curtin".to_string()),
            "Time you enjoy wasting is not wasted time.",
            None,
        ),
        Quote::new(
            Some("Mother Teresa".to_string()),
            "Yesterday is gone. Tomorrow has not yet come. We have only today. Let us begin.",
            None,
        ),
        Quote::new(
            Some("Coco Chanel".to_string()),
            "Don't spend time beating on a wall, hoping to transform it into a door.",
            None,
        ),
        Quote::new(
            Some("Andy Warhol".to_string()),
            "They always say time changes things, but you actually have to change them yourself.",
            None,
        ),
    ]
}
