use std::{path::Path, str::FromStr};

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::{
    analog_clock::AnalogClock,
    clock::Clock,
    color_clock::ColorClock,
    color_theme::FALLBACK_COLOR_THEME,
    digital_clock::DigitalClock,
    helper::{TimeUnit, generate_led_coords_to_base},
};

use crate::{
    AssetsLoadError, LoaderResult,
    bundled::CLOCK_FACES,
    configs::{
        analog_clock_config::AnalogClockConfig,
        color_clock_config::{ColorClockConfig, CoordSource},
        digital_clock_config::DigitalClockConfig,
        helper::generate_from_ascii,
    },
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

impl TryFrom<ClockConfig> for Clock {
    type Error = AssetsLoadError;

    fn try_from(config: ClockConfig) -> Result<Self, Self::Error> {
        match config {
            ClockConfig::ColorClock(c) => Ok(Clock::Color(ColorClock::try_from(*c)?)),
            ClockConfig::AnalogClock(c) => Ok(Clock::Analog(AnalogClock::from(*c))),
            ClockConfig::DigitalClock(c) => Ok(Clock::Digital(DigitalClock::from(*c))),
        }
    }
}

impl TryFrom<ColorClockConfig> for ColorClock {
    type Error = AssetsLoadError;

    fn try_from(config: ColorClockConfig) -> Result<ColorClock, AssetsLoadError> {
        let always_on = config.always_on_coords.unwrap_or_default();
        let render_mode = config.render_mode;

        let (hour_coords, minute_coords, second_coords) = match config.coord_source {
            CoordSource::Explicit {
                hour_coords,
                minute_coords,
                second_coords,
            } => (
                generate_led_coords_to_base(
                    &hour_coords[0],
                    &hour_coords[1],
                    &always_on,
                    TimeUnit::Hours,
                    render_mode,
                ),
                generate_led_coords_to_base(
                    &minute_coords[0],
                    &minute_coords[1],
                    &always_on,
                    TimeUnit::Minutes,
                    render_mode,
                ),
                generate_led_coords_to_base(
                    &second_coords[0],
                    &second_coords[1],
                    &always_on,
                    TimeUnit::Seconds,
                    render_mode,
                ),
            ),
            CoordSource::Mapped { mapping } => (
                generate_from_ascii(
                    &config.hour,
                    &mapping,
                    TimeUnit::Hours,
                    render_mode,
                    &always_on,
                )?,
                generate_from_ascii(
                    &config.minutes,
                    &mapping,
                    TimeUnit::Minutes,
                    render_mode,
                    &always_on,
                )?,
                generate_from_ascii(
                    &config.seconds,
                    &mapping,
                    TimeUnit::Seconds,
                    render_mode,
                    &always_on,
                )?,
            ),
        };

        let clock_color = config
            .clock_color
            .map(|c| Color::from_str(&c).unwrap_or(FALLBACK_COLOR_THEME[0]));

        let accent_color = config
            .accent_color
            .map(|c| Color::from_str(&c).unwrap_or(FALLBACK_COLOR_THEME[2]));

        Ok(ColorClock::new(
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
        ))
    }
}

impl From<DigitalClockConfig> for DigitalClock {
    fn from(_config: DigitalClockConfig) -> DigitalClock {
        todo!()
    }
}

impl From<AnalogClockConfig> for AnalogClock {
    fn from(_config: AnalogClockConfig) -> AnalogClock {
        todo!()
    }
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    fn load_user_clockfaces(config_path: &Path) -> LoaderResult<Vec<Clock>> {
        let folder_path = config_path.join("clock_faces");

        if !folder_path.exists() {
            return Ok(vec![]);
        }

        let mut clock_faces = Vec::new();

        for entry in std::fs::read_dir(folder_path)? {
            let path = entry?.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = std::fs::read_to_string(&path)?;
            let mut config: ClockConfig = toml::from_str(&content)?;

            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                config.set_name_if_none(stem.to_string());
            }

            clock_faces.push(config.try_into()?);
        }

        Ok(clock_faces)
    }

    pub fn load_clockfaces(config_path: &Path) -> LoaderResult<Vec<Clock>> {
        let mut clock_faces = CLOCK_FACES
            .iter()
            .map(|clock_face| {
                let clock_config: ClockConfig = toml::from_str(clock_face)?;
                clock_config.try_into()
            })
            .collect::<LoaderResult<Vec<_>>>()?;

        if let Ok(user_clockfaces) = Self::load_user_clockfaces(config_path) {
            clock_faces.extend(user_clockfaces);
        }

        Ok(clock_faces)
    }
}
