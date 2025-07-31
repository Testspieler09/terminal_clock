use crate::tui_models::{ApplicationState, TuiState};
use ratatui::crossterm::event::{self, Event, KeyCode};
use tokio::{io, time::Duration};

pub struct EventHandler;

impl EventHandler {
    pub fn handle_events(tui_state: &mut TuiState) -> io::Result<()> {
        if event::poll(Duration::from_secs(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('h') => {
                        tui_state.help_box.toggle_visibility();
                        tui_state.application_state = if tui_state.help_box.is_visible() {
                            ApplicationState::ShowingHelp
                        } else {
                            ApplicationState::Running
                        };
                    }
                    KeyCode::Char('q') => {
                        tui_state.application_state = ApplicationState::Finished;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
