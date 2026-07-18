pub(crate) mod components;
pub(crate) mod helpers;
pub(crate) mod tui_models;
pub(crate) mod views;

#[cfg(feature = "debug-views")]
pub mod debug_views;
use std::{path::PathBuf, sync::OnceLock};

use chrono::Local;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, BorderType},
};
use tc_models::{clock::TimeFormat, color_theme::ThemeColor};
#[cfg(feature = "debug-views")]
pub use tui_models::tui::TuiAssets;

#[cfg(not(feature = "debug-views"))]
use crate::tui_models::tui::TuiAssets;
use crate::{
    components::{
        carousel_selector::SettingsMenuCtx,
        pomodoro::{PomodoroConfig, PomodoroState, PomodoroTimer},
    },
    tui_models::{
        application::ApplicationState,
        clock::ClockState,
        controller,
        tui::{TuiComponents, TuiState},
    },
    views::{clock::render_clock_view, fireworks},
};

pub struct TuiRenderer;

impl TuiRenderer {
    /// Initializes the terminal UI, runs the main rendering loop, and restores the terminal state.
    ///
    /// This async function sets up the terminal using the `ratatui` backend, calls the internal
    /// [`Self::run`] method to execute the rendering loop, and ensures the terminal is properly
    /// restored to its original state afterward, even if an error occurs.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the renderer executed successfully or returned an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the rendering loop fails during execution.
    pub async fn execute_renderer(config_path: Option<PathBuf>, refresh_rate: u16) -> Result<()> {
        let terminal = ratatui::init();
        let result = Self::run(terminal, config_path, refresh_rate).await;
        ratatui::restore();
        result
    }

    async fn run(
        mut terminal: DefaultTerminal,
        config_path: Option<PathBuf>,
        refresh_rate: u16,
    ) -> Result<()> {
        let config_path = match config_path {
            Some(path) => path,
            None => tc_user_config_loader::get_user_config_path()
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?,
        };
        static TUI_ASSETS: OnceLock<TuiAssets> = OnceLock::new();
        let assets = TUI_ASSETS.get_or_init(|| {
            TuiAssets::try_new(config_path).expect("failed to initialize TUI assets")
        });

        let mut tui_state = TuiState {
            application_state: ApplicationState::Running,
            // TODO: Load the config one as the first here
            clock_state: ClockState {
                clock_face_idx: 0,
                clock_time_fmt: TimeFormat::Hms,
            },
            color_theme_idx: 0,
            quote_idx: Some(0),
            pomodoro: None,
            refresh_rate,
        };

        if fireworks::is_new_years(Local::now()) {
            fireworks::run_fireworks(&mut terminal, assets, &tui_state)?;
        }

        let mut tui_components = TuiComponents::new(assets);

        loop {
            terminal.draw(|frame| Self::render(frame, &tui_state, assets, &tui_components))?;

            let should_exit = controller::handle_events(&mut tui_state, &mut tui_components)?;

            if matches!(
                tui_state.application_state,
                ApplicationState::ShowingSettings
            ) {
                tui_components.settings_menu.tick(assets);
            }

            if should_exit {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, state: &TuiState, assets: &TuiAssets, components: &TuiComponents) {
        // Set the right background with a nice border
        let theme = assets.get_color_theme(state.color_theme_idx);
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(*theme.get(&ThemeColor::Borders)))
                .style(theme.default_style()),
            frame.area(),
        );

        match state.application_state {
            ApplicationState::Running => {
                render_clock_view(frame, state, assets);
            }
            ApplicationState::ShowingHero => components
                .logo
                .render_component_with_logo(&components.hero, frame),
            ApplicationState::ShowingHelp => components.logo.render_styled_component_with_logo(
                &components.help_box,
                frame,
                theme,
            ),
            ApplicationState::ShowingSettings => components.logo.render_styled_component_with_logo(
                &components.settings_menu,
                frame,
                &SettingsMenuCtx::new(theme, assets),
            ),
            ApplicationState::Finished => {}
        }
    }
}
