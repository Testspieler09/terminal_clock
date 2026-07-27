pub(crate) const DEFAULT_QUOTES: &str = include_str!("./default_quotes.toml");
pub(crate) const DEFAULT_CONFIG: &str = include_str!("./default_config.toml");

pub(crate) const COLOR_THEMES: [&str; 3] = [
    include_str!("./color_themes/terminal_clock.toml"),
    include_str!("./color_themes/tokyo_night.toml"),
    include_str!("./color_themes/catppuccin_mocha.toml"),
];
pub(crate) const CLOCK_FACES: [&str; 3] = [
    include_str!("./clock_faces/temple.toml"),
    include_str!("./clock_faces/binary_squares.toml"),
    // include_str!("./clock_faces/thermometer.toml"),
    // include_str!("./clock_faces/seven_segment_led.toml"),
    include_str!("./clock_faces/italic_seven_segment.toml"),
];

#[cfg(test)]
mod tests {
    use tc_models::{clock::Clock, color_theme::ColorTheme};

    use super::*;
    use crate::{clock_face_loader::ClockConfig, color_theme_loader::ThemeConfig};

    #[test]
    fn all_bundled_clock_faces_load() {
        for (i, content) in CLOCK_FACES.iter().enumerate() {
            let config: ClockConfig = toml::from_str(content)
                .unwrap_or_else(|e| panic!("CLOCK_FACES[{i}] failed to parse: {e}"));
            let _: Clock = config
                .try_into()
                .unwrap_or_else(|e| panic!("CLOCK_FACES[{i}] failed to convert: {e}"));
        }
    }

    #[test]
    fn all_bundled_themes_load() {
        for (i, content) in COLOR_THEMES.iter().enumerate() {
            let config: ThemeConfig = toml::from_str(content)
                .unwrap_or_else(|e| panic!("COLOR_THEMES[{i}] failed to parse: {e}"));
            let _: ColorTheme = config.into();
        }
    }
}
