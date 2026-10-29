use std::time::Duration;

use ratatui::style::Color;

use crate::{components::settings_menu::SelectorConfig, tui_models::settings::Setting};

pub const FULL_LOGO: &str = concat!(
    "████████╗ ██████╗\n",
    "╚══██╔══╝██╔════╝\n",
    "   ██║   ██║\n",
    "   ██║   ██║\n",
    "   ██║   ╚██████╗\n",
    "   ╚═╝    ╚═════╝ v",
    env!("CARGO_PKG_VERSION")
);

/// Mainly used for Logo and Hero
pub const CYAN_SHADES: [Color; 6] = [
    Color::Rgb(0x44, 0xAE, 0xB3),
    Color::Rgb(0x24, 0x9E, 0xA0),
    Color::Rgb(0x00, 0x8B, 0x8B),
    Color::Rgb(0x00, 0x79, 0x79),
    Color::Rgb(0x00, 0x67, 0x67),
    Color::Rgb(0x00, 0x55, 0x55),
];

/// Mainly used for Logo and Hero
pub const GRAY_SHADES: [Color; 6] = [
    Color::Rgb(0xBB, 0xBB, 0xBB),
    Color::Rgb(0xAA, 0xAA, 0xAA),
    Color::Rgb(0x99, 0x99, 0x99),
    Color::Rgb(0x88, 0x88, 0x88),
    Color::Rgb(0x77, 0x77, 0x77),
    Color::Rgb(0x66, 0x66, 0x66),
];

pub const GENERAL_TAB_CONFIG: &'static [SelectorConfig<'static>] = &[
    SelectorConfig {
        setting: Setting::RefreshRate,
        description: &["The rate on which the screen gets refreshed"],
    },
    SelectorConfig {
        setting: Setting::ClockFace,
        description: &["The clock face you want to be displayed"],
    },
    SelectorConfig {
        setting: Setting::ClockFormat,
        description: &[
            "The format the time is displayed in.",
            "",
            "The options are HH:MM:SS, MM:HH:SS and HH:MM",
        ],
    },
    SelectorConfig {
        setting: Setting::Quote,
        description: &["The quote that is supposed to be rendered"],
    },
];

pub const POMODORO_TAB_CONFIG: &'static [SelectorConfig<'static>] = &[
    SelectorConfig {
        setting: Setting::TotalSessions,
        description: &["The total number of Pomodoro sessions"],
    },
    SelectorConfig {
        setting: Setting::SessionsBeforeLongBreak,
        description: &["The number of sessions to complete before taking a long break"],
    },
    SelectorConfig {
        setting: Setting::WorkDuration,
        description: &["Duration of each focused work session (in minutes)"],
    },
    SelectorConfig {
        setting: Setting::ShortBreakDuration,
        description: &["Duration of a short break between work sessions (in minutes)"],
    },
    SelectorConfig {
        setting: Setting::LongBreakDuration,
        description: &["Duration of a long break after multiple sessions (in minutes)"],
    },
];

pub const COLOR_TAB_CONFIG: &'static [SelectorConfig<'static>] = &[
    SelectorConfig {
        setting: Setting::ColorTheme,
        description: &["The overall color theme used across the application"],
    },
    SelectorConfig {
        setting: Setting::ForegroundColor,
        description: &["Color used for primary text and UI elements"],
    },
    SelectorConfig {
        setting: Setting::BackgroundColor,
        description: &[
            "Background color of the application interface",
            "",
            "Set this to `None` to get a transparent background",
        ],
    },
    SelectorConfig {
        setting: Setting::SelectionColor,
        description: &["Color used when selecting text or items"],
    },
    SelectorConfig {
        setting: Setting::AccentColor,
        description: &["Highlight color used for emphasis or active items"],
    },
    SelectorConfig {
        setting: Setting::BorderColor,
        description: &["Color used for borders and outlines"],
    },
];

pub const HELP_BOX_CONTENT: [[&str; 2]; 3] = [
    ["?, h", "Show this help box"],
    ["q", "Quit this program"],
    ["ESC", "Toggles main menu"],
];

/// Overwrite of the user specified refresh rate for
/// the settings menu. This measure is taken, so
/// that the interaction experience is smooth.
pub const SETTINGS_MENU_TICK_MS: u16 = 50;

pub const CAROUSEL_SCROLL_STEP_MS: Duration = Duration::from_millis(200);
pub const CAROUSEL_SCROLL_PAUSE_MS: Duration = Duration::from_millis(600);

pub const FIREWORK_ART_CHARS: &[u8] =
    b"@#$%&*+=^~!?|\\/<>{}[]()0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const FIREWORK_PROJECTILE_TTL_S: Duration = Duration::from_secs(5);
pub const FIREWORK_FRAME_MS: u16 = 16;
