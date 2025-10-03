use strum::AsRefStr;

#[derive(AsRefStr, Clone, Copy)]
pub(crate) enum Setting {
    // General
    #[strum(serialize = "Refresh Rate")]
    RefreshRate,
    #[strum(serialize = "Clock Face")]
    ClockFace,
    #[strum(serialize = "Clock Format")]
    ClockFormat,
    #[strum(serialize = "Quote")]
    Quote,

    // Pomodoro
    #[strum(serialize = "Total Sessions")]
    TotalSessions,
    #[strum(serialize = "Sessions Before Long Break")]
    SessionsBeforeLongBreak,
    #[strum(serialize = "Work Duration")]
    WorkDuration,
    #[strum(serialize = "Short Break Duration")]
    ShortBreakDuration,
    #[strum(serialize = "Long Break Duration")]
    LongBreakDuration,

    // Color
    #[strum(serialize = "Color Theme")]
    ColorTheme,
    #[strum(serialize = "Foreground Color")]
    ForegroundColor,
    #[strum(serialize = "Background Color")]
    BackgroundColor,
    #[strum(serialize = "Selection Color")]
    SelectionColor,
    #[strum(serialize = "Accent Color")]
    AccentColor,
    #[strum(serialize = "Border Color")]
    BorderColor,
}
