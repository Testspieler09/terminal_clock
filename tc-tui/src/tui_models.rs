use tc_models::{clock::Clock, colorscheme::ColorScheme, quote::Quote};

use crate::components::help_box::HelpBox;
use crate::components::pomodoro::PomodoroTimer;

// TODO: consider using the ARC here?!
pub struct TuiAssets {
    pub colorschemes: Vec<ColorScheme>,
    pub clock_faces: Vec<Box<dyn Clock>>,
    pub quotes: Vec<Quote>,
}

pub enum ApplicationState {
    /// The clock is running and displayed for the user
    Running,

    /// The hero menu (TerminalClock title, Settings, Help, Quit) is rendered
    ShowingHero,

    /// The help box is rendered and displayed for the user
    ShowingHelp,

    /// The settings menu is rendered and displayed for the user
    ShowingSettings,

    /// The program finished successfully
    Finished,
}

// TODO: consider using the ARC here?!
pub struct TuiState<'a> {
    pub application_state: ApplicationState,
    pub current_colorscheme: &'a ColorScheme,
    pub current_clock_face: &'a dyn Clock,
    pub current_quote: Option<&'a Quote>,
    pub current_pomodoro: Option<&'a PomodoroTimer>,
    pub help_box: HelpBox<'a>,
    pub refresh_rate: u32,
}
