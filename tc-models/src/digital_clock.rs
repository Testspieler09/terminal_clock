use crate::{
    clock::{ClockBehaviour, TimeFormat},
    color_theme::ColorTheme,
};
use ratatui::widgets::Paragraph;

#[derive(Clone, Copy)]
pub struct DigitalClock {
    name: &'static str,

    // The static ascii art for the clock face
    numbers: [&'static str; 10],
    seperator: &'static str,

    format: Option<TimeFormat>,
}

impl DigitalClock {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl ClockBehaviour for DigitalClock {
    // TODO: add a default implementaton of the func here
    fn generate_clock_face_with_dimensions(
        &self,
        theme: &ColorTheme,
    ) -> (Paragraph<'_>, usize, usize) {
        todo!();
    }

    fn set_clock_format_to(&mut self, fmt: TimeFormat) {
        self.format = Some(fmt);
    }
}
