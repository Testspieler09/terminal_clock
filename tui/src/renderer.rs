use crate::helpers::center_widget;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    prelude::Constraint,
};
use std::{thread::sleep, time::Duration};

use user_config_loader::{
    clock_face_loader::ClockFaceLoader, colorscheme_loader::ColorSchemeLoader,
};

pub fn init_renderer() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;

        // TODO: use async code -> tokyo
        sleep(Duration::from_millis(100));

        // TODO: Adjust Eventhandler here or move into separate file / impl
        if event::poll(Duration::from_secs(0))? && matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let colorscheme = ColorSchemeLoader;
    let clock = ClockFaceLoader.load_clockface();
    let (ascii_art_paragraph, width, height) = clock.draw_clockface("HH:MM:SS");
    let area = center_widget(
        frame.area(),
        Constraint::Length(width as u16),
        Constraint::Length(height as u16),
    );

    frame.render_widget(ascii_art_paragraph, area);
}
