use crate::clock::Clock;
use ratatui::widgets::Paragraph;

pub struct DigitalClock {
    // The static ascii art for the clock face
    numbers: [&'static str; 10],
    seperator: &'static str,
}

impl Clock for DigitalClock {
    // TODO: add a default implementaton of the func here
    fn draw_clockface(&self, clock_format: &str) -> (Paragraph, usize, usize) {
        todo!()
    }
}
