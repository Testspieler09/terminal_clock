use crate::components::{
    help_box::HelpBox,
    hero::{Hero, MenuLabel},
    logo::Logo,
    pomodoro::PomodoroTimer,
    settings_menu::{SettingMenu, SettingsAction, SettingsTab},
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Color,
};
use std::sync::{Arc, RwLock};
use tc_models::{
    clock::Clock,
    color_theme::{ColorTheme, ThemeColor},
    quote::Quote,
};
use tc_user_config_loader::{
    LoaderResult, clock_face_loader::ClockFaceLoader, color_theme_loader::ColorThemeLoader,
    quote_loader::QuoteLoader,
};
use tokio::{io, time::Duration};

#[derive(Clone)]
pub(crate) struct TuiAssets {
    pub color_themes: Vec<Arc<ColorTheme>>,
    pub clock_faces: Vec<Arc<dyn Clock>>,
    pub quotes: Vec<Arc<Quote>>,
}

impl TuiAssets {
    pub fn try_default() -> LoaderResult<TuiAssets> {
        Ok(TuiAssets {
            color_themes: ColorThemeLoader::load_color_themes()?,
            clock_faces: ClockFaceLoader::load_clockfaces()?,
            quotes: QuoteLoader::load_quotes()?,
        })
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct TuiState {
    pub application_state: ApplicationState,
    pub colorscheme: Arc<ColorTheme>,
    pub clock_face: Arc<dyn Clock>,
    pub quote: Option<Arc<Quote>>,
    pub pomodoro: Option<Arc<PomodoroTimer>>,
    pub refresh_rate: u64,
}

pub(crate) struct TuiComponents {
    pub help_box: HelpBox,
    pub settings_menu: SettingMenu,
    pub hero: Hero,
    pub logo: Logo,
}

impl TuiComponents {
    pub fn new(controller: Arc<TuiController>) -> TuiComponents {
        TuiComponents {
            help_box: HelpBox::new(controller.clone()),
            settings_menu: SettingMenu::new(controller),
            hero: Hero::default(),
            logo: Logo::default(),
        }
    }
}

pub(crate) struct TuiController {
    tui_state: Arc<RwLock<TuiState>>,
    tui_assets: Arc<TuiAssets>,
}

impl TuiController {
    pub fn new(tui_state: Arc<RwLock<TuiState>>, tui_assets: Arc<TuiAssets>) -> Self {
        TuiController {
            tui_state,
            tui_assets,
        }
    }

    pub fn process_settings_action(&mut self, action: SettingsAction) {
        let mut state = self.tui_state.write().unwrap();
        match action {
            SettingsAction::UpdateColor(variant, new_color) => {}
            SettingsAction::UpdateQuote(new_quote) => {}
            SettingsAction::UpdateClockFace(new_clock_face) => {}
            SettingsAction::UpdateRefreshRate(new_refresh_rate) => {}
            _ => {}
        }
    }

    pub fn get_color(&self, key: &ThemeColor) -> Color {
        let state = self.tui_state.read().unwrap();
        *state.colorscheme.get(key)
    }

    pub fn handle_events(&self, components: &mut TuiComponents) -> io::Result<bool> {
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
                        self.handle_setting_keys(key_event, components)
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

    fn handle_setting_keys(&self, key_event: KeyEvent, components: &mut TuiComponents) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {}
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('h') | KeyCode::Left => {}
            KeyCode::Char('l') | KeyCode::Right => {}
            KeyCode::Tab => components.settings_menu.next_tab(),
            KeyCode::BackTab => components.settings_menu.prev_tab(),
            KeyCode::Char('1') => components
                .settings_menu
                .display_tab(SettingsTab::General(0)),
            KeyCode::Char('2') => components
                .settings_menu
                .display_tab(SettingsTab::Pomodoro(0)),
            KeyCode::Char('3') => components.settings_menu.display_tab(SettingsTab::Color(0)),
            KeyCode::Char('s') => {
                let mut tui_state = self.tui_state.write().unwrap();

                if components.settings_menu.was_called_from_hero() {
                    tui_state.application_state = ApplicationState::ShowingHero;
                } else {
                    tui_state.application_state = ApplicationState::Running;
                }
                components.settings_menu.set_called_from_hero(false);
            }
            _ => {}
        }
    }
}
