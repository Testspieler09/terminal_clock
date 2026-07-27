use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    Digits,
    Bits,
    Gauge,
}

pub fn default_render_mode() -> RenderMode {
    RenderMode::Digits
}
