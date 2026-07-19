use std::{thread, time::Duration};

use chrono::{DateTime, Datelike, Local, Timelike};
use ratatui::{
    DefaultTerminal,
    style::Style,
    widgets::{Block, BorderType, Widget},
};
use tc_models::color_theme::ThemeColor;

use crate::{Result, assets, components::fireworks::FireworksAnimation, tui_models::tui::TuiState};

// Trigger window: Dec 31 23:59:59 -> Jan 1 00:00:05
pub(crate) fn is_new_years(now: DateTime<Local>) -> bool {
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let min = now.minute();
    let sec = now.second();

    (month == 12 && day == 31 && hour == 23 && min == 59 && sec >= 59)
        || (month == 1 && day == 1 && hour == 0 && min == 0 && sec <= 5)
}

pub(crate) fn run_fireworks(terminal: &mut DefaultTerminal, state: &TuiState) -> Result<()> {
    let theme = assets().get_color_theme(state.color_theme_idx);
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

        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
