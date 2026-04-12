use std::str::FromStr;

use ratatui::style::Color;
use serde::Deserialize;
use tc_models::{
    analog_clock::AnalogClock, clock::Clock, color_clock::ColorClock,
    color_theme::FALLBACK_COLOR_THEME, digital_clock::DigitalClock, helper::TimeUnit,
};

use crate::{
    LoaderResult,
    configs::{
        analog_clock_config::AnalogClockConfig, color_clock_config::ColorClockConfig,
        digital_clock_config::DigitalClockConfig, helper::resolve_coords,
    },
    default_themes::CLOCK_FACES,
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

impl From<ColorClockConfig> for ColorClock {
    fn from(config: ColorClockConfig) -> ColorClock {
        let always_on_coords = config.always_on_coords.unwrap_or_default();

        let hour_coords = resolve_coords(
            config.hour_coords,
            &config.hour,
            &config.mapping,
            &always_on_coords,
            TimeUnit::Hours,
            config.render_mode,
        );

        let minute_coords = resolve_coords(
            config.minute_coords,
            &config.minutes,
            &config.mapping,
            &always_on_coords,
            TimeUnit::Minutes,
            config.render_mode,
        );

        let second_coords = resolve_coords(
            config.second_coords,
            &config.seconds,
            &config.mapping,
            &always_on_coords,
            TimeUnit::Seconds,
            config.render_mode,
        );

        let clock_color = config
            .clock_color
            .map(|c| Color::from_str(&c).unwrap_or(FALLBACK_COLOR_THEME[0]));

        let accent_color = config
            .accent_color
            .map(|c| Color::from_str(&c).unwrap_or(FALLBACK_COLOR_THEME[2]));

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
        )
    }
}

// impl From<ColorClockConfig> for ColorClock {
//     fn from(config: ColorClockConfig) -> ColorClock {
//         let always_on_coords_slice = if let Some(always_on_coords) = config.always_on_coords {
//             always_on_coords
//         } else {
//             Vec::with_capacity(0)
//         };
//         let hour_coords = generate_led_coords_to_base(
//             &config.hour_coords[0],
//             &config.hour_coords[1],
//             always_on_coords_slice.as_slice(),
//             TimeUnit::Hours,
//             config.render_mode,
//         );
//         let minute_coords = generate_led_coords_to_base(
//             &config.minute_coords[0],
//             &config.minute_coords[1],
//             always_on_coords_slice.as_slice(),
//             TimeUnit::Minutes,
//             config.render_mode,
//         );
//         let second_coords = generate_led_coords_to_base(
//             &config.second_coords[0],
//             &config.second_coords[1],
//             always_on_coords_slice.as_slice(),
//             TimeUnit::Seconds,
//             config.render_mode,
//         );
//         let clock_color = config
//             .clock_color
//             .map(|color| Color::from_str(&color).unwrap_or(FALLBACK_COLOR_THEME[0]));
//         let accent_color = config
//             .accent_color
//             .map(|color| Color::from_str(&color).unwrap_or(FALLBACK_COLOR_THEME[2]));
//
//         ColorClock::new(
//             config.name.unwrap(),
//             config.hour,
//             config.minutes,
//             config.seconds,
//             config.separator,
//             hour_coords,
//             minute_coords,
//             second_coords,
//             clock_color,
//             accent_color,
//         )
//     }
// }

impl From<DigitalClockConfig> for DigitalClock {
    fn from(config: DigitalClockConfig) -> DigitalClock {
        todo!()
    }
}

impl From<AnalogClockConfig> for AnalogClock {
    fn from(config: AnalogClockConfig) -> AnalogClock {
        todo!()
    }
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    fn load_user_clockfaces() -> LoaderResult<Vec<Clock>> {
        todo!()
    }

    pub fn load_clockfaces() -> LoaderResult<Vec<Clock>> {
        let mut clock_faces = CLOCK_FACES
            .iter()
            .map(|clock_face| {
                let clock_config: ClockConfig = toml::from_str(clock_face)?;
                Ok(clock_config.into())
            })
            .collect::<LoaderResult<Vec<_>>>()?;

        // if let Ok(user_clockfaces) = Self::load_user_clockfaces() {
        //  clock_faces.extend(user_clockfaces);
        // }

        Ok(clock_faces)
    }
}
