use ratatui::style::Color;

pub(super) mod carousel_selector;
pub(super) mod color_input_field;
pub(super) mod fallback_terminal_too_small;
pub(super) mod help_box;
pub(super) mod hero;
pub(super) mod logo;
pub(super) mod number_input;
pub(super) mod pomodoro;
pub(super) mod quote;
pub(super) mod settings_menu;

/// Mainly used for Logo and Hero
pub(super) const CYAN_SHADES: [Color; 6] = [
    Color::Rgb(0x44, 0xAE, 0xB3),
    Color::Rgb(0x24, 0x9E, 0xA0),
    Color::Rgb(0x00, 0x8B, 0x8B),
    Color::Rgb(0x00, 0x79, 0x79),
    Color::Rgb(0x00, 0x67, 0x67),
    Color::Rgb(0x00, 0x55, 0x55),
];

/// Mainly used for Logo and Hero
pub(super) const GRAY_SHADES: [Color; 6] = [
    Color::Rgb(0xBB, 0xBB, 0xBB),
    Color::Rgb(0xAA, 0xAA, 0xAA),
    Color::Rgb(0x99, 0x99, 0x99),
    Color::Rgb(0x88, 0x88, 0x88),
    Color::Rgb(0x77, 0x77, 0x77),
    Color::Rgb(0x66, 0x66, 0x66),
];

pub trait Dimensions {
    fn height(&self) -> u16;
    fn width(&self) -> u16;
}
