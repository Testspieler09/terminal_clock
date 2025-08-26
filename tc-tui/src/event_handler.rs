use crate::tui_models::{ApplicationState, TuiState};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{io, time::Duration};

pub(crate) struct EventHandler;

impl EventHandler {
    pub fn handle_events(tui_state: &mut TuiState) -> io::Result<()> {
        if event::poll(Duration::from_secs(tui_state.refresh_rate))? {
            if let Event::Key(key_event) = event::read()? {
                // Handle global keys first (like Esc to close buffer/windows)
                if Self::handle_global_keys(key_event, tui_state) {
                    return Ok(());
                }

                // Then handle state-specific keys
                match tui_state.application_state {
                    ApplicationState::ShowingHero => Self::handle_hero_keys(key_event, tui_state),
                    ApplicationState::ShowingHelp => Self::handle_help_keys(key_event, tui_state),
                    ApplicationState::ShowingSettings => {
                        Self::handle_setting_keys(key_event, tui_state)
                    }
                    ApplicationState::Running => Self::handle_normal_keys(key_event, tui_state),
                    ApplicationState::Finished => {}
                }
            }
        }
        Ok(())
    }

    fn handle_global_keys(key_event: KeyEvent, tui_state: &mut TuiState) -> bool {
        match key_event.code {
            KeyCode::Esc => {
                // Close any open modal
                match tui_state.application_state {
                    ApplicationState::ShowingHero => {
                        tui_state.hero.set_visibility(false);
                        tui_state.application_state = ApplicationState::Running;
                        true
                    }
                    ApplicationState::ShowingHelp => {
                        tui_state.help_box.set_visibility(false);
                        tui_state.application_state = ApplicationState::Running;
                        true
                    }
                    _ => false,
                }
            }
            KeyCode::Char('q') => match tui_state.application_state {
                ApplicationState::Running | ApplicationState::ShowingHelp => {
                    tui_state.application_state = ApplicationState::Finished;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn handle_normal_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Esc => {
                tui_state.hero.set_visibility(true);
                tui_state.application_state = ApplicationState::ShowingHero;
            }
            KeyCode::Char('?') => {
                tui_state.help_box.set_visibility(true);
                tui_state.application_state = ApplicationState::ShowingHelp;
            }
            _ => {}
        }
    }

    fn handle_help_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Char('?') => {
                tui_state.help_box.set_visibility(false);
                tui_state.application_state = ApplicationState::Running;
            }
            _ => {}
        }
    }

    fn handle_hero_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {}
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('s') => {
                tui_state.settings_menu.set_visibility(true);
                tui_state.application_state = ApplicationState::ShowingSettings;
            }
            _ => {}
        }
    }

    fn handle_setting_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {}
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('h') | KeyCode::Left => {}
            KeyCode::Char('l') | KeyCode::Right => {}
            KeyCode::Esc => {
                tui_state.settings_menu.set_visibility(false);
                tui_state.application_state = ApplicationState::ShowingHero;
            }
            KeyCode::Tab => {}
            KeyCode::Char('1') => {}
            KeyCode::Char('2') => {}
            KeyCode::Char('3') => {}
            _ => {}
        }
    }
}
