use crate::color_theme::ColorTheme;
use ratatui::widgets::Paragraph;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimeFormat {
    Hms,
    Hm,
    Mhs,
}

pub trait Clock {
    fn draw_clockface(&self, theme: &ColorTheme) -> (Paragraph<'_>, usize, usize);
    fn set_clock_format_to(&mut self, fmt: TimeFormat);
}
