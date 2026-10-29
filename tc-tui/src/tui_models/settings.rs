use strum::AsRefStr;

use crate::tui_models::{selector::SelectorType, tui_traits::GetSelectorType};

/// [`Setting`] enum contains all the entries shown in the settings menu.
/// It is split into [`General`], [`Pomodoro`] and [`Color`] just like
/// the tabs in the settings menu.
///
/// This was done to enable compiletime checks for exhaustive matching.
/// The tradeoff is that we cannot check exhaustive for the selectortype now.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Setting {
    General(GeneralSetting),
    Pomodoro(PomodoroSetting),
    Color(ColorSetting),
}

impl Setting {
    // General
    #[allow(non_upper_case_globals)]
    pub const RefreshRate: Self = Self::General(GeneralSetting::RefreshRate);

    #[allow(non_upper_case_globals)]
    pub const ClockFace: Self = Self::General(GeneralSetting::ClockFace);

    #[allow(non_upper_case_globals)]
    pub const ClockFormat: Self = Self::General(GeneralSetting::ClockFormat);

    #[allow(non_upper_case_globals)]
    pub const Quote: Self = Self::General(GeneralSetting::Quote);

    // Pomodoro
    #[allow(non_upper_case_globals)]
    pub const TotalSessions: Self = Self::Pomodoro(PomodoroSetting::TotalSessions);

    #[allow(non_upper_case_globals)]
    pub const SessionsBeforeLongBreak: Self =
        Self::Pomodoro(PomodoroSetting::SessionsBeforeLongBreak);

    #[allow(non_upper_case_globals)]
    pub const WorkDuration: Self = Self::Pomodoro(PomodoroSetting::WorkDuration);

    #[allow(non_upper_case_globals)]
    pub const ShortBreakDuration: Self = Self::Pomodoro(PomodoroSetting::ShortBreakDuration);

    #[allow(non_upper_case_globals)]
    pub const LongBreakDuration: Self = Self::Pomodoro(PomodoroSetting::LongBreakDuration);

    // Color
    #[allow(non_upper_case_globals)]
    pub const ColorTheme: Self = Self::Color(ColorSetting::ColorTheme);

    #[allow(non_upper_case_globals)]
    pub const ForegroundColor: Self = Self::Color(ColorSetting::ForegroundColor);

    #[allow(non_upper_case_globals)]
    pub const BackgroundColor: Self = Self::Color(ColorSetting::BackgroundColor);

    #[allow(non_upper_case_globals)]
    pub const SelectionColor: Self = Self::Color(ColorSetting::SelectionColor);

    #[allow(non_upper_case_globals)]
    pub const AccentColor: Self = Self::Color(ColorSetting::AccentColor);

    #[allow(non_upper_case_globals)]
    pub const BorderColor: Self = Self::Color(ColorSetting::BorderColor);

    pub fn as_ref(&self) -> &str {
        match self {
            Self::General(setting) => setting.as_ref(),
            Self::Pomodoro(setting) => setting.as_ref(),
            Self::Color(setting) => setting.as_ref(),
        }
    }
}

impl GetSelectorType for Setting {
    fn selector_type(self) -> SelectorType {
        match self {
            Setting::General(setting) => setting.selector_type(),
            Setting::Pomodoro(setting) => setting.selector_type(),
            Setting::Color(setting) => setting.selector_type(),
        }
    }
}

#[derive(AsRefStr, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralSetting {
    // General
    #[strum(serialize = "Refresh Rate")]
    RefreshRate,
    #[strum(serialize = "Clock Face")]
    ClockFace,
    #[strum(serialize = "Clock Format")]
    ClockFormat,
    #[strum(serialize = "Quote")]
    Quote,
}

impl From<GeneralSetting> for Setting {
    fn from(value: GeneralSetting) -> Self {
        Self::General(value)
    }
}

impl GetSelectorType for GeneralSetting {
    fn selector_type(self) -> SelectorType {
        match self {
            Self::RefreshRate => SelectorType::Number,
            Self::ClockFace => SelectorType::Carousel,
            Self::ClockFormat => SelectorType::Carousel,
            Self::Quote => SelectorType::Carousel,
        }
    }
}

#[derive(AsRefStr, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PomodoroSetting {
    // Pomodoro
    #[strum(serialize = "Total Sessions")]
    TotalSessions,
    #[strum(serialize = "Long Break After")]
    SessionsBeforeLongBreak,
    #[strum(serialize = "Work Duration")]
    WorkDuration,
    #[strum(serialize = "Short Break Duration")]
    ShortBreakDuration,
    #[strum(serialize = "Long Break Duration")]
    LongBreakDuration,
}

impl From<PomodoroSetting> for Setting {
    fn from(value: PomodoroSetting) -> Self {
        Self::Pomodoro(value)
    }
}

impl GetSelectorType for PomodoroSetting {
    fn selector_type(self) -> SelectorType {
        match self {
            Self::TotalSessions
            | Self::SessionsBeforeLongBreak
            | Self::WorkDuration
            | Self::ShortBreakDuration
            | Self::LongBreakDuration => SelectorType::Number,
        }
    }
}

#[derive(AsRefStr, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColorSetting {
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

impl From<ColorSetting> for Setting {
    fn from(value: ColorSetting) -> Self {
        Self::Color(value)
    }
}

impl GetSelectorType for ColorSetting {
    fn selector_type(self) -> SelectorType {
        match self {
            Self::ColorTheme => SelectorType::Carousel,
            Self::ForegroundColor
            | Self::BackgroundColor
            | Self::SelectionColor
            | Self::AccentColor
            | Self::BorderColor => SelectorType::Color,
        }
    }
}
