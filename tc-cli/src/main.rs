use color_eyre::Result;
use tc_tui::renderer::TuiRenderer;

fn main() -> Result<()> {
    color_eyre::install()?;
    TuiRenderer::start_renderer()
}
