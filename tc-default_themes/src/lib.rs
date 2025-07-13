use crate::colorschemes::neo_tokyo::neo_tokyo;
use tc_user_config_loader::{clock::Clock, colorscheme::ColorScheme};

mod colorschemes;

pub fn load_all_default_colorschemes() -> Vec<ColorScheme> {
    vec![neo_tokyo()]
}

pub fn load_all_default_font_faces() -> Vec<Box<dyn Clock>> {
    todo!();
}
