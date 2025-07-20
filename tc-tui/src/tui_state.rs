use tc_user_config_loader::clock::Clock;
use tc_user_config_loader::colorscheme::ColorScheme;
use tc_user_config_loader::quote::Quote;

use crate::components::help_box::HelpBox;
use crate::components::pomodoro::PomodoroTimer;

pub enum ApplicationState {
    RUNNING,
    FINISHED,
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
