pub trait AnalogClock {
    // The static ascii art for the clock face
    const HOUR_HAND_FRAMES: Vec<&'static str>;
    const MINUTE_HAND_FRAMES: Vec<&'static str>;
    const SECOND_HAND_FRAMES: Vec<&'static str>;

    const CLOCK_BASE: &'static str;

    const CLOCK_CENTER: [u32; 2];
    const HOUR_CENTER: [u32; 2];
    const MINUTE_CENTER: [u32; 2];
    const SECONDS_CENTER: [u32; 2];

    // TODO: add a default implementaton of the func here
    fn draw_clockface(clock_format: &str) {
        return;
    }
}
