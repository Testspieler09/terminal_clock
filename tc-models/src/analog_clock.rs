use chrono::{DateTime, Local};
use ratatui::widgets::Paragraph;

use crate::{
    clock::{ClockBehaviour, TimeFormat},
    color_theme::ColorTheme,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct AnalogClock {
    name: &'static str,

    // The static ascii art for the clock face
    hour_hand_frames: Vec<&'static str>,
    minute_hand_frames: Vec<&'static str>,
    second_hand_frames: Vec<&'static str>,

    clock_base: &'static str,

    clock_center: [u32; 2],
    hour_center: [u32; 2],
    minute_center: [u32; 2],
    seconds_center: [u32; 2],
}

impl AnalogClock {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl ClockBehaviour for AnalogClock {
    fn generate_clock_face_with_dimensions(
        &self,
        _theme: &ColorTheme,
        _clock_fmt: TimeFormat,
        _now: DateTime<Local>,
    ) -> (Paragraph<'_>, usize, usize) {
        todo!()
    }
}
