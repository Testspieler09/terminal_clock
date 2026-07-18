use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use tc_tui::TuiRenderer;

/// A simple but fancy looking customizable terminal clock
#[derive(Parser)]
#[command(name = "Tc", version, long_about = None)]
#[command(about=concat!(
    "\n\n",
    "████████╗ ██████╗\n",
    "╚══██╔══╝██╔════╝\n",
    "   ██║   ██║\n",
    "   ██║   ██║\n",
    "   ██║   ╚██████╗\n",
    "   ╚═╝    ╚═════╝ v",
    env!("CARGO_PKG_VERSION"),
    "\n\n",
    "A simple but fancy looking customizable terminal clock",
))]
struct Args {
    /// The refresh rate in milliseconds
    #[arg(short, long, default_value_t = 500)]
    refresh_rate: u16,

    /// Path to the config folder (defaults to ~/.config/terminal_clock on Unix,
    /// %APPDATA%\terminal_clock on Windows)
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    TuiRenderer::execute_renderer(args.config, args.refresh_rate).await
}
