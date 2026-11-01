use std::{
    backtrace::Backtrace, fs::OpenOptions, io::Write, panic, path::PathBuf, process,
    time::SystemTime,
};

use clap::Parser;
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
    /// %LOCALAPPDATA%\terminal_clock on Windows)
    #[arg(short, long)]
    config: Option<PathBuf>,
}

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|info| {
        ratatui::restore();

        let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("crash.log")
        else {
            eprintln!("Panic occurred, but crash.log could not be opened.");
            return;
        };

        let backtrace = Backtrace::force_capture();

        let _ = writeln!(file, "\n========== APPLICATION CRASH ==========");
        let _ = writeln!(file, "Time: {:?}", SystemTime::now());
        let _ = writeln!(file, "Panic: {info}");

        if let Some(location) = info.location() {
            let _ = writeln!(
                file,
                "Location: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }

        let _ = writeln!(file, "\nBacktrace:\n{backtrace}");
        let _ = writeln!(file, "=======================================\n");
        let _ = file.flush();

        eprintln!("A crash occured. Thus crash.log generated");
    }));
}

#[tokio::main]
async fn main() {
    setup_panic_hook();

    let args = Args::parse();
    if let Err(e) = TuiRenderer::execute_renderer(args.config, args.refresh_rate).await {
        eprintln!("{e}");
        process::exit(1);
    }
}
