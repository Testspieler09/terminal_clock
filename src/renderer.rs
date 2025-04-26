use color_eyre::Result;
use ratatui::crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

use crate::ascii_art::temple::Temple;
use crate::font_face_types::color_clock::ColorClock;
//
pub fn init_renderer() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let clock = Temple;
    clock.draw_clockface(frame, "HH:MM:SS", frame.area());
}
