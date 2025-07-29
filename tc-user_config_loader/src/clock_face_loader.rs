use ratatui::style::Color;
use serde::Deserialize;
use std::str::FromStr;
use tc_models::{
    analog_clock::AnalogClock,
    clock::Clock,
    color_clock::ColorClock,
    digital_clock::DigitalClock,
    helper::{TimeUnit, generate_binary_led_coords},
};

#[derive(Deserialize)]
struct ClockFile {
    clock_type: String,
    settings: ClockConfig,
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum ClockConfig {
    ColorClock(ColorClockConfig),
    DigitalClock(DigitalClockConfig),
    AnalogClock(AnalogClockConfig),
}

#[derive(Deserialize)]
pub struct ColorClockConfig {
    pub hour: String,
    pub minutes: String,
    pub seconds: String,
    pub hour_coords: Vec<Vec<(u32, u32)>>,
    pub minute_coords: Vec<Vec<(u32, u32)>>,
    pub second_coords: Vec<Vec<(u32, u32)>>,
    pub accent_color: String,
}

impl From<ColorClockConfig> for ColorClock {
    fn from(config: ColorClockConfig) -> ColorClock {
        ColorClock::new(
            config.hour,
            config.minutes,
            config.seconds,
            config.hour_coords,
            config.minute_coords,
            config.second_coords,
            Color::from_str(config.accent_color.as_str()).unwrap_or(Color::White),
        )
    }
}

#[derive(Deserialize)]
pub struct DigitalClockConfig {
    // pub numbers: [&'static str; 10],
    // pub seperator: &'static str,
}

#[derive(Deserialize)]
pub struct AnalogClockConfig {
    // hour_hand_frames: Vec<&'static str>,
    // minute_hand_frames: Vec<&'static str>,
    // second_hand_frames: Vec<&'static str>,
    //
    // clock_base: &'static str,
    clock_center: [u32; 2],
    hour_center: [u32; 2],
    minute_center: [u32; 2],
    seconds_center: [u32; 2],
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    pub fn load_clockface(&self) -> Box<dyn Clock> {
        // FIX: replace this later
        let hour: String =
            include_str!("../../tc-default_themes/src/ascii_art/temple/H_temple.ascii").to_string();
        let minutes: String =
            include_str!("../../tc-default_themes/src/ascii_art/temple/MS_temple.ascii")
                .to_string();
        let seconds = minutes.clone();

        let led_bit_mapping_hour_face: [&[(u8, (u32, u32))]; 2] = [
            &[(1, (11, 1)), (2, (8, 0))],
            &[
                (1, (11, 19)),
                (2, (9, 19)),
                (4, (7, 19)),
                (8, (5, 19)),
                (8, (5, 8)),
            ],
        ];

        let led_bit_mapping_minute_face: [&[(u8, (u32, u32))]; 2] = [
            &[(1, (11, 1)), (2, (9, 1)), (4, (7, 0))],
            &[(1, (11, 19)), (2, (9, 19)), (4, (7, 19)), (8, (5, 19))],
        ];

        let led_coords_hours = generate_binary_led_coords(
            led_bit_mapping_hour_face[0],
            led_bit_mapping_hour_face[1],
            &[(0, 14)],
            TimeUnit::Hours,
        );
        let led_coords_minutes = generate_binary_led_coords(
            led_bit_mapping_minute_face[0],
            led_bit_mapping_minute_face[1],
            &[(0, 14)],
            TimeUnit::Minutes,
        );

        Box::new(ColorClock::new(
            hour,
            minutes,
            seconds,
            led_coords_hours,
            led_coords_minutes.clone(),
            led_coords_minutes,
            Color::Red,
        ))
    }

    fn from(config: ClockConfig) {
        todo!()
    }
}
