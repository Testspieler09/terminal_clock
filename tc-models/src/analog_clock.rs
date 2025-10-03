use crate::{
    clock::{ClockBehaviour, TimeFormat},
    color_theme::ColorTheme,
};
use ratatui::widgets::Paragraph;

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

    format: Option<TimeFormat>,
}

impl AnalogClock {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl ClockBehaviour for AnalogClock {
    // TODO: add a default implementaton of the func here
    fn generate_clock_face_with_dimensions(
        &self,
        theme: &ColorTheme,
    ) -> (Paragraph<'_>, usize, usize) {
        todo!()
    }

    fn set_clock_format_to(&mut self, fmt: TimeFormat) {
        self.format = Some(fmt);
    }
}
