use std::time::Duration;

use ratatui::{
    DefaultTerminal,
    style::Style,
    widgets::{Block, BorderType, Widget},
};
use tc_models::color_theme::ThemeColor;

use crate::{
    AppError, Result, assets,
    components::fireworks::FireworksAnimation,
    debug_views::{DebugView, FireworksViewConfig},
};

pub struct FireworksView(pub FireworksViewConfig);

impl DebugView for FireworksView {
    fn name(&self) -> &'static str {
        "Fireworks (New Year's easter egg)"
    }

    fn run(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        use ratatui::crossterm::event::{self, Event};

        let theme_idx = match &self.0.theme {
            None => 0,
            Some(name) => assets()
                .color_themes
                .iter()
                .position(|t| t.get_name().eq_ignore_ascii_case(name))
                .ok_or_else(|| -> AppError {
                    let available: Vec<&str> =
                        assets().color_themes.iter().map(|t| t.get_name()).collect();
                    format!(
                        "unknown theme {:?} -- available: {}",
                        name,
                        available.join(", ")
                    )
                    .into()
                })?,
        };

        let theme = assets().get_color_theme(theme_idx as u16);
        let bg_style = theme.default_style();
        let border_color = *theme.get(&ThemeColor::Borders);
        let burst_colors = [
            *theme.get(&ThemeColor::Accent),
            *theme.get(&ThemeColor::Foreground),
            *theme.get(&ThemeColor::Selection),
            *theme.get(&ThemeColor::Borders),
        ];
        let mut animation = FireworksAnimation::new(burst_colors);

        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();

                let block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(border_color))
                    .style(bg_style);

                let inner = block.inner(area);
                block.render(area, buf);

                animation.tick(inner);
                animation.render(inner, buf);
            })?;

            if animation.finished {
                break;
            }

            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }

            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
