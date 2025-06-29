use color_eyre::Result;
use tui::renderer::TuiRenderer;

fn main() -> Result<()> {
    color_eyre::install()?;
    TuiRenderer::start_renderer()
}
