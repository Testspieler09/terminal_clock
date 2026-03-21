use ratatui::style::Color;
use tc_models::{clock::TimeFormat, color_theme::ThemeColor};

pub enum TuiAction {
    /// General actions
    UpdateRefreshRate(u64),
    UpdateClockFace(u16),
    UpdateClockFormat(TimeFormat),
    UpdateQuote(Option<u16>),

    /// Pomodoro actions
    UpdateTotalSession(u32),
    UpdateSessionsBeforeLongBreak(u32),
    UpdateWorkDuration(u64),
    UpdateShortBreakDuration(u64),
    UpdateLongBreakDuration(u64),

    /// Color actions
    UpdateColorTheme(u16),
    UpdateColor(ThemeColor, Color),
}
