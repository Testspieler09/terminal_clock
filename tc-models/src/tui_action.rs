// use crate::{
//     clock::{Clock, TimeFormat},
//     color_theme::{ColorTheme, ThemeColor},
//     quote::Quote,
// };
// use ratatui::style::Color;
// use std::sync::{Arc, Mutex};
//
// pub enum TuiAction {
//     /// General actions
//     UpdateRefreshRate(u64),
//     UpdateClockFace(Arc<Mutex<Clock>>),
//     UpdateClockFormat(TimeFormat),
//     UpdateQuote(Option<Arc<Quote>>),
//
//     /// Pomodoro actions
//     UpdateTotalSession(u32),
//     UpdateSessionsBeforeLongBreak(u32),
//     UpdateWorkDuration(u64),
//     UpdateShortBreakDuration(u64),
//     UpdateLongBreakDuration(u64),
//
//     /// Color actions
//     UpdateColorTheme(Arc<Mutex<ColorTheme>>),
//     UpdateColor(ThemeColor, Color),
// }
