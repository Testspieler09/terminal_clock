use serde::Deserialize;
use tc_models::{
    clock::TimeFormat,
    render_mode::{RenderMode, default_render_mode},
};

use crate::configs::misc::{BehaviorConfig, MappingConfig};

pub(crate) type LedMap = Vec<(u8, Vec<(u32, u32)>)>;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CoordSource {
    Explicit {
        hour_coords: [LedMap; 2],
        minute_coords: [LedMap; 2],
        second_coords: [LedMap; 2],
    },
    Mapped {
        mapping: MappingConfig,
    },
}

#[derive(Deserialize)]
pub struct ColorClockConfig {
    pub name: Option<String>,

    pub hour: String,
    pub minutes: String,
    pub seconds: String,
    pub separator: Option<String>,

    #[serde(default = "default_render_mode")]
    pub render_mode: RenderMode,

    pub behavior: Option<BehaviorConfig>,

    #[serde(flatten)]
    pub coord_source: CoordSource,

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
