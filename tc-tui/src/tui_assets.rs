use tc_user_config_loader::clock::Clock;
use tc_user_config_loader::colorscheme::ColorScheme;

pub struct TuiAssets {
    colorschemes: Vec<ColorScheme>,
    clock_faces: Vec<Box<dyn Clock>>,
    // quotes: Vec<String>,
}
