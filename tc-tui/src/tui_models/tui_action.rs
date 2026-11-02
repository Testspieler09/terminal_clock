use tc_models::clock::TimeFormat;

use crate::tui_models::number_input::NumericValue;

#[derive(Clone, Copy)]
pub enum TuiAction {
    /// Update any of the [`NumericValue`]s by incrementing based on their config.
    /// The config can be fetched by calling `NumericValue::derive_config(self)`
    UpdateNumericValue(NumericOperation, NumericValue),

    /// General actions
    ClockFace(u16),
    ClockFormat(TimeFormat),
    Quote(Option<u16>),

    /// Color actions
    ColorTheme(u16),

    /// Fallback actions
    UpdateFallback((u16, u16)),
}

#[derive(Clone, Copy)]
pub(crate) enum NumericOperation {
    Increment,
    Decrement,
}
