pub trait ColorClock {
    // The static ascii art for the clock face
    const HOUR: &'static str;
    const MINUTES: &'static str;
    const SECONDS: &'static str;

    // The position of the characters that are suppsoed
    // to change color to display the time
    // TODO: adjust type
    const LED_COORDS: Vec<u32>;

    // An optional quote
    const QUOTE: Option<&'static str>;

    // TODO: add a default implementaton of the func here
    fn draw_clockface(clock_format: &str);
}
