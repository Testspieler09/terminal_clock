use crate::{LoaderResult, clock_face_loader, default_themes::CLOCK_FACES};
use ratatui::style::Color;
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tc_models::{
    analog_clock::AnalogClock,
    clock::{Clock, TimeFormat},
    color_clock::ColorClock,
    color_theme::FALLBACK_COLOR_THEME,
    digital_clock::DigitalClock,
    helper::{TimeUnit, generate_binary_led_coords},
};

#[derive(Deserialize)]
#[serde(tag = "clock_type", content = "config")]
pub enum ClockConfig {
    ColorClock(Box<ColorClockConfig>),
    DigitalClock(Box<DigitalClockConfig>),
    AnalogClock(Box<AnalogClockConfig>),
}

impl From<ClockConfig> for Arc<dyn Clock> {
    fn from(config: ClockConfig) -> Self {
        match config {
            ClockConfig::ColorClock(c) => Arc::new(ColorClock::from(*c)),
            ClockConfig::AnalogClock(c) => Arc::new(AnalogClock::from(*c)),
            ClockConfig::DigitalClock(c) => Arc::new(DigitalClock::from(*c)),
        }
    }
}

#[derive(Deserialize)]
pub struct ColorClockConfig {
    pub hour: String,
    pub minutes: String,
    pub seconds: String,
    pub hour_coords: [Vec<(u8, (u32, u32))>; 2],
    pub minute_coords: [Vec<(u8, (u32, u32))>; 2],
    pub second_coords: [Vec<(u8, (u32, u32))>; 2],
    pub always_on_coords: Vec<(u32, u32)>,
    pub accent_color: Option<String>,
    pub format: Option<TimeFormat>,
}

impl From<ColorClockConfig> for ColorClock {
    fn from(config: ColorClockConfig) -> ColorClock {
        let hour_coords = generate_binary_led_coords(
            &config.hour_coords[0],
            &config.hour_coords[1],
            config.always_on_coords.as_slice(),
            TimeUnit::Hours,
        );
        let minute_coords = generate_binary_led_coords(
            &config.minute_coords[0],
            &config.minute_coords[1],
            config.always_on_coords.as_slice(),
            TimeUnit::Minutes,
        );
        let second_coords = generate_binary_led_coords(
            &config.second_coords[0],
            &config.second_coords[1],
            config.always_on_coords.as_slice(),
            TimeUnit::Seconds,
        );
        let accent_color = config
            .accent_color
            .map(|color| Color::from_str(&color).unwrap_or(FALLBACK_COLOR_THEME[0]));

        ColorClock::new(
            config.hour,
            config.minutes,
            config.seconds,
            hour_coords,
            minute_coords,
            second_coords,
            accent_color,
            config.format,
        )
    }
}

#[derive(Deserialize)]
pub struct DigitalClockConfig {
    // pub numbers: [&'static str; 10],
    // pub seperator: &'static str,
}

impl From<DigitalClockConfig> for DigitalClock {
    fn from(config: DigitalClockConfig) -> DigitalClock {
        todo!()
    }
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

impl From<AnalogClockConfig> for AnalogClock {
    fn from(config: AnalogClockConfig) -> AnalogClock {
        todo!()
    }
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    fn load_user_clockfaces() -> LoaderResult<Vec<Box<dyn Clock>>> {
        todo!()
    }

    pub fn load_clockfaces() -> LoaderResult<Vec<Arc<dyn Clock>>> {
        let mut clock_faces = Vec::new();
        for clock_face in CLOCK_FACES {
            let clock_config: ClockConfig = toml::from_str(clock_face)?;
            clock_faces.push(clock_config.into());
        }
        // clock_faces.extend(Self::load_user_clockfaces()?);
        Ok(clock_faces)
    }
}
