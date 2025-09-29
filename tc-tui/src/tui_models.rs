use crate::components::{
    help_box::HelpBox,
    hero::{Hero, MenuLabel},
    logo::Logo,
    pomodoro::PomodoroTimer,
    settings_menu::SettingMenu,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Color,
};
use std::sync::{Arc, Mutex, RwLock};
use tc_models::{
    clock::{Clock, ClockBehaviour},
    color_theme::{ColorTheme, ThemeColor},
    quote::Quote,
    selectable_item::SelectableItem,
    tui_action::TuiAction,
};
use tc_user_config_loader::{
    LoaderResult, clock_face_loader::ClockFaceLoader, color_theme_loader::ColorThemeLoader,
    quote_loader::QuoteLoader,
};
use tokio::{io, time::Duration};

pub(crate) struct TuiAssets {
    pub color_themes: Vec<Arc<Mutex<ColorTheme>>>,
    pub clock_faces: Vec<Arc<Mutex<Clock>>>,
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

pub(crate) struct TuiState {
    pub application_state: ApplicationState,
    pub color_theme: Arc<Mutex<ColorTheme>>,
    pub clock_face: Arc<Mutex<Clock>>,
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
            help_box: HelpBox::new(Arc::clone(&controller)),
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

    pub fn process_settings_action(&self, action: &TuiAction) {
        let mut state = self.tui_state.write().unwrap();
        match action {
            // = Settings actions
            // == General settings
            TuiAction::UpdateClockFace(new_clock_face) => {
                state.clock_face = Arc::clone(new_clock_face)
            }
            TuiAction::UpdateClockFormat(new_format) => {
                let mut clock_lock = state.clock_face.lock().unwrap();
                clock_lock.set_clock_format_to(*new_format);
            }
            TuiAction::UpdateRefreshRate(new_refresh_rate) => {
                state.refresh_rate = *new_refresh_rate
            }
            TuiAction::UpdateQuote(new_quote) => state.quote = new_quote.clone(),
            // == Pomodoro settings
            TuiAction::UpdateTotalSession(new_total_sessions) => {}
            TuiAction::UpdateWorkDuration(new_work_duration) => {}
            TuiAction::UpdateLongBreakDuration(new_long_break_duration) => {}
            TuiAction::UpdateShortBreakDuration(new_short_break_duration) => {}
            TuiAction::UpdateSessionsBeforeLongBreak(new_sessions_before_long_break) => {}
            // == Color settings
            TuiAction::UpdateColorTheme(theme) => state.color_theme = Arc::clone(theme),
            TuiAction::UpdateColor(variant, new_color) => {
                let mut lock = state.color_theme.lock().unwrap();
                lock.update(variant.clone(), *new_color);
            }
        }
    }

    pub fn get_color_themes_as_selection(&self) -> Vec<SelectableItem> {
        self.tui_assets
            .color_themes
            .iter()
            .map(|color_theme| {
                let color_lock = color_theme.lock().unwrap();
                SelectableItem::Theme(Arc::new(color_lock.clone()))
            })
            .collect()
    }

    pub fn get_color(&self, key: &ThemeColor) -> Color {
        let state = self.tui_state.read().unwrap();
        let lock = state.color_theme.lock().unwrap();
        *lock.get(key)
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
                        components
                            .settings_menu
                            .handle_setting_keys(key_event, Arc::clone(&self.tui_state));
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
