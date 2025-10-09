use crate::{LoaderResult, default_themes::CLOCK_FACES};
use ratatui::style::Color;
use serde::Deserialize;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use tc_models::{
    analog_clock::AnalogClock,
    clock::{Clock, TimeFormat},
    color_clock::ColorClock,
    color_theme::FALLBACK_COLOR_THEME,
    digital_clock::DigitalClock,
    display_mode::DisplayMode,
    helper::{TimeUnit, generate_led_coords_to_base},
};

#[derive(Deserialize)]
#[serde(tag = "clock_type", content = "config")]
pub enum ClockConfig {
    ColorClock(Box<ColorClockConfig>),
    DigitalClock(Box<DigitalClockConfig>),
    AnalogClock(Box<AnalogClockConfig>),
}

impl ClockConfig {
    pub fn set_name_if_none(&mut self, new_name: String) {
        match self {
            ClockConfig::AnalogClock(_clock) => {
                todo!()
            }
            ClockConfig::ColorClock(clock) => {
                clock.set_name_if_none(new_name);
            }
            ClockConfig::DigitalClock(_clock) => {
                todo!()
            }
        };
    }
}

impl From<ClockConfig> for Clock {
    fn from(config: ClockConfig) -> Self {
        match config {
            ClockConfig::ColorClock(c) => Clock::Color(ColorClock::from(*c)),
            ClockConfig::AnalogClock(c) => Clock::Analog(AnalogClock::from(*c)),
            ClockConfig::DigitalClock(c) => Clock::Digital(DigitalClock::from(*c)),
        }
    }
}

#[derive(Deserialize)]
pub struct ColorClockConfig {
    pub name: Option<String>,
    pub hour: String,
    pub minutes: String,
    pub seconds: String,
    pub separator: Option<String>,
    pub display_mode: DisplayMode,
    pub hour_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
    pub minute_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
    pub second_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
    pub always_on_coords: Option<Vec<(u32, u32)>>,
    pub clock_color: Option<String>,
    pub accent_color: Option<String>,
    pub format: Option<TimeFormat>,
}

impl ColorClockConfig {
    pub fn set_name_if_none(&mut self, name: String) {
        self.name = Some(name)
    }
}

impl From<ColorClockConfig> for ColorClock {
    fn from(config: ColorClockConfig) -> ColorClock {
        let always_on_coords_slice = if let Some(always_on_coords) = config.always_on_coords {
            always_on_coords
        } else {
            vec![]
        };
        let hour_coords = generate_led_coords_to_base(
            &config.hour_coords[0],
            &config.hour_coords[1],
            always_on_coords_slice.as_slice(),
            TimeUnit::Hours,
            config.display_mode,
        );
        let minute_coords = generate_led_coords_to_base(
            &config.minute_coords[0],
            &config.minute_coords[1],
            always_on_coords_slice.as_slice(),
            TimeUnit::Minutes,
            config.display_mode,
        );
        let second_coords = generate_led_coords_to_base(
            &config.second_coords[0],
            &config.second_coords[1],
            always_on_coords_slice.as_slice(),
            TimeUnit::Seconds,
            config.display_mode,
        );
        let clock_color = config
            .clock_color
            .map(|color| Color::from_str(&color).unwrap_or(FALLBACK_COLOR_THEME[0]));
        let accent_color = config
            .accent_color
            .map(|color| Color::from_str(&color).unwrap_or(FALLBACK_COLOR_THEME[2]));

        ColorClock::new(
            config.name.unwrap(),
            config.hour,
            config.minutes,
            config.seconds,
            config.separator,
            hour_coords,
            minute_coords,
            second_coords,
            clock_color,
            accent_color,
            config.format.unwrap_or(TimeFormat::default()),
        )
    }
}

#[derive(Deserialize)]
pub struct DigitalClockConfig {
    // pub name: String,
    // pub numbers: [&'static str; 10],
    // pub seperator: &'static str,
}

// impl DigitalClockConfig {
//     pub fn set_name_if_not_already_set(&mut self, name: String) {
//         self.name = name
//     }
// }

impl From<DigitalClockConfig> for DigitalClock {
    fn from(config: DigitalClockConfig) -> DigitalClock {
        todo!()
    }
}

#[derive(Deserialize)]
pub struct AnalogClockConfig {
    // name: String,
    //
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

// impl AnalogClockConfig {
//     pub fn set_name_if_not_already_set(&mut self, name: String) {
//         self.name = name
//     }
// }

impl From<AnalogClockConfig> for AnalogClock {
    fn from(config: AnalogClockConfig) -> AnalogClock {
        todo!()
    }
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    fn load_user_clockfaces() -> LoaderResult<Vec<Arc<Mutex<Clock>>>> {
        todo!()
    }

    pub fn load_clockfaces() -> LoaderResult<Vec<Arc<Mutex<Clock>>>> {
        let mut clock_faces = CLOCK_FACES
            .iter()
            .map(|clock_face| {
                let clock_config: ClockConfig = toml::from_str(clock_face)?;
                Ok(Arc::new(Mutex::new(clock_config.into())))
            })
            .collect::<LoaderResult<Vec<_>>>()?;

        // clock_faces.extend(Self::load_user_clockfaces()?);

        Ok(clock_faces)
    }
}
