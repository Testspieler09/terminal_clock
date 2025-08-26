use crate::{
    clock::{Clock, TimeFormat},
    colorscheme::{ColorScheme, FALLBACK_COLORSCHEME, SchemeColor},
    helper::{art_block, combine_ascii_art_while_applying_led},
};
use chrono::{Local, Timelike};
use ratatui::{style::Color, widgets::Paragraph};

pub struct ColorClock {
    // The static ascii art for the clock face
    hour: String,
    minutes: String,
    seconds: String,

    // The position of the characters that are suppsoed
    // to change color to display the time
    led_coords_hours: Vec<Vec<(u32, u32)>>,
    led_coords_minutes: Vec<Vec<(u32, u32)>>,
    led_coords_seconds: Vec<Vec<(u32, u32)>>,

    accent_color: Option<Color>,
    format: Option<TimeFormat>,
}

impl ColorClock {
    pub fn new(
        hour: String,
        minutes: String,
        seconds: String,
        led_coords_hours: Vec<Vec<(u32, u32)>>,
        led_coords_minutes: Vec<Vec<(u32, u32)>>,
        led_coords_seconds: Vec<Vec<(u32, u32)>>,
        accent_color: Option<Color>,
        format: Option<TimeFormat>,
    ) -> Self {
        ColorClock {
            hour,
            minutes,
            seconds,
            led_coords_hours,
            led_coords_minutes,
            led_coords_seconds,
            accent_color,
            format,
        }
    }
}

impl Clock for ColorClock {
    fn draw_clockface(&self, scheme: &ColorScheme) -> (Paragraph<'_>, usize, usize) {
        let time_stamp = Local::now();
        let hour_value = time_stamp.hour();
        let minute_value = time_stamp.minute();
        let second_value = time_stamp.second();

        static EMPTY_COORDS: &Vec<Vec<(u32, u32)>> = &Vec::new();
        let empty_block = art_block("", EMPTY_COORDS, 0);

        let result = match self.format {
            Some(TimeFormat::Hms) => (
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                art_block(&self.seconds, &self.led_coords_seconds, second_value),
            ),
            Some(TimeFormat::Mhs) => (
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.seconds, &self.led_coords_seconds, second_value),
            ),
            Some(TimeFormat::Hm) | None => (
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                empty_block,
            ),
        };

        let color = if let Some(color) = self.accent_color {
            color
        } else {
            *scheme
                .get(&SchemeColor::Cyan)
                .unwrap_or(&FALLBACK_COLORSCHEME[SchemeColor::Cyan as usize])
        };

        combine_ascii_art_while_applying_led(&result.0, &result.1, &result.2, color)
    }
}
