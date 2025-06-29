use crate::clock::Clock;
use crate::clock_types::{
    analog_clock::AnalogClock, color_clock::ColorClock, digital_clock::DigitalClock,
};
use ratatui::style::Color;
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct DigitalClockConfig {
    pub bit_depth: u8,
    pub on_color: String,
    pub off_color: String,
}

#[derive(Deserialize)]
pub struct AnalogClockConfig {
    pub face_path: String,
    pub hand_style: String,
}

pub struct ClockFaceLoader;

impl ClockFaceLoader {
    pub fn load_clockface(&self) -> Box<dyn Clock> {
        let hour: String = String::from(
            r"              Θ
              |
         _,*/@@@\,_
       ∙«≡≡≡≡«..»≡≡»∙
          ╟╧╧╧││╧╢
        Θ«╟╤╤╤││╤╢»Θ
    ∙,____╟╧╧╧││╧╢
  _,*@@@@@╟╤╤╤││╤╢»Θ
Θ≈≈≡≡≡≡≡≡≡╟╧╧╧││╧╢
   ╟╪╪╪╪╪╡╟╤╤╤││╤╢»Θ
   ╟╪╪╪╪╪╡╟╧╧╧││╧╢
 Θ«╟╧╧╧╧╧╡╟╤╤╤││╤╢»Θ
   ╟╤╤╤╤╤╡╟╧╧╧││╧╢
   ╟╪╪╪╪╪╡╟╤╤╤││╤╢
   ╙┴┴┴┴┴┘╙┴┴┴┴┴┴╜",
        );
        let minutes: String = String::from(
            r"              Θ
              |
         _,*/@@@\,_
       ∙«≡≡≡≡«..»≡≡»∙
          ╟╧╧╧││╧╢
    ∙,____╟╤╤╤││╤╢»Θ
  _,*@@@@@╟╧╧╧││╧╢
Θ≈≈≡≡≡≡≡≡≡╟╤╤╤││╤╢»Θ
   ╟╪╪╪╪╪╡╟╧╧╧││╧╢
 Θ«╟╧╧╧╧╧╡╟╤╤╤││╤╢»Θ
   ╟╤╤╤╤╤╡╟╧╧╧││╧╢
 Θ«╟╧╧╧╧╧╡╟╤╤╤││╤╢»Θ
   ╟╤╤╤╤╤╡╟╧╧╧││╧╢
   ╟╪╪╪╪╪╡╟╤╤╤││╤╢
   ╙┴┴┴┴┴┘╙┴┴┴┴┴┴╜",
        );
        let seconds = minutes.clone();
        let led_coords_hours: Vec<Vec<(u32, u32)>> = vec![
            // Hour 00
            vec![(0, 14)],
            // Hour 01
            vec![(0, 14), (11, 19)],
            // Hour 02
            vec![(0, 14), (9, 19)],
            // Hour 03
            vec![(0, 14), (11, 19), (9, 19)],
            // Hour 04
            vec![(0, 14), (7, 19)],
            // Hour 05
            vec![(0, 14), (11, 19), (7, 19)],
            // Hour 06
            vec![(0, 14), (9, 19), (7, 19)],
            // Hour 07
            vec![(0, 14), (11, 19), (9, 19), (7, 19)],
            // Hour 08
            vec![(0, 14), (5, 19), (5, 8)],
            // Hour 09
            vec![(0, 14), (11, 19), (5, 19), (5, 8)],
            // Hour 10
            vec![(0, 14), (11, 1)],
            // Hour 11
            vec![(0, 14), (11, 19), (11, 1)],
            // Hour 12
            vec![(0, 14), (9, 19), (11, 1)],
            // Hour 13
            vec![(0, 14), (11, 19), (9, 19), (11, 1)],
            // Hour 14
            vec![(0, 14), (7, 19), (11, 1)],
            // Hour 15
            vec![(0, 14), (11, 19), (7, 19), (11, 1)],
            // Hour 16
            vec![(0, 14), (9, 19), (7, 19), (11, 1)],
            // Hour 17
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (11, 1)],
            // Hour 18
            vec![(0, 14), (5, 19), (5, 8), (11, 1)],
            // Hour 19
            vec![(0, 14), (11, 19), (5, 19), (5, 8), (11, 1)],
            // Hour 20
            vec![(0, 14), (8, 0)],
            // Hour 21
            vec![(0, 14), (11, 19), (8, 0)],
            // Hour 22
            vec![(0, 14), (9, 19), (8, 0)],
            // Hour 23
            vec![(0, 14), (11, 19), (9, 19), (8, 0)],
        ];

        let led_coords_minutes: Vec<Vec<(u32, u32)>> = vec![
            // Minute 00
            vec![(0, 14)],
            // Minute 01
            vec![(0, 14), (11, 19)],
            // Minute 02
            vec![(0, 14), (9, 19)],
            // Minute 03
            vec![(0, 14), (11, 19), (9, 19)],
            // Minute 04
            vec![(0, 14), (7, 19)],
            // Minute 05
            vec![(0, 14), (11, 19), (7, 19)],
            // Minute 06
            vec![(0, 14), (9, 19), (7, 19)],
            // Minute 07
            vec![(0, 14), (11, 19), (9, 19), (7, 19)],
            // Minute 08
            vec![(0, 14), (5, 19)],
            // Minute 09
            vec![(0, 14), (11, 19), (5, 19)],
            // Minute 10
            vec![(0, 14), (11, 1)],
            // Minute 11
            vec![(0, 14), (11, 19), (11, 1)],
            // Minute 12
            vec![(0, 14), (9, 19), (11, 1)],
            // Minute 13
            vec![(0, 14), (11, 19), (9, 19), (11, 1)],
            // Minute 14
            vec![(0, 14), (7, 19), (11, 1)],
            // Minute 15
            vec![(0, 14), (11, 19), (7, 19), (11, 1)],
            // Minute 16
            vec![(0, 14), (9, 19), (7, 19), (11, 1)],
            // Minute 17
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (11, 1)],
            // Minute 18
            vec![(0, 14), (5, 19), (11, 1)],
            // Minute 19
            vec![(0, 14), (11, 19), (5, 19), (11, 1)],
            // Minute 20
            vec![(0, 14), (9, 1)],
            // Minute 21
            vec![(0, 14), (11, 19), (9, 1)],
            // Minute 22
            vec![(0, 14), (9, 19), (9, 1)],
            // Minute 23
            vec![(0, 14), (11, 19), (9, 19), (9, 1)],
            // Minute 24
            vec![(0, 14), (7, 19), (9, 1)],
            // Minute 25
            vec![(0, 14), (11, 19), (7, 19), (9, 1)],
            // Minute 26
            vec![(0, 14), (9, 19), (7, 19), (9, 1)],
            // Minute 27
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (9, 1)],
            // Minute 28
            vec![(0, 14), (5, 19), (9, 1)],
            // Minute 29
            vec![(0, 14), (11, 19), (5, 19), (9, 1)],
            // Minute 30
            vec![(0, 14), (9, 1), (11, 1)],
            // Minute 31
            vec![(0, 14), (11, 19), (9, 1), (11, 1)],
            // Minute 32
            vec![(0, 14), (9, 19), (9, 1), (11, 1)],
            // Minute 33
            vec![(0, 14), (11, 19), (9, 19), (9, 1), (11, 1)],
            // Minute 34
            vec![(0, 14), (7, 19), (9, 1), (11, 1)],
            // Minute 35
            vec![(0, 14), (11, 19), (7, 19), (9, 1), (11, 1)],
            // Minute 36
            vec![(0, 14), (9, 19), (7, 19), (9, 1), (11, 1)],
            // Minute 37
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (9, 1), (11, 1)],
            // Minute 38
            vec![(0, 14), (5, 19), (9, 1), (11, 1)],
            // Minute 39
            vec![(0, 14), (11, 19), (5, 19), (9, 1), (11, 1)],
            // Minute 40
            vec![(0, 14), (7, 0)],
            // Minute 41
            vec![(0, 14), (11, 19), (7, 0)],
            // Minute 42
            vec![(0, 14), (9, 19), (7, 0)],
            // Minute 43
            vec![(0, 14), (11, 19), (9, 19), (7, 0)],
            // Minute 44
            vec![(0, 14), (7, 19), (7, 0)],
            // Minute 45
            vec![(0, 14), (11, 19), (7, 19), (7, 0)],
            // Minute 46
            vec![(0, 14), (9, 19), (7, 19), (7, 0)],
            // Minute 47
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (7, 0)],
            // Minute 48
            vec![(0, 14), (5, 19), (7, 0)],
            // Minute 49
            vec![(0, 14), (11, 19), (5, 19), (7, 0)],
            // Minute 50
            vec![(0, 14), (7, 0), (11, 1)],
            // Minute 51
            vec![(0, 14), (11, 19), (7, 0), (11, 1)],
            // Minute 52
            vec![(0, 14), (9, 19), (7, 0), (11, 1)],
            // Minute 53
            vec![(0, 14), (11, 19), (9, 19), (7, 0), (7, 19), (11, 1)],
            // Minute 54
            vec![(0, 14), (7, 0), (11, 1), (7, 19)],
            // Minute 55
            vec![(0, 14), (11, 19), (7, 19), (7, 0), (11, 1)],
            // Minute 56
            vec![(0, 14), (9, 19), (7, 19), (7, 0), (11, 1)],
            // Minute 57
            vec![(0, 14), (11, 19), (9, 19), (7, 19), (7, 0), (11, 1)],
            // Minute 58
            vec![(0, 14), (5, 19), (7, 0), (11, 1)],
            // Minute 59
            vec![(0, 14), (11, 19), (5, 19), (7, 0), (11, 1)],
        ];

        Box::new(ColorClock::new(
            hour,
            minutes,
            seconds,
            led_coords_hours,
            led_coords_minutes.clone(),
            led_coords_minutes,
            Color::Blue,
        ))
    }

    fn from(config: ClockConfig) {
        todo!()
    }
}
