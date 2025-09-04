pub(crate) mod components;
pub(crate) mod event_handler;
pub(crate) mod helpers;
pub(crate) mod tui_models;
pub(crate) mod views;

use crate::{
    components::{help_box::HelpBox, hero::Hero, logo::Logo, settings_menu::SettingMenu},
    event_handler::EventHandler,
    tui_models::{ApplicationState, TuiAssets, TuiState},
    views::clock::render_clock_view,
};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, BorderType},
};
use tc_models::colorscheme::SchemeColor;
use tc_user_config_loader::{
    clock_face_loader::ClockFaceLoader, colorscheme_loader::ColorSchemeLoader,
    quote_loader::QuoteLoader,
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
        let tui_assets = TuiAssets {
            clock_faces: ClockFaceLoader::load_clockfaces()?,
            quotes: QuoteLoader::load_quotes()?,
            colorschemes: ColorSchemeLoader::load_colorschemes()?,
        };

        let starting_colorscheme = tui_assets.colorschemes[2].clone();

        let mut tui_state = TuiState {
            application_state: ApplicationState::Running,
            clock_face: tui_assets.clock_faces[0].clone(),
            colorscheme: starting_colorscheme.clone(),
            quote: Some(tui_assets.quotes[0].clone()),
            pomodoro: None,
            help_box: HelpBox::new(starting_colorscheme.clone()),
            settings_menu: SettingMenu::new(starting_colorscheme),
            hero: Hero::default(),
            logo: Logo::default(),
            refresh_rate: 500,
        };

        loop {
            terminal.draw(|frame| Self::render(frame, &tui_state))?;

            EventHandler::handle_events(&mut tui_state).await?;

            if matches!(tui_state.application_state, ApplicationState::Finished) {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, config: &TuiState) {
        // Set the right background with a nice border
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(*config.colorscheme.get(&SchemeColor::Borders)))
                .style(config.colorscheme.default_style()),
            frame.area(),
        );

        match config.application_state {
            ApplicationState::Running => render_clock_view(frame, config),
            ApplicationState::ShowingHero => {
                config.logo.render_component_with_logo(&config.hero, frame)
            }
            ApplicationState::ShowingHelp => config
                .logo
                .render_component_with_logo(&config.help_box, frame),
            ApplicationState::ShowingSettings => config
                .logo
                .render_component_with_logo(&config.settings_menu, frame),
            ApplicationState::Finished => {}
        }
    }
}
