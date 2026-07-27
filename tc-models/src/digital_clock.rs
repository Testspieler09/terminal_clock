use chrono::{DateTime, Local};
use ratatui::widgets::Paragraph;

use crate::{
    clock::{ClockBehaviour, TimeFormat},
    color_theme::ColorTheme,
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct DigitalClock {
    name: &'static str,

    // The static ascii art for the clock face
    numbers: [&'static str; 10],
    seperator: &'static str,
}

impl DigitalClock {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl ClockBehaviour for DigitalClock {
    fn generate_clock_face_with_dimensions(
        &self,
        _theme: &ColorTheme,
        _clock_fmt: TimeFormat,
        _now: DateTime<Local>,
    ) -> (Paragraph<'_>, usize, usize) {
        todo!();
    }
}
