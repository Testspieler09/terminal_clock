use std::collections::HashMap;

use serde::Deserialize;
use tc_models::helper::TimeUnit;

#[derive(Deserialize, PartialEq, Clone, Copy)]
pub enum SymbolRole {
    Hours,
    HourTens,
    Minutes,
    MinuteTens,
    Seconds,
    SecondTens,
}

impl SymbolRole {
    pub fn time_unit(self) -> TimeUnit {
        match self {
            SymbolRole::Hours | SymbolRole::HourTens => TimeUnit::Hours,
            SymbolRole::Minutes | SymbolRole::MinuteTens => TimeUnit::Minutes,
            SymbolRole::Seconds | SymbolRole::SecondTens => TimeUnit::Seconds,
        }
    }

    pub fn is_tens(self) -> bool {
        matches!(
            self,
            SymbolRole::HourTens | SymbolRole::MinuteTens | SymbolRole::SecondTens
        )
    }
}

#[derive(Deserialize)]
pub struct MappingConfig {
    pub symbols: HashMap<char, SymbolRole>,
}

#[derive(Deserialize)]
pub struct BehaviorConfig {
    pub bits: Option<BitsConfig>,
    pub gauge: Option<GaugeConfig>,
}

#[derive(Deserialize)]
pub struct BitsConfig {
    /// optional manual weights (otherwise auto: 1,2,4,8...)
    pub weights: Option<Vec<u32>>,

    /// direction of increasing weight
    #[serde(default = "default_direction")]
    pub direction: Direction,
}

#[derive(Deserialize)]
pub struct GaugeConfig {
    /// max values (fallback to 24/60/60 if None)
    pub max_hour: Option<u32>,
    pub max_minute: Option<u32>,
    pub max_second: Option<u32>,

    /// fill direction
    #[serde(default = "default_direction")]
    pub direction: Direction,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    BottomToTop,
    TopToBottom,
    LeftToRight,
    RightToLeft,
}

fn default_direction() -> Direction {
    Direction::BottomToTop
}
