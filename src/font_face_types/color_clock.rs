pub trait ColorClock {
    // The static ascii art for the clock face
    const HOUR: &'static str;
    const MINUTES: &'static str;
    const SECONDS: &'static str;

    // The position of the characters that are suppsoed
    // to change color to display the time
    const LED_COORDS_HOURS: &[&[(u32, u32)]];
    const LED_COORDS_MINUTES: &[&[(u32, u32)]];
    const LED_COORDS_SECONDS: &[&[(u32, u32)]];

    // TODO: add a default implementaton of the func here
    fn draw_clockface(&self, clock_format: &str) {
        return;
    }
}
