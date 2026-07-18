use std::path::Path;

use tc_user_config_loader::app_config_loader::AppConfigLoader;

use crate::tui_models::{
    application::ApplicationState,
    clock::ClockState,
    tui::{TuiAssets, TuiState},
};

impl TuiAssets {
    pub fn initial_state(&self, config_path: &Path, refresh_rate_arg: u16) -> TuiState {
        let config = AppConfigLoader::load(config_path);

        let clock_face_idx = config
            .clock_face
            .as_deref()
            .and_then(|name| self.clock_faces.iter().position(|c| c.get_name() == name))
            .unwrap_or(0) as u16;

        let color_theme_idx = config
            .color_theme
            .as_deref()
            .and_then(|name| self.color_themes.iter().position(|t| t.get_name() == name))
            .unwrap_or(0) as u16;

        let clock_time_fmt = config.time_format.unwrap_or_default();

        let quote_idx = if config.show_quote.unwrap_or(true) && !self.quotes.is_empty() {
            Some(0)
        } else {
            None
        };

        // CLI arg takes priority over config file only if it differs from the default
        let refresh_rate = if refresh_rate_arg != 500 {
            refresh_rate_arg
        } else {
            config.refresh_rate.unwrap_or(500)
        };

        TuiState {
            application_state: ApplicationState::Running,
            clock_state: ClockState {
                clock_face_idx,
                clock_time_fmt,
            },
            color_theme_idx,
            quote_idx,
            pomodoro: None,
            refresh_rate,
        }
    }
}
