use crate::{
    clock::{Clock, TimeFormat},
    color_theme::ColorTheme,
};
use ratatui::widgets::Paragraph;

pub struct DigitalClock {
    // The static ascii art for the clock face
    numbers: [&'static str; 10],
    seperator: &'static str,

    format: Option<TimeFormat>,
}

impl Clock for DigitalClock {
    // TODO: add a default implementaton of the func here
    fn draw_clockface(&self, theme: &ColorTheme) -> (Paragraph<'_>, usize, usize) {
        todo!()
    }

    fn set_clock_format_to(&mut self, fmt: TimeFormat) {
        self.format = Some(fmt);
    }
}
