#[derive(Clone, Copy)]
pub(crate) enum NumericValue {
    RefreshRate,
    TotalSessions,
    SessionsBeforeLongBreak,
    WorkDuration,
    ShortBreakDuration,
    LongBreakDuration,
}

impl NumericValue {
    pub fn derive_config(self) -> NumberSelectorConfig {
        match self {
            Self::RefreshRate => NumberSelectorConfig {
                preset_interval: 100,
                upper_clamp: 2000,
                fallback: 500,
            },
            Self::WorkDuration
            | Self::ShortBreakDuration
            | Self::LongBreakDuration
            | Self::SessionsBeforeLongBreak
            | Self::TotalSessions => NumberSelectorConfig {
                preset_interval: 1,
                upper_clamp: 50,
                fallback: 8,
            },
        }
    }
}

pub(crate) struct NumberSelectorConfig {
    pub preset_interval: u16,
    pub upper_clamp: u64,
    pub fallback: u32,
}
