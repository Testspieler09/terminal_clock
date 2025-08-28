use crate::{
    components::{hero::MenuLabel, settings_menu::SettingsTab},
    tui_models::{ApplicationState, TuiState},
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{io, time::Duration};

pub(crate) struct EventHandler;

impl EventHandler {
    pub async fn handle_events(tui_state: &mut TuiState) -> io::Result<()> {
        if event::poll(Duration::from_millis(tui_state.refresh_rate))? {
            if let Event::Key(key_event) = event::read()? {
                // Handle global keys first (like Esc to close buffer/windows)
                if Self::handle_global_keys(key_event, tui_state) {
                    return Ok(());
                }

                // Then handle state-specific keys
                match tui_state.application_state {
                    ApplicationState::Running => Self::handle_normal_keys(key_event, tui_state),
                    ApplicationState::ShowingHero => Self::handle_hero_keys(key_event, tui_state),
                    ApplicationState::ShowingSettings => {
                        Self::handle_setting_keys(key_event, tui_state)
                    }
                    ApplicationState::ShowingHelp | ApplicationState::Finished => {}
                }
            }
        }
        Ok(())
    }

    fn handle_global_keys(key_event: KeyEvent, tui_state: &mut TuiState) -> bool {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => match tui_state.application_state {
                ApplicationState::ShowingHero => {
                    tui_state.application_state = ApplicationState::Running;
                    tui_state.hero.set_visibility(false);
                    true
                }
                ApplicationState::ShowingHelp => {
                    if tui_state.help_box.was_called_from_hero() {
                        tui_state.application_state = ApplicationState::ShowingHero;
                    } else {
                        tui_state.application_state = ApplicationState::Running;
                    }
                    tui_state.help_box.set_visibility(false, false);
                    true
                }
                ApplicationState::ShowingSettings => {
                    if tui_state.settings_menu.was_called_from_hero() {
                        tui_state.application_state = ApplicationState::ShowingHero;
                    } else {
                        tui_state.application_state = ApplicationState::Running;
                    }
                    tui_state.settings_menu.set_visibility(false, false);
                    true
                }
                ApplicationState::Running => {
                    if matches!(key_event.code, KeyCode::Char('q')) {
                        tui_state.application_state = ApplicationState::Finished;
                        true
                    } else {
                        false
                    }
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
            KeyCode::Char('?') | KeyCode::Char('h') => {
                tui_state.help_box.set_visibility(true, false);
                tui_state.application_state = ApplicationState::ShowingHelp;
            }
            KeyCode::Char('s') => {
                tui_state.settings_menu.set_visibility(true, false);
                tui_state.application_state = ApplicationState::ShowingSettings;
            }
            _ => {}
        }
    }

    fn handle_hero_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => tui_state.hero.next_label(),
            KeyCode::Char('k') | KeyCode::Up => tui_state.hero.prev_label(),
            KeyCode::Enter => match tui_state.hero.active_label {
                MenuLabel::QUIT => tui_state.application_state = ApplicationState::Finished,
                MenuLabel::HELP => {
                    tui_state.help_box.set_visibility(true, true);
                    tui_state.application_state = ApplicationState::ShowingHelp;
                }
                MenuLabel::SETTINGS => {
                    tui_state.settings_menu.set_visibility(true, true);
                    tui_state.application_state = ApplicationState::ShowingSettings;
                }
            },
            _ => {}
        }
    }

    fn handle_setting_keys(key_event: KeyEvent, tui_state: &mut TuiState) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {}
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('h') | KeyCode::Left => {}
            KeyCode::Char('l') | KeyCode::Right => {}
            KeyCode::Tab => tui_state.settings_menu.next_label(),
            KeyCode::BackTab => tui_state.settings_menu.prev_label(),
            KeyCode::Char('1') => tui_state.settings_menu.display_tab(SettingsTab::General),
            KeyCode::Char('2') => tui_state.settings_menu.display_tab(SettingsTab::Pomodoro),
            KeyCode::Char('3') => tui_state.settings_menu.display_tab(SettingsTab::Color),
            KeyCode::Char('s') => {
                if tui_state.settings_menu.was_called_from_hero() {
                    tui_state.application_state = ApplicationState::ShowingHero;
                } else {
                    tui_state.application_state = ApplicationState::Running;
                }
                tui_state.settings_menu.set_visibility(false, false);
            }
            _ => {}
        }
    }
}
