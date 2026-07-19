use std::time::Duration;

use ratatui::{
    DefaultTerminal,
    style::Style,
    widgets::{Block, BorderType},
};
use tc_models::clock::TimeFormat;

use crate::{
    AppError, Result,
    debug_views::{ClockViewConfig, DebugView},
    tui_models::{
        application::ApplicationState,
        clock::ClockState,
        tui::{TuiAssets, TuiState},
    },
    views::clock::render_clock_view,
};

pub struct ClockView(pub ClockViewConfig);

impl ClockView {
    fn resolve_state(&self, assets: &TuiAssets) -> Result<TuiState> {
        let clock_face_idx = match &self.0.clock_face {
            None => 0,
            Some(name) => assets
                .clock_faces
                .iter()
                .position(|c| c.get_name().eq_ignore_ascii_case(name))
                .ok_or_else(|| -> AppError {
                    let available: Vec<&str> =
                        assets.clock_faces.iter().map(|c| c.get_name()).collect();
                    format!(
                        "unknown clock face {:?} -- available: {}",
                        name,
                        available.join(", ")
                    )
                    .into()
                })? as u16,
        };

        let color_theme_idx = match &self.0.theme {
            None => 0,
            Some(name) => assets
                .color_themes
                .iter()
                .position(|t| t.get_name().eq_ignore_ascii_case(name))
                .ok_or_else(|| -> AppError {
                    let available: Vec<&str> =
                        assets.color_themes.iter().map(|t| t.get_name()).collect();
                    format!(
                        "unknown theme {:?} -- available: {}",
                        name,
                        available.join(", ")
                    )
                    .into()
                })? as u16,
        };

        let quote_idx = match &self.0.quote {
            None => None,
            Some(name) if name.eq_ignore_ascii_case("none") => None,
            Some(name) => {
                let idx = assets
                    .quotes
                    .iter()
                    .position(|q| q.text.eq_ignore_ascii_case(name.as_str()))
                    .ok_or_else(|| -> AppError {
                        format!(
                            "unknown quote {:?} -- use 'none' to disable, or omit for default",
                            name
                        )
                        .into()
                    })? as u16;
                Some(idx)
            }
        };

        Ok(TuiState {
            application_state: ApplicationState::Running,
            clock_state: ClockState {
                clock_face_idx,
                clock_time_fmt: self.0.format.unwrap_or(TimeFormat::Hms),
            },
            color_theme_idx,
            quote_idx,
            pomodoro: None,
            refresh_rate: 500,
        })
    }
}

impl DebugView for ClockView {
    fn name(&self) -> &'static str {
        "Clock view"
    }

    fn run(&self, terminal: &mut DefaultTerminal, assets: &TuiAssets) -> Result<()> {
        use ratatui::crossterm::event::{self, Event, KeyCode};

        let state = self.resolve_state(assets)?;

        loop {
            terminal.draw(|frame| {
                let theme = assets.get_color_theme(state.color_theme_idx);
                frame.render_widget(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .border_style(
                            Style::default()
                                .fg(*theme.get(&tc_models::color_theme::ThemeColor::Borders)),
                        )
                        .style(theme.default_style()),
                    frame.area(),
                );
                render_clock_view(frame, &state, assets);
            })?;

            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
