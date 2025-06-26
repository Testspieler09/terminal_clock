use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
};
use std::{thread::sleep, time::Duration};

use crate::ascii_art::temple::Temple;
use crate::font_face_types::color_clock::ColorClock;

pub fn init_renderer() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;

        sleep(Duration::from_millis(100));

        if event::poll(Duration::from_secs(0))? {
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let clock = Temple;
    clock.draw_clockface(frame, "HH:MM:SS", frame.area());
}
