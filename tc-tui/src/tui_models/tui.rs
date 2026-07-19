use std::path::PathBuf;

use tc_models::{clock::Clock, color_theme::ColorTheme, quote::Quote};
use tc_user_config_loader::{
    clock_face_loader::ClockFaceLoader, color_theme_loader::ColorThemeLoader,
    quote_loader::QuoteLoader,
};

use crate::{
    Result,
    components::{
        help_box::HelpBox, hero::Hero, logo::Logo, pomodoro::PomodoroTimer,
        settings_menu::SettingMenu,
    },
    tui_models::{application::ApplicationState, clock::ClockState},
};

pub struct TuiAssets {
    pub color_themes: Vec<ColorTheme>,
    pub clock_faces: Vec<Clock>,
    pub quotes: Vec<Quote>,
    pub initial_quote_idx: Option<usize>,
}

impl TuiAssets {
    pub fn try_new(config_path: PathBuf) -> Result<TuiAssets> {
        let (quotes, initial_quote_idx) = QuoteLoader::load_quotes(&config_path)?;
        Ok(TuiAssets {
            color_themes: ColorThemeLoader::load_color_themes(&config_path)?,
            clock_faces: ClockFaceLoader::load_clockfaces(&config_path)?,
            quotes,
            initial_quote_idx,
        })
    }

    pub fn get_clock(&self, clock_idx: u16) -> &Clock {
        &self
            .clock_faces
            .get(clock_idx as usize)
            .expect("The clock_idx should never be out of range")
    }

    pub fn get_color_theme(&self, color_theme_idx: u16) -> &ColorTheme {
        &self
            .color_themes
            .get(color_theme_idx as usize)
            .expect("The color_theme_idx should never be out of range")
    }

    pub fn get_quote(&self, quote_idx: Option<u16>) -> Option<&Quote> {
        if let Some(idx) = quote_idx {
            Some(
                &self
                    .quotes
                    .get(idx as usize)
                    .expect("The quote_idx should never be out of range"),
            )
        } else {
            None
        }
    }
}

pub(crate) struct TuiState {
    pub application_state: ApplicationState,
    pub color_theme_idx: u16,
    pub clock_state: ClockState,
    pub quote_idx: Option<u16>,
    pub pomodoro: Option<PomodoroTimer>,
    pub refresh_rate: u16,
}

pub(crate) struct TuiComponents {
    pub help_box: HelpBox,
    pub settings_menu: SettingMenu,
    pub hero: Hero,
    pub logo: Logo,
}

impl TuiComponents {
    pub fn new(tui_assets: &TuiAssets) -> TuiComponents {
        TuiComponents {
            help_box: HelpBox::new(),
            settings_menu: SettingMenu::new(tui_assets),
            hero: Hero::default(),
            logo: Logo::default(),
        }
    }
}
