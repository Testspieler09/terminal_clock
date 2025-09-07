use crate::{
    clock::{Clock, TimeFormat},
    color_theme::ColorTheme,
};
use ratatui::widgets::Paragraph;

pub struct AnalogClock {
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

impl Clock for AnalogClock {
    // TODO: add a default implementaton of the func here
    fn draw_clockface(&self, theme: &ColorTheme) -> (Paragraph<'_>, usize, usize) {
        todo!()
    }

    fn set_clock_format_to(&mut self, fmt: TimeFormat) {
        self.format = Some(fmt);
    }
}
