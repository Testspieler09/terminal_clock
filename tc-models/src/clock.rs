use crate::colorscheme::ColorScheme;
use ratatui::widgets::Paragraph;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimeFormat {
    Hms,
    Hm,
    Mhs,
}

pub trait Clock {
    fn draw_clockface(&self, scheme: &ColorScheme) -> (Paragraph<'_>, usize, usize);
}
