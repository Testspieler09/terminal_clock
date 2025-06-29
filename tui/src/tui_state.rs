use user_config_loader::clock::Clock;
use user_config_loader::colorscheme::ColorScheme;

pub struct TuiState {
    current_colorscheme: ColorScheme,
    current_clock_face: dyn Clock,
}
