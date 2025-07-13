use crate::ascii_art::temple::temple;
use crate::colorschemes::neo_tokyo::neo_tokyo;
use tc_user_config_loader::{clock::Clock, colorscheme::ColorScheme};

mod ascii_art;
mod colorschemes;

pub async fn load_all_default_colorschemes() -> Vec<ColorScheme> {
    vec![neo_tokyo()]
}

pub async fn load_all_default_font_faces() -> Vec<Box<dyn Clock>> {
    vec![temple()]
}
