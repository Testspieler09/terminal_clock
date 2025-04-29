mod helpers;
mod pomodoro;
mod renderer;

mod font_face_types {
    pub mod analog_clock;
    pub mod color_clock;
    pub mod digital_clock;
}
mod ascii_art;

use crate::renderer::init_renderer;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_renderer()
}
