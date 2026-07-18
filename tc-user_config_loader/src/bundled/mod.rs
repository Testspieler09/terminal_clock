pub(crate) const DEFAULT_QUOTES: &str = include_str!("./default_quotes.toml");
pub(crate) const DEFAULT_CONFIG: &str = include_str!("./default_config.toml");

pub(crate) const COLOR_THEMES: [&str; 3] = [
    include_str!("./color_themes/terminal_clock.toml"),
    include_str!("./color_themes/tokyo_night.toml"),
    include_str!("./color_themes/catppuccin_mocha.toml"),
];
pub(crate) const CLOCK_FACES: [&str; 2] = [
    include_str!("./clock_faces/temple.toml"),
    // include_str!("./clock_faces/thermometer.toml"),
    // include_str!("./clock_faces/seven_segment_led.toml"),
    include_str!("./clock_faces/italic_seven_segment.toml"),
];
