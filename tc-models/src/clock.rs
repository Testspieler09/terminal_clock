use crate::{
    analog_clock::AnalogClock, color_clock::ColorClock, color_theme::ColorTheme,
    digital_clock::DigitalClock,
};
use ratatui::widgets::Paragraph;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Default, Deserialize, Clone, Copy, EnumIter)]
pub enum TimeFormat {
    #[default]
    Hms,
    Hm,
    Mhs,
}

impl TimeFormat {
    pub fn get_str_repr(&self) -> &str {
        match self {
            TimeFormat::Hms => "HH:MM:SS",
            TimeFormat::Hm => "HH:MM",
            TimeFormat::Mhs => "MM:HH:SS",
        }
    }
}

pub trait ClockBehaviour {
    fn generate_clock_face_with_dimensions(
        &self,
        theme: &ColorTheme,
        clock_fmt: TimeFormat,
    ) -> (Paragraph<'_>, usize, usize);
}

#[derive(Clone)]
pub enum Clock {
    Digital(DigitalClock),
    Analog(AnalogClock),
    Color(ColorClock),
}

impl Clock {
    pub fn get_name(&self) -> &str {
        match self {
            Clock::Digital(clock) => clock.get_name(),
            Clock::Analog(clock) => clock.get_name(),
            Clock::Color(clock) => clock.get_name(),
        }
    }
}

impl ClockBehaviour for Clock {
    fn generate_clock_face_with_dimensions(
        &self,
        theme: &ColorTheme,
        clock_fmt: TimeFormat,
    ) -> (Paragraph<'_>, usize, usize) {
        match self {
            Clock::Digital(clock) => clock.generate_clock_face_with_dimensions(theme, clock_fmt),
            Clock::Analog(clock) => clock.generate_clock_face_with_dimensions(theme, clock_fmt),
            Clock::Color(clock) => clock.generate_clock_face_with_dimensions(theme, clock_fmt),
        }
    }
}
