use crate::tui_state::{ApplicationState, TuiState};
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
                    }
                    KeyCode::Char('q') => {
                        tui_state.application_state = ApplicationState::FINISHED;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
