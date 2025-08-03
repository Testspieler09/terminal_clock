use crate::colorscheme::ColorScheme;
use ratatui::widgets::Paragraph;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimeFormat {
    Hms,
    Hm,
    Mhs,
}

impl std::str::FromStr for TimeFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HH:MM:SS" => Ok(TimeFormat::Hms),
            "HH:MM" => Ok(TimeFormat::Hm),
            "MM:HH:SS" => Ok(TimeFormat::Mhs),
            _ => Err(()),
        }
    }
}

pub trait Clock {
    fn draw_clockface(&self, scheme: &ColorScheme) -> (Paragraph, usize, usize);
}
