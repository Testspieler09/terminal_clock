use crate::{
    clock::{Clock, TimeFormat},
    color_theme::{ColorTheme, ThemeColor},
    quote::Quote,
};
use ratatui::style::Color;
use std::sync::Arc;

pub enum TuiAction {
    /// General actions
    UpdateRefreshRate(u64),
    UpdateClockFace(Arc<dyn Clock>),
    UpdateClockFormat(TimeFormat),
    UpdateQuote(Arc<Quote>),

    /// Pomodoro actions
    UpdateTotalSession(u32),
    UpdateSessionsBeforeLongBreak(u32),
    UpdateWorkDuration(u64),
    UpdateShortBreakDuration(u64),
    UpdateLongBreakDuration(u64),

    /// Color actions
    UpdateColorTheme(Arc<ColorTheme>),
    UpdateColor(ThemeColor, Color),
}
