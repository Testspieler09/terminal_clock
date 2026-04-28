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

    /// The quote that is displayed under the clock
    #[arg(short, long)]
    quote_text: Option<String>,

    /// The path to the terminal clock config
    #[arg(short, long)]
    configpath: Option<String>,

    /// The path to the colorscheme
    #[arg(short, long)]
    themepath: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();
    color_eyre::install()?;
    TuiRenderer::execute_renderer().await
}
