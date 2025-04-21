pub trait DigitalClock {
    // The static ascii art for the clock face
    const NUMBERS: [&'static str; 10];
    const SEPERATOR: &'static str;

    // An optional quote
    const QUOTE: Option<&'static str>;

    // TODO: add a default implementaton of the func here
    fn draw_clockface(clock_format: &str);
}
