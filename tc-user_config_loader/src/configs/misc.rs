use std::collections::HashMap;

use serde::Deserialize;
use tc_models::helper::TimeUnit;

#[derive(Deserialize)]
pub struct MappingConfig {
    /// maps characters in ASCII -> semantic meaning
    ///
    /// example:
    /// H = "hour"
    /// M = "minute"
    /// S = "second"
    pub symbols: HashMap<char, TimeUnit>,
}

#[derive(Deserialize)]
pub struct BehaviorConfig {
    pub bits: Option<BitsConfig>,
    pub stack: Option<StackConfig>,
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
pub struct StackConfig {
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
