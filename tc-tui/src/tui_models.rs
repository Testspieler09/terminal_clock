use crate::components::{
    help_box::HelpBox, hero::Hero, pomodoro::PomodoroTimer, settings_menu::SettingMenu,
};
use std::sync::Arc;
use tc_models::{clock::Clock, colorscheme::ColorScheme, quote::Quote};

pub(crate) struct TuiAssets {
    pub colorschemes: Vec<Arc<ColorScheme>>,
    pub clock_faces: Vec<Arc<dyn Clock>>,
    pub quotes: Vec<Arc<Quote>>,
}

pub(crate) enum ApplicationState {
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

pub(crate) struct TuiState {
    pub application_state: ApplicationState,
    pub colorscheme: Arc<ColorScheme>,
    pub clock_face: Arc<dyn Clock>,
    pub quote: Option<Arc<Quote>>,
    pub pomodoro: Option<Arc<PomodoroTimer>>,
    pub help_box: HelpBox,
    pub settings_menu: SettingMenu,
    pub hero: Hero,
    pub refresh_rate: u64,
}
