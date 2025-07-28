use tc_user_config_loader::models::{clock::Clock, colorscheme::ColorScheme, quote::Quote};

use crate::components::help_box::HelpBox;
use crate::components::pomodoro::PomodoroTimer;

pub struct TuiAssets {
    pub colorschemes: Vec<ColorScheme>,
    pub clock_faces: Vec<Box<dyn Clock>>,
    pub quotes: Vec<Quote>,
}

pub enum ApplicationState {
    Running,
    Finished,
}

pub struct TuiState<'a> {
    pub application_state: ApplicationState,
    pub current_colorscheme: &'a ColorScheme,
    pub current_clock_face: &'a dyn Clock,
    pub current_quote: Option<&'a Quote>,
    pub current_pomodoro: Option<&'a PomodoroTimer>,
    pub help_box: HelpBox<'a>,
    pub refresh_rate: u32,
}
