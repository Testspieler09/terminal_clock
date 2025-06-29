use crate::{
    clock_face_loader::{AnalogClockConfig, ColorClockConfig, DigitalClockConfig},
    clock_types::color_clock::ColorClock,
};
use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use std::str::FromStr;

pub trait Clock {
    fn draw_clockface(&self, clock_format: &str) -> (Paragraph, usize, usize);
}

impl From<ColorClockConfig> for ColorClock {
    fn from(config: ColorClockConfig) -> ColorClock {
        ColorClock::new(
            config.hour,
            config.minutes,
            config.seconds,
            config.hour_coords,
            config.minute_coords,
            config.second_coords,
            Color::from_str(config.accent_color.as_str()).unwrap_or(Color::White),
        )
    }
}
