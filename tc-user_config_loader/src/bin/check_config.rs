use std::process;

use clap::Parser;
use tc_user_config_loader::{
    clock_face_loader::ClockConfig, color_theme_loader::ThemeConfig, quote_loader::QuoteConfig,
};

/// Validate one or more terminal-clock config files (themes, clock faces, quotes).
///
/// The file type is detected automatically: clock configs have a `clock_type` key,
/// quote configs have a `text` field, everything else is treated as a theme.
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
    // Clock configs are tagged with `clock_type`
    if content.contains("clock_type") {
        return toml::from_str::<ClockConfig>(content)
            .map(|_| "clock face config")
            .map_err(|e| format!("invalid clock face config: {e}"));
    }

    // Quote configs require a `text` field
    if content.contains("text =") {
        if toml::from_str::<QuoteConfig>(content).is_ok() {
            return Ok("quote config");
        }
    }

    // Fall back to theme config
    toml::from_str::<ThemeConfig>(content)
        .map(|_| "theme config")
        .map_err(|e| format!("invalid theme config: {e}"))
}
