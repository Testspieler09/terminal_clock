use std::process;

use clap::Parser;
use tc_user_config_loader::{
    clock_face_loader::ClockConfig, color_theme_loader::ThemeConfig, quote_loader::QuotesConfig,
};

/// Validate one or more terminal-clock config files (themes, clock faces, quotes).
///
/// The file type is detected automatically by trying each parser in order.
#[derive(Parser)]
#[command(name = "check-config", version)]
struct Args {
    /// Paths to TOML config files to validate
    #[arg(required = true, num_args = 1..)]
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut any_failed = false;

    for path in &args.paths {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FAILED  {path}  --  could not read file: {e}");
                any_failed = true;
                continue;
            }
        };

        match detect_and_validate(&content) {
            Ok(kind) => println!("PASSED  {path}  --  valid {kind}"),
            Err(e) => {
                eprintln!("FAILED  {path}  --  {e}");
                any_failed = true;
            }
        }
    }

    if any_failed {
        process::exit(1);
    }
}

fn detect_and_validate(content: &str) -> Result<&'static str, String> {
    if toml::from_str::<ClockConfig>(content).is_ok() {
        return Ok("clock face config");
    }

    if let Ok(config) = toml::from_str::<QuotesConfig>(content) {
        return config
            .into_quotes_with_initial()
            .map(|_| "quotes config")
            .map_err(|e| format!("invalid quotes config: {e}"));
    }

    toml::from_str::<ThemeConfig>(content)
        .map(|_| "theme config")
        .map_err(|e| format!("invalid theme config: {e}"))
}
