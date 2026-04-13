use serde::Deserialize;
use tc_models::{
    clock::TimeFormat,
    render_mode::{RenderMode, default_render_mode},
};

use crate::configs::misc::{BehaviorConfig, MappingConfig};

// #[derive(Deserialize)]
// pub struct ColorClockConfig {
//     pub name: Option<String>,
//     pub hour: String,
//     pub minutes: String,
//     pub seconds: String,
//     pub separator: Option<String>,
//     pub display_mode: DisplayMode,
//     pub hour_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
//     pub minute_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
//     pub second_coords: [Vec<(u8, Vec<(u32, u32)>)>; 2],
//     pub always_on_coords: Option<Vec<(u32, u32)>>,
//     pub clock_color: Option<String>,
//     pub accent_color: Option<String>,
//     pub format: Option<TimeFormat>,
// }
pub(crate) type LedMap = Vec<(u8, Vec<(u32, u32)>)>;

#[derive(Deserialize)]
pub struct ColorClockConfig {
    pub name: Option<String>,

    pub hour: String,
    pub minutes: String,
    pub seconds: String,
    pub separator: Option<String>,

    #[serde(default = "default_render_mode")]
    pub render_mode: RenderMode,

    pub mapping: Option<MappingConfig>,

    // TODO: think about moving this into the rendermode as enum
    pub behavior: Option<BehaviorConfig>,

    // TODO: make the renderer smart -> if one of the below is not defined it sets its update
    // interval automatically lower if possible (i.e. for embedded devices / runmode otherwise it
    // will interfere with the user input)
    pub hour_coords: Option<[LedMap; 2]>,
    pub minute_coords: Option<[LedMap; 2]>,
    pub second_coords: Option<[LedMap; 2]>,

    pub always_on_coords: Option<Vec<(u32, u32)>>,

    pub clock_color: Option<String>,
    pub accent_color: Option<String>,

    // FIX: the format field is currently not in use (i think)
    pub format: Option<TimeFormat>,
}

impl ColorClockConfig {
    pub fn set_name_if_none(&mut self, name: String) {
        self.name = Some(name)
    }
}
