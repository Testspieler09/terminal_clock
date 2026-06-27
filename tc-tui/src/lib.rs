pub(crate) mod components;
pub(crate) mod helpers;
pub(crate) mod tui_models;
pub(crate) mod views;

use std::sync::{LazyLock, RwLock};

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, BorderType},
};
use tc_models::{clock::TimeFormat, color_theme::ThemeColor};

use crate::{
    components::{
        carousel_selector::SettingsMenuCtx,
        pomodoro::{PomodoroConfig, PomodoroState, PomodoroTimer},
    },
    tui_models::{
        application::ApplicationState,
        clock::ClockState,
        tui::{TuiAssets, TuiComponents, TuiController, TuiState},
    },
    views::clock::render_clock_view,
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
    pub async fn execute_renderer() -> Result<()> {
        let terminal = ratatui::init();
        let result = Self::run(terminal).await;
        ratatui::restore();
        result
    }

    async fn run(mut terminal: DefaultTerminal) -> Result<()> {
        static TUI_ASSETS: LazyLock<TuiAssets> =
            LazyLock::new(|| TuiAssets::try_default().expect("failed to initialize TUI assets"));

        let tui_state = RwLock::new(TuiState {
            application_state: ApplicationState::Running,
            // TODO: Load the config one as the first here
            clock_state: ClockState {
                clock_face_idx: 0,
                clock_time_fmt: TimeFormat::Hms,
            },
            color_theme_idx: 0,
            quote_idx: Some(0),
            pomodoro: None,
            refresh_rate: 500,
        });

        let controller = TuiController::new(&tui_state);
        let mut tui_components = TuiComponents::new(&TUI_ASSETS);

        loop {
            {
                let state_guard = tui_state.read().unwrap();
                terminal.draw(|frame| {
                    Self::render(frame, &state_guard, &TUI_ASSETS, &tui_components)
                })?;
            }

            let should_exit = controller.handle_events(&mut tui_components)?;
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
