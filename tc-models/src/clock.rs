use crate::{
    analog_clock::AnalogClock, color_clock::ColorClock, color_theme::ColorTheme,
    digital_clock::DigitalClock, selectable_item::Selectable,
};
use ratatui::widgets::Paragraph;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum TimeFormat {
    Hms,
    Hm,
    Mhs,
}

pub trait ClockBehaviour {
    fn generate_clock_face_with_dimensions(
        &self,
        theme: &ColorTheme,
    ) -> (Paragraph<'_>, usize, usize);
    fn set_clock_format_to(&mut self, fmt: TimeFormat);
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
    ) -> (Paragraph<'_>, usize, usize) {
        match self {
            Clock::Digital(clock) => clock.generate_clock_face_with_dimensions(theme),
            Clock::Analog(clock) => clock.generate_clock_face_with_dimensions(theme),
            Clock::Color(clock) => clock.generate_clock_face_with_dimensions(theme),
        }
    }

    fn set_clock_format_to(&mut self, fmt: TimeFormat) {
        match self {
            Clock::Digital(clock) => {
                clock.set_clock_format_to(fmt);
            }
            Clock::Analog(clock) => {
                clock.set_clock_format_to(fmt);
            }
            Clock::Color(clock) => {
                clock.set_clock_format_to(fmt);
            }
        }
    }
}
