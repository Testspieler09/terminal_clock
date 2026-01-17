use crate::{
    components::{
        help_box::HelpBox,
        hero::{Hero, MenuLabel},
        logo::Logo,
        pomodoro::PomodoroTimer,
        settings_menu::SettingMenu,
    },
    tui_models::application::ApplicationState,
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::sync::RwLock;
use tc_models::{clock::Clock, color_theme::ColorTheme, quote::Quote};
use tc_user_config_loader::{
    LoaderResult, clock_face_loader::ClockFaceLoader, color_theme_loader::ColorThemeLoader,
    quote_loader::QuoteLoader,
};
use tokio::{io, time::Duration};

pub(crate) struct TuiAssets {
    pub color_themes: Vec<ColorTheme>,
    pub clock_faces: Vec<Clock>,
    pub quotes: Vec<Quote>,
}

impl TuiAssets {
    pub fn try_default() -> LoaderResult<TuiAssets> {
        Ok(TuiAssets {
            color_themes: ColorThemeLoader::load_color_themes()?,
            clock_faces: ClockFaceLoader::load_clockfaces()?,
            quotes: QuoteLoader::load_quotes()?,
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
}
pub(crate) struct TuiState {
    pub application_state: ApplicationState,
    pub color_theme_idx: u16,
    pub clock_face_idx: u16,
    pub quote_idx: Option<u16>,
    pub pomodoro: Option<PomodoroTimer>,
    pub refresh_rate: u64,
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

pub(crate) struct TuiController<'a> {
    tui_state: &'a RwLock<TuiState>,
}

impl<'a> TuiController<'a> {
    pub fn new(tui_state: &'a RwLock<TuiState>) -> Self {
        TuiController { tui_state }
    }

    // pub fn process_settings_action(&mut self, action: &'a TuiAction) {
    //     let mut state = self.tui_state.write().unwrap();
    //     match action {
    //         // = Settings actions
    //         // == General settings
    //         TuiAction::UpdateClockFace(new_clock_face_idx) => {
    //             state.clock_face_idx = *new_clock_face_idx
    //         }
    //         TuiAction::UpdateClockFormat(new_format) => {
    //             self.tui_assets.clock_faces[state.clock_face_idx as usize]
    //                 .set_clock_format_to(*new_format);
    //         }
    //         TuiAction::UpdateRefreshRate(new_refresh_rate) => {
    //             state.refresh_rate = *new_refresh_rate
    //         }
    //         TuiAction::UpdateQuote(new_quote) => state.quote_idx = *new_quote,
    //         // == Pomodoro settings
    //         TuiAction::UpdateTotalSession(_new_total_sessions) => {}
    //         TuiAction::UpdateWorkDuration(_new_work_duration) => {}
    //         TuiAction::UpdateLongBreakDuration(_new_long_break_duration) => {}
    //         TuiAction::UpdateShortBreakDuration(_new_short_break_duration) => {}
    //         TuiAction::UpdateSessionsBeforeLongBreak(_new_sessions_before_long_break) => {}
    //         // == Color settings
    //         TuiAction::UpdateColorTheme(theme) => state.color_theme_idx = *theme,
    //         TuiAction::UpdateColor(variant, new_color) => {
    //             self.tui_assets.color_themes[state.color_theme_idx as usize]
    //                 .update(variant.clone(), *new_color);
    //         }
    //     }
    // }
    //
    // pub fn carousel_options_for(&self, setting: Setting) -> Vec<SelectableItem> {
    //     match setting {
    //         Setting::ClockFace => self
    //             .tui_assets
    //             .clock_faces
    //             .iter()
    //             .map(|clock_face| SelectableItem::ClockFace(clock_face))
    //             .collect(),
    //         Setting::ColorTheme => self
    //             .tui_assets
    //             .color_themes
    //             .iter()
    //             .map(|color_theme| SelectableItem::Theme(color_theme))
    //             .collect(),
    //         Setting::ClockFormat => TimeFormat::iter()
    //             .map(|fmt| SelectableItem::Format(fmt))
    //             .collect(),
    //         Setting::Quote => self
    //             .tui_assets
    //             .quotes
    //             .iter()
    //             .map(|quote| SelectableItem::Quote(Some(quote)))
    //             .chain(std::iter::once(SelectableItem::Quote(None)))
    //             .collect(),
    //         _ => unreachable!(),
    //     }
    // }

    pub fn handle_events(&self, components: &'a mut TuiComponents) -> io::Result<bool> {
        let app_state;
        let refresh_rate;
        {
            let state = self.tui_state.read().unwrap();
            if matches!(state.application_state, ApplicationState::Finished) {
                return Ok(true);
            }
            refresh_rate = state.refresh_rate;
            app_state = state.application_state.clone();
        }

        if event::poll(Duration::from_millis(refresh_rate))? {
            if let Event::Key(key_event) = event::read()? {
                if matches!(key_event.kind, event::KeyEventKind::Release) {
                    return Ok(false);
                }

                // Handle global keys first (like Esc to close buffer/windows)
                if self.handle_global_keys(key_event, components) {
                    let state = self.tui_state.read().unwrap();
                    let should_exit = matches!(state.application_state, ApplicationState::Finished);
                    return Ok(should_exit);
                }

                // Then handle state-specific keys
                match app_state {
                    ApplicationState::Running => self.handle_normal_keys(key_event, components),
                    ApplicationState::ShowingHero => self.handle_hero_keys(key_event, components),
                    ApplicationState::ShowingSettings => {
                        components
                            .settings_menu
                            .handle_setting_keys(key_event, self.tui_state);
                    }
                    ApplicationState::ShowingHelp | ApplicationState::Finished => {}
                }
            }
        }

        let state = self.tui_state.read().unwrap();
        let should_exit = matches!(state.application_state, ApplicationState::Finished);

        Ok(should_exit)
    }

    fn handle_global_keys(&self, key_event: KeyEvent, components: &mut TuiComponents) -> bool {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                let mut tui_state = self.tui_state.write().unwrap();

                match tui_state.application_state {
                    ApplicationState::ShowingHero => {
                        tui_state.application_state = ApplicationState::Running;
                        true
                    }
                    ApplicationState::ShowingHelp => {
                        if components.help_box.was_called_from_hero() {
                            tui_state.application_state = ApplicationState::ShowingHero;
                        } else {
                            tui_state.application_state = ApplicationState::Running;
                        }
                        components.help_box.set_called_from_hero(false);
                        true
                    }
                    ApplicationState::ShowingSettings => {
                        if components.settings_menu.was_called_from_hero() {
                            tui_state.application_state = ApplicationState::ShowingHero;
                        } else {
                            tui_state.application_state = ApplicationState::Running;
                        }
                        components.settings_menu.set_called_from_hero(false);
                        true
                    }
                    ApplicationState::Running => {
                        if matches!(key_event.code, KeyCode::Char('q')) {
                            tui_state.application_state = ApplicationState::Finished;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn handle_normal_keys(&self, key_event: KeyEvent, components: &mut TuiComponents) {
        let mut tui_state = self.tui_state.write().unwrap();

        match key_event.code {
            KeyCode::Esc => {
                tui_state.application_state = ApplicationState::ShowingHero;
            }
            KeyCode::Char('?') | KeyCode::Char('h') => {
                components.help_box.set_called_from_hero(false);
                tui_state.application_state = ApplicationState::ShowingHelp;
            }
            KeyCode::Char('s') => {
                components.settings_menu.set_called_from_hero(false);
                tui_state.application_state = ApplicationState::ShowingSettings;
            }
            _ => {}
        }
    }

    fn handle_hero_keys(&self, key_event: KeyEvent, components: &mut TuiComponents) {
        let mut tui_state = self.tui_state.write().unwrap();

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => components.hero.next_label(),
            KeyCode::Char('k') | KeyCode::Up => components.hero.prev_label(),
            KeyCode::Enter => match components.hero.active_label {
                MenuLabel::QUIT => tui_state.application_state = ApplicationState::Finished,
                MenuLabel::HELP => {
                    components.help_box.set_called_from_hero(true);
                    tui_state.application_state = ApplicationState::ShowingHelp;
                }
                MenuLabel::SETTINGS => {
                    components.settings_menu.set_called_from_hero(true);
                    tui_state.application_state = ApplicationState::ShowingSettings;
                }
            },
            _ => {}
        }
    }
}
