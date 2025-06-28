use color_eyre::Result;
use tui::renderer::init_renderer;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_renderer()
}
