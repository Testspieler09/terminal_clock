use std::time::Duration;

use ratatui::{
    DefaultTerminal,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use crate::{
    Result, components::fireworks::FireworksAnimation, debug_views::DebugView,
    tui_models::tui::TuiAssets,
};

pub struct FireworksView;

impl DebugView for FireworksView {
    fn name(&self) -> &'static str {
        "Fireworks (New Year's easter egg)"
    }

    fn run(&self, terminal: &mut DefaultTerminal, _assets: &TuiAssets) -> Result<()> {
        use ratatui::crossterm::event::{self, Event};

        let mut animation = FireworksAnimation::new();

        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                animation.tick(area);
                render_frame(frame.buffer_mut(), area, &mut animation);
            })?;

            if animation.finished {
                break;
            }

            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }

            // Cap at ~60 fps; animation advances by wall-time so rate doesn't affect speed
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}

fn render_frame(buf: &mut Buffer, area: Rect, animation: &mut FireworksAnimation) {
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buf[(x, y)]
                .set_symbol(" ")
                .set_style(Style::default().bg(Color::Black));
        }
    }
    (&mut *animation).render(area, buf);
}
