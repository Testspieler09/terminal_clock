use tc_user_config_loader::clock::Clock;
use tc_user_config_loader::colorscheme::ColorScheme;

pub struct TuiState {
    current_colorscheme: ColorScheme,
    current_clock_face: Box<dyn Clock>,
    // current_quote: Option<String>,
    // current_pomodoro: Option<Pomodoro>,
    refresh_rate: u32,
}
