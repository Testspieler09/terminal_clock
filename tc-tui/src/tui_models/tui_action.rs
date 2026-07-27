#![allow(dead_code)]

use ratatui::style::Color;
use tc_models::{clock::TimeFormat, color_theme::ThemeColor};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TuiAction {
    /// General actions
    RefreshRate(u16),
    ClockFace(u16),
    ClockFormat(TimeFormat),
    Quote(Option<u16>),

    /// Pomodoro actions
    TotalSession(u32),
    SessionsBeforeLongBreak(u32),
    WorkDuration(u64),
    ShortBreakDuration(u64),
    LongBreakDuration(u64),

    /// Color actions
    ColorTheme(u16),
    Color(ThemeColor, Color),
}
