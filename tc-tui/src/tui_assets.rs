use tc_user_config_loader::clock::Clock;
use tc_user_config_loader::colorscheme::ColorScheme;
use tc_user_config_loader::quote::Quote;

pub struct TuiAssets {
    pub colorschemes: Vec<ColorScheme>,
    pub clock_faces: Vec<Box<dyn Clock>>,
    pub quotes: Vec<Quote>,
}
