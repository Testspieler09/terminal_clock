use std::path::Path;

use serde::Deserialize;
use tc_models::clock::TimeFormat;

use crate::bundled::DEFAULT_CONFIG;

#[derive(Deserialize)]
pub struct AppConfig {
    pub clock_face: Option<String>,
    pub color_theme: Option<String>,
    pub time_format: Option<TimeFormat>,
    pub show_quote: Option<bool>,
    pub refresh_rate: Option<u16>,
}

pub struct AppConfigLoader;

impl AppConfigLoader {
    pub fn load(config_path: &Path) -> AppConfig {
        let bundled: AppConfig =
            toml::from_str(DEFAULT_CONFIG).expect("bundled default_config.toml is invalid");

        let user_path = config_path.join("tc.toml");
        if !user_path.exists() {
            return bundled;
        }

        let content = match std::fs::read_to_string(&user_path) {
            Ok(c) => c,
            Err(_) => return bundled,
        };

        let user: AppConfig = match toml::from_str(&content) {
            Ok(c) => c,
            Err(_) => return bundled,
        };

        AppConfig {
            clock_face: user.clock_face.or(bundled.clock_face),
            color_theme: user.color_theme.or(bundled.color_theme),
            time_format: user.time_format.or(bundled.time_format),
            show_quote: user.show_quote.or(bundled.show_quote),
            refresh_rate: user.refresh_rate.or(bundled.refresh_rate),
        }
    }
}
