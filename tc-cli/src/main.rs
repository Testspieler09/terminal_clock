use color_eyre::Result;
use tc_tui::renderer::TuiRenderer;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    TuiRenderer::start_renderer().await
}
