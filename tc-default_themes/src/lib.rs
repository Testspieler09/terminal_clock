use crate::colorschemes::neo_tokyo::neo_tokyo;
use crate::{ascii_art::temple::temple, default_quotes::init_default_quotes};
use tc_user_config_loader::models::{clock::Clock, colorscheme::ColorScheme, quote::Quote};

mod ascii_art;
mod colorschemes;
mod default_quotes;

pub fn load_all_default_colorschemes() -> Vec<ColorScheme> {
    vec![neo_tokyo()]
}

pub fn load_all_default_font_faces() -> Vec<Box<dyn Clock>> {
    vec![temple()]
}

pub fn load_all_default_quotes() -> Vec<Quote> {
    init_default_quotes()
}
