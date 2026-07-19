use super::*;

fn make_config(quotes: Vec<(&str, bool, Option<&str>)>) -> QuotesConfig {
    QuotesConfig {
        quote: quotes
            .into_iter()
            .map(|(text, initial, color)| QuoteConfig {
                author: None,
                text: text.to_string(),
                accent_color: color.map(str::to_string),
                initial,
            })
            .collect(),
    }
}

#[test]
fn no_initial_flag_loads_all_quotes_with_no_initial() {
    let config = make_config(vec![("a", false, None), ("b", false, None)]);
    let (quotes, initial) = config.into_quotes_with_initial().unwrap();
    assert_eq!(quotes.len(), 2);
    assert_eq!(initial, None);
}

#[test]
fn initial_flag_sets_initial_index_and_all_quotes_are_loaded() {
    let config = make_config(vec![
        ("a", false, None),
        ("b", true, None),
        ("c", false, None),
    ]);
    let (quotes, initial) = config.into_quotes_with_initial().unwrap();
    assert_eq!(quotes.len(), 3);
    assert_eq!(initial, Some(1));
    assert_eq!(quotes[1].text, "b");
}

#[test]
fn multiple_initial_flags_is_an_error() {
    let config = make_config(vec![
        ("a", true, None),
        ("b", false, None),
        ("c", true, None),
    ]);
    assert!(config.into_quotes_with_initial().is_err());
}

#[test]
fn invalid_accent_color_is_an_error() {
    let config = make_config(vec![("a", false, Some("not-a-color")), ("b", false, None)]);
    let result = config.into_quotes_with_initial();
    let err = result.err().unwrap();
    assert!(err.contains("not-a-color"));
    assert!(err.contains("\"a\""));
}

#[test]
fn initial_quote_with_invalid_color_is_an_error() {
    let config = make_config(vec![("a", true, Some("not-a-color")), ("b", false, None)]);
    let err = config.into_quotes_with_initial().err().unwrap();
    assert!(err.contains("not-a-color"));
    assert!(err.contains("\"a\""));
}

#[test]
fn valid_accent_color_is_parsed() {
    let config = make_config(vec![("a", false, Some("#ff0000"))]);
    let (quotes, _) = config.into_quotes_with_initial().unwrap();
    assert_eq!(quotes.len(), 1);
    assert!(quotes[0].accent_color.is_some());
}

#[test]
fn empty_config_returns_empty_vec_and_no_initial() {
    let config = QuotesConfig { quote: vec![] };
    let (quotes, initial) = config.into_quotes_with_initial().unwrap();
    assert!(quotes.is_empty());
    assert_eq!(initial, None);
}

#[test]
fn invalid_accent_color_before_initial_quote_is_an_error() {
    let config = make_config(vec![
        ("a", false, Some("not-a-color")),
        ("b", true, None),
        ("c", false, None),
    ]);
    let result = config.into_quotes_with_initial();
    let err = result.err().unwrap();
    assert!(err.contains("not-a-color"));
    assert!(err.contains("\"a\""));
}
