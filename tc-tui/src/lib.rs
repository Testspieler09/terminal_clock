use ratatui::prelude::Widget;
pub(crate) mod components;
pub mod error;
pub(crate) mod helpers;
pub(crate) mod tui_models;
pub(crate) mod views;

use std::{path::PathBuf, sync::OnceLock};

pub use error::AppError;

use crate::{components::fireworks::FireworksAnimation, tui_models::tui_action::TuiAction};
pub(crate) type Result<T> = std::result::Result<T, AppError>;

use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, BorderType},
};
use tc_models::color_theme::ThemeColor;

use crate::{
    tui_models::{
        application::ApplicationState,
        controller,
        tui::{TuiAssets, TuiComponents, TuiState},
    },
    views::clock::render_clock_view,
};

static TUI_ASSETS: OnceLock<TuiAssets> = OnceLock::new();

pub(crate) fn assets() -> &'static TuiAssets {
    TUI_ASSETS.get().expect("TUI_ASSETS not initialised")
}

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
            None => tc_user_config_loader::get_user_config_path()?,
        };
        let assets_owned = TuiAssets::try_new(config_path.clone())?;
        TUI_ASSETS.get_or_init(|| assets_owned);

        let mut tui_state = assets().initial_state(&config_path, refresh_rate);

        let mut tui_components = TuiComponents::new();

        loop {
            terminal.draw(|frame| {
                if matches!(
                    tui_state.application_state,
                    ApplicationState::ShowingFirework
                ) {
                    tui_components.firework_animation.tick(
                        frame.area(),
                        FireworksAnimation::get_burst_colors(tui_state.color_theme_idx),
                    );
                }
                Self::render(frame, &tui_state, &tui_components)
            })?;

            let should_exit = controller::handle_events(&mut tui_state, &mut tui_components)?;

            if matches!(
                tui_state.application_state,
                ApplicationState::ShowingSettings
            ) {
                tui_components.settings_menu.tick();
            }

            if should_exit {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, state: &TuiState, components: &TuiComponents) {
        let theme = assets().get_color_theme(state.color_theme_idx);

        // Set the right background with a nice border
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(*theme.get(&ThemeColor::Borders)))
                .style(theme.default_style()),
            frame.area(),
        );

        match state.application_state {
            ApplicationState::Running => {
                render_clock_view(frame, state);
            }
            ApplicationState::ShowingHero => {
                components
                    .logo
                    .render_component_with_logo(&components.hero, frame, state)
            }
            ApplicationState::ShowingHelp => components.logo.render_styled_component_with_logo(
                &components.help_box,
                frame,
                state,
            ),
            ApplicationState::ShowingSettings => components.logo.render_styled_component_with_logo(
                &components.settings_menu,
                frame,
                state,
            ),
            ApplicationState::ShowingFirework => {
                components
                    .firework_animation
                    .render(frame.area(), frame.buffer_mut());
            }
            ApplicationState::Finished => {}
        }
    }
}
