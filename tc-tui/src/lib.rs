pub(crate) mod components;
pub(crate) mod event_handler;
pub(crate) mod helpers;
pub(crate) mod tui_models;

use crate::{
    components::{
        help_box::HelpBox,
        hero::Hero,
        pomodoro::{PomodoroConfig, PomodoroTimer},
        settings_menu::SettingMenu,
    },
    event_handler::EventHandler,
    helpers::{center_widget, center_widget_horizontally},
    tui_models::{ApplicationState, TuiAssets, TuiState},
};
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame, prelude::Constraint};
use tc_user_config_loader::{
    clock_face_loader::ClockFaceLoader, colorscheme_loader::ColorSchemeLoader,
    quote_loader::QuoteLoader,
};
use tokio::time::{Duration, sleep};

pub struct TuiRenderer;

impl TuiRenderer {
    pub async fn start_renderer() -> Result<()> {
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
        let mut tui_state = TuiState {
            application_state: ApplicationState::Running,
            current_clock_face: tui_assets.clock_faces[0].clone(),
            current_colorscheme: tui_assets.colorschemes[0].clone(),
            current_quote: Some(tui_assets.quotes[0].clone()),
            current_pomodoro: None,
            help_box: HelpBox::default(),
            settings_menu: SettingMenu::default(),
            hero: Hero::default(),
            refresh_rate: 100,
        };

        loop {
            terminal.draw(|frame| Self::render(frame, &tui_state))?;

            sleep(Duration::from_millis(tui_state.refresh_rate)).await;

            EventHandler::handle_events(&mut tui_state)?;

            if matches!(tui_state.application_state, ApplicationState::Finished) {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, config: &TuiState) {
        let (ascii_art_paragraph, width, height) = config
            .current_clock_face
            .draw_clockface(&config.current_colorscheme);
        let area = center_widget(
            frame.area(),
            Constraint::Length(width as u16),
            Constraint::Length(height as u16),
        );

        match config.application_state {
            ApplicationState::Running => {
                // Render Clock
                frame.render_widget(ascii_art_paragraph, area);

                // Render Quote if exists
                if let Some(quote) = &config.current_quote {
                    let quote_area = center_widget_horizontally(
                        frame.area(),
                        Constraint::Length(quote.text.len() as u16),
                        Constraint::Length(1),
                        area.y + area.height + 1,
                    );

                    frame.render_widget(quote.render(&config.current_colorscheme), quote_area);
                }

                // Render Pomodoro if active
                if let Some(_pomodoro) = &config.current_pomodoro {
                    let _ = PomodoroTimer::new(PomodoroConfig {
                        work_duration: 25,
                        short_break_duration: 5,
                        long_break_duration: 15,
                        total_sessions: 5,
                        sessions_before_long_break: 2,
                    });
                }
            }
            ApplicationState::ShowingHero => frame.render_widget(&config.hero, frame.area()),
            ApplicationState::ShowingHelp => frame.render_widget(&config.help_box, frame.area()),
            ApplicationState::ShowingSettings => {
                frame.render_widget(&config.settings_menu, frame.area())
            }
            ApplicationState::Finished => {}
        }
    }
}
