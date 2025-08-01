use crate::{LoaderResult, clock_face_loader, default_themes::CLOCK_FACES};
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
#[serde(tag = "clock_type", content = "config")]
pub enum ClockConfig {
    ColorClock(ColorClockConfig),
    DigitalClock(DigitalClockConfig),
    AnalogClock(AnalogClockConfig),
}

impl From<ClockConfig> for Box<dyn Clock> {
    fn from(config: ClockConfig) -> Self {
        match config {
            ClockConfig::ColorClock(c) => Box::new(ColorClock::from(c)),
            ClockConfig::AnalogClock(c) => Box::new(AnalogClock::from(c)),
            ClockConfig::DigitalClock(c) => Box::new(DigitalClock::from(c)),
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
    pub accent_color: String,
    pub format: Option<String>,
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

        ColorClock::new(
            config.hour,
            config.minutes,
            config.seconds,
            hour_coords,
            minute_coords,
            second_coords,
            Color::from_str(config.accent_color.as_str()).unwrap_or(Color::White),
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

    pub fn load_clockfaces() -> LoaderResult<Vec<Box<dyn Clock>>> {
        let mut clock_faces = Vec::new();
        for clock_face in CLOCK_FACES {
            let clock_config: ClockConfig = toml::from_str(clock_face)?;
            clock_faces.push(clock_config.into());
        }
        // clock_faces.extend(Self::load_user_clockfaces()?);
        Ok(clock_faces)
    }
}
