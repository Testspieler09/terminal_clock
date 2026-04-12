use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    Digits, // current system
    Bits,   // binary clock
            // Stack,  // thermometer / gauge
            // Auto,   // optional smart detection / FIX: will probably remove this
}

pub fn default_render_mode() -> RenderMode {
    RenderMode::Digits
}
