pub(crate) mod components;
pub(crate) mod helpers;
pub(crate) mod tui_models;
pub(crate) mod views;

use crate::{
    tui_models::{ApplicationState, TuiAssets, TuiComponents, TuiController, TuiState},
    views::clock::render_clock_view,
};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, BorderType},
};
use std::sync::{Arc, Mutex, RwLock};
use tc_models::color_theme::ThemeColor;

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
    pub async fn execute_renderer() -> Result<()> {
        let terminal = ratatui::init();
        let result = Self::run(terminal).await;
        ratatui::restore();
        result
    }

    async fn run(mut terminal: DefaultTerminal) -> Result<()> {
        let tui_assets = Arc::new(TuiAssets::try_default()?);

        let color_lock = tui_assets.color_themes[0].lock().unwrap();
        let tui_state = Arc::new(RwLock::new(TuiState {
            application_state: ApplicationState::Running,
            clock_face: tui_assets.clock_faces[0].clone(),
            // TODO: Load the config one as the first here
            color_theme: Arc::new(Mutex::new(color_lock.clone())),
            quote: Some(tui_assets.quotes[0].clone()),
            pomodoro: None,
            refresh_rate: 500,
        }));
        drop(color_lock);

        let controller = Arc::new(TuiController::new(tui_state.clone(), tui_assets.clone()));
        let mut tui_components = TuiComponents::new(controller.clone());

        loop {
            {
                let state_guard = tui_state.read().unwrap();
                terminal.draw(|frame| Self::render(frame, &state_guard, &tui_components))?;
            }

            let should_exit = controller.handle_events(&mut tui_components)?;
            if should_exit {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, state: &TuiState, components: &TuiComponents) {
        // Set the right background with a nice border
        let theme_lock = state.color_theme.lock().unwrap();
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(*theme_lock.get(&ThemeColor::Borders)))
                .style(theme_lock.default_style()),
            frame.area(),
        );
        drop(theme_lock);

        match state.application_state {
            ApplicationState::Running => render_clock_view(frame, state),
            ApplicationState::ShowingHero => components
                .logo
                .render_component_with_logo(&components.hero, frame),
            ApplicationState::ShowingHelp => components
                .logo
                .render_component_with_logo(&components.help_box, frame),
            ApplicationState::ShowingSettings => components
                .logo
                .render_component_with_logo(&components.settings_menu, frame),
            ApplicationState::Finished => {}
        }
    }
}
