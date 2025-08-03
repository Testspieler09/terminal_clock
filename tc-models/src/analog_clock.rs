use crate::{
    clock::{Clock, TimeFormat},
    colorscheme::ColorScheme,
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
}

impl Clock for AnalogClock {
    // TODO: add a default implementaton of the func here
    fn draw_clockface(&self, scheme: &ColorScheme) -> (Paragraph, usize, usize) {
        todo!()
    }
}
