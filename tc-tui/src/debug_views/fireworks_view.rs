use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use crate::{
    components::fireworks::FireworksAnimation, debug_views::DebugView, tui_models::tui::TuiAssets,
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

            std::thread::sleep(Duration::from_millis(50));
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
