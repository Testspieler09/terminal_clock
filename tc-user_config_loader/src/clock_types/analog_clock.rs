use ratatui::widgets::Paragraph;

use crate::models::clock::Clock;

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
    fn draw_clockface(&self, clock_format: &str) -> (Paragraph, usize, usize) {
        todo!()
    }
}
