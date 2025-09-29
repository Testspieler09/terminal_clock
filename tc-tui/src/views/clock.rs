use crate::{
    TuiState,
    components::pomodoro::{PomodoroConfig, PomodoroTimer},
    helpers::{center_widget, center_widget_horizontally},
};
use ratatui::{Frame, prelude::Constraint};
use tc_models::clock::ClockBehaviour;

pub(crate) fn render_clock_view(frame: &mut Frame, config: &TuiState) {
    let clock_lock = config.clock_face.lock().unwrap();
    let color_theme_lock = config.color_theme.lock().unwrap();

    let (ascii_art_paragraph, width, height) =
        clock_lock.generate_clock_face_with_dimensions(&color_theme_lock);

    let area = center_widget(
        frame.area(),
        Constraint::Length(width as u16),
        Constraint::Length(height as u16),
    );

    // Render Clock
    frame.render_widget(ascii_art_paragraph, area);

    // Render Quote if exists
    if let Some(quote) = &config.quote {
        let quote_area = center_widget_horizontally(
            frame.area(),
            Constraint::Length(quote.text.len() as u16),
            Constraint::Length(1),
            area.y + area.height + 1,
        );

        frame.render_widget(quote.render(&color_theme_lock), quote_area);
    }

    // Render Pomodoro if active
    if let Some(_pomodoro) = &config.pomodoro {
        let _ = PomodoroTimer::new(PomodoroConfig {
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            total_sessions: 5,
            sessions_before_long_break: 2,
        });
    }
}
