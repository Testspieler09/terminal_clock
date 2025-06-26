use crate::renderer::init_renderer;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_renderer()
}
