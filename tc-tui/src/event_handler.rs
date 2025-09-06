use crate::{
    TuiComponents,
    components::{hero::MenuLabel, settings_menu::SettingsTab},
    tui_models::{ApplicationState, TuiState},
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{io, time::Duration};

pub(crate) struct EventHandler;

impl EventHandler {
    pub fn handle_events(
        tui_state: &mut TuiState,
        components: &mut TuiComponents,
    ) -> io::Result<()> {
        if event::poll(Duration::from_millis(tui_state.refresh_rate))? {
            if let Event::Key(key_event) = event::read()? {
                if matches!(key_event.kind, event::KeyEventKind::Release) {
                    return Ok(());
                }

                // Handle global keys first (like Esc to close buffer/windows)
                if Self::handle_global_keys(key_event, tui_state, components) {
                    return Ok(());
                }

                // Then handle state-specific keys
                match tui_state.application_state {
                    ApplicationState::Running => {
                        Self::handle_normal_keys(key_event, tui_state, components)
                    }
                    ApplicationState::ShowingHero => {
                        Self::handle_hero_keys(key_event, tui_state, components)
                    }
                    ApplicationState::ShowingSettings => {
                        Self::handle_setting_keys(key_event, tui_state, components)
                    }
                    ApplicationState::ShowingHelp | ApplicationState::Finished => {}
                }
            }
        }
        Ok(())
    }

    fn handle_global_keys(
        key_event: KeyEvent,
        tui_state: &mut TuiState,
        components: &mut TuiComponents,
    ) -> bool {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => match tui_state.application_state {
                ApplicationState::ShowingHero => {
                    tui_state.application_state = ApplicationState::Running;
                    true
                }
                ApplicationState::ShowingHelp => {
                    if components.help_box.was_called_from_hero() {
                        tui_state.application_state = ApplicationState::ShowingHero;
                    } else {
                        tui_state.application_state = ApplicationState::Running;
                    }
                    components.help_box.set_called_from_hero(false);
                    true
                }
                ApplicationState::ShowingSettings => {
                    if components.settings_menu.was_called_from_hero() {
                        tui_state.application_state = ApplicationState::ShowingHero;
                    } else {
                        tui_state.application_state = ApplicationState::Running;
                    }
                    components.settings_menu.set_called_from_hero(false);
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

    fn handle_normal_keys(
        key_event: KeyEvent,
        tui_state: &mut TuiState,
        components: &mut TuiComponents,
    ) {
        match key_event.code {
            KeyCode::Esc => {
                tui_state.application_state = ApplicationState::ShowingHero;
            }
            KeyCode::Char('?') | KeyCode::Char('h') => {
                components.help_box.set_called_from_hero(false);
                tui_state.application_state = ApplicationState::ShowingHelp;
            }
            KeyCode::Char('s') => {
                components.settings_menu.set_called_from_hero(false);
                tui_state.application_state = ApplicationState::ShowingSettings;
            }
            _ => {}
        }
    }

    fn handle_hero_keys(
        key_event: KeyEvent,
        tui_state: &mut TuiState,
        components: &mut TuiComponents,
    ) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => components.hero.next_label(),
            KeyCode::Char('k') | KeyCode::Up => components.hero.prev_label(),
            KeyCode::Enter => match components.hero.active_label {
                MenuLabel::QUIT => tui_state.application_state = ApplicationState::Finished,
                MenuLabel::HELP => {
                    components.help_box.set_called_from_hero(true);
                    tui_state.application_state = ApplicationState::ShowingHelp;
                }
                MenuLabel::SETTINGS => {
                    components.settings_menu.set_called_from_hero(true);
                    tui_state.application_state = ApplicationState::ShowingSettings;
                }
            },
            _ => {}
        }
    }

    fn handle_setting_keys(
        key_event: KeyEvent,
        tui_state: &mut TuiState,
        components: &mut TuiComponents,
    ) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {}
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('h') | KeyCode::Left => {}
            KeyCode::Char('l') | KeyCode::Right => {}
            KeyCode::Tab => components.settings_menu.next_label(),
            KeyCode::BackTab => components.settings_menu.prev_label(),
            // KeyCode::Char('1') => components.settings_menu.display_tab(SettingsTab::General),
            // KeyCode::Char('2') => components.settings_menu.display_tab(SettingsTab::Pomodoro),
            // KeyCode::Char('3') => components.settings_menu.display_tab(SettingsTab::Color),
            KeyCode::Char('s') => {
                if components.settings_menu.was_called_from_hero() {
                    tui_state.application_state = ApplicationState::ShowingHero;
                } else {
                    tui_state.application_state = ApplicationState::Running;
                }
                components.settings_menu.set_called_from_hero(false);
            }
            _ => {}
        }
    }
}
