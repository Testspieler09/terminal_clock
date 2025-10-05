use crate::{
    ApplicationState, TuiState,
    components::Dimensions,
    helpers::generate_title,
    tui_models::{
        selector::{Selector, SelectorType, SettingsSelector},
        settings::Setting,
        tui::TuiController,
    },
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect},
    style::Style,
    symbols::{
        border::{ROUNDED, Set},
        line::NORMAL,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};
use std::sync::{Arc, RwLock};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use tc_models::{color_theme::ThemeColor, tui_action::TuiAction};

// TODO: change the title from &str to an enum that maps the string to a variant, for the
// tuicontroller to handle it more clean
struct TabConfig<'a> {
    setting: Setting,
    description: &'a [&'a str],
    pub selector_type: SelectorType,
}

enum PrimitiveTabNavigationOperation {
    Next,
    Prev,
}

#[derive(EnumIter, Clone, Copy, AsRefStr)]
pub(crate) enum SettingsTab {
    General(u16),
    Pomodoro(u16),
    Color(u16),
}

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::General(0)
    }
}

impl PartialEq for SettingsTab {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::General(_), Self::General(_)) => true,
            (Self::Pomodoro(_), Self::Pomodoro(_)) => true,
            (Self::Color(_), Self::Color(_)) => true,
            _ => false,
        }
    }
}

pub(crate) struct SettingMenu {
    current_tab: SettingsTab,

    /// All the selectors for the options available to the user
    general_tab_selectors: Vec<Selector>,
    pomodoro_tab_selectors: Vec<Selector>,
    color_tab_selectors: Vec<Selector>,

    /// Needed for the navigation
    called_from_hero: bool,

    /// Fields for applying the action based on the changed option
    tui_controller: Arc<TuiController>,
    pending_action: Option<TuiAction>,

    height: u16,
    width: u16,
}

impl SettingMenu {
    const GENERAL_TAB_CONFIG: &[TabConfig<'_>] = &[
        TabConfig {
            setting: Setting::RefreshRate,
            description: &["The rate on which the screen gets refreshed"],
            selector_type: SelectorType::Number,
        },
        TabConfig {
            setting: Setting::ClockFace,
            description: &["The clock face you want to be displayed"],
            selector_type: SelectorType::Carousel,
        },
        TabConfig {
            setting: Setting::ClockFormat,
            description: &[
                "The format the time is displayed in.",
                "",
                "The options are HH:MM:SS, MM:HH:SS and HH:MM",
            ],
            selector_type: SelectorType::Carousel,
        },
        TabConfig {
            setting: Setting::Quote,
            description: &["The quote that is supposed to be rendered"],
            selector_type: SelectorType::Carousel,
        },
    ];

    const POMODORO_TAB_CONFIG: &[TabConfig<'_>] = &[
        TabConfig {
            setting: Setting::TotalSessions,
            description: &["The total number of Pomodoro sessions"],
            selector_type: SelectorType::Number,
        },
        TabConfig {
            setting: Setting::SessionsBeforeLongBreak,
            description: &["The number of sessions to complete before taking a long break"],
            selector_type: SelectorType::Number,
        },
        TabConfig {
            setting: Setting::WorkDuration,
            description: &["Duration of each focused work session (in minutes)"],
            selector_type: SelectorType::Number,
        },
        TabConfig {
            setting: Setting::ShortBreakDuration,
            description: &["Duration of a short break between work sessions (in minutes)"],
            selector_type: SelectorType::Number,
        },
        TabConfig {
            setting: Setting::LongBreakDuration,
            description: &["Duration of a long break after multiple sessions (in minutes)"],
            selector_type: SelectorType::Number,
        },
    ];

    const COLOR_TAB_CONFIG: &[TabConfig<'_>] = &[
        TabConfig {
            setting: Setting::ColorTheme,
            description: &["The overall color theme used across the application"],
            selector_type: SelectorType::Carousel,
        },
        TabConfig {
            setting: Setting::ForegroundColor,
            description: &["Color used for primary text and UI elements"],
            selector_type: SelectorType::Color,
        },
        TabConfig {
            setting: Setting::BackgroundColor,
            description: &[
                "Background color of the application interface",
                "",
                "Set this to `None` to get a transparent background",
            ],
            selector_type: SelectorType::Color,
        },
        TabConfig {
            setting: Setting::SelectionColor,
            description: &["Color used when selecting text or items"],
            selector_type: SelectorType::Color,
        },
        TabConfig {
            setting: Setting::AccentColor,
            description: &["Highlight color used for emphasis or active items"],
            selector_type: SelectorType::Color,
        },
        TabConfig {
            setting: Setting::BorderColor,
            description: &["Color used for borders and outlines"],
            selector_type: SelectorType::Color,
        },
    ];

    pub fn new(tui_controller: Arc<TuiController>) -> SettingMenu {
        let general_tab_selectors: Vec<Selector> = Self::GENERAL_TAB_CONFIG
            .iter()
            .enumerate()
            .map(|(idx, config)| {
                let is_active = idx == 0;
                config.selector_type.create_selector(
                    Arc::clone(&tui_controller),
                    config.setting,
                    is_active,
                )
            })
            .collect();
        let pomodoro_tab_selectors: Vec<Selector> = Self::POMODORO_TAB_CONFIG
            .iter()
            .map(|config| {
                config.selector_type.create_selector(
                    Arc::clone(&tui_controller),
                    config.setting,
                    false,
                )
            })
            .collect();
        let color_tab_selectors: Vec<Selector> = Self::COLOR_TAB_CONFIG
            .iter()
            .map(|config| {
                config.selector_type.create_selector(
                    Arc::clone(&tui_controller),
                    config.setting,
                    false,
                )
            })
            .collect();

        SettingMenu {
            current_tab: SettingsTab::default(),
            general_tab_selectors,
            pomodoro_tab_selectors,
            color_tab_selectors,
            called_from_hero: false,
            tui_controller,
            pending_action: None,
            height: 40u16,
            width: 75u16,
        }
    }

    fn reset_tab_page(&mut self) {
        for selector in self
            .general_tab_selectors
            .iter_mut()
            .chain(self.pomodoro_tab_selectors.iter_mut())
            .chain(self.color_tab_selectors.iter_mut())
        {
            selector.set_to_inactive();
        }

        match self.current_tab {
            SettingsTab::General(_) => self.general_tab_selectors[0].set_to_active(),
            SettingsTab::Pomodoro(_) => self.pomodoro_tab_selectors[0].set_to_active(),
            SettingsTab::Color(_) => self.color_tab_selectors[0].set_to_active(),
        };
    }

    fn display_tab(&mut self, tab: SettingsTab) {
        self.current_tab = tab;
        self.reset_tab_page();
    }

    fn next_tab(&mut self) {
        self.current_tab = SettingsTab::iter()
            .cycle()
            .skip_while(|tab| *tab != self.current_tab)
            .skip(1)
            .next()
            .unwrap();
        self.reset_tab_page();
    }

    fn prev_tab(&mut self) {
        let tabs = SettingsTab::iter().collect::<Vec<_>>();
        let current_position = tabs
            .iter()
            .position(|tab| *tab == self.current_tab)
            .unwrap();

        let previous_position = if current_position == 0 {
            tabs.len() - 1
        } else {
            current_position - 1
        };

        self.current_tab = tabs[previous_position];
        self.reset_tab_page();
    }

    fn update_option_index(&mut self, operation: PrimitiveTabNavigationOperation) {
        self.current_tab = match self.current_tab {
            SettingsTab::General(option_idx) => {
                self.general_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::GENERAL_TAB_CONFIG.len());
                self.general_tab_selectors[new_idx as usize].set_to_active();

                SettingsTab::General(new_idx)
            }
            SettingsTab::Pomodoro(option_idx) => {
                self.pomodoro_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::POMODORO_TAB_CONFIG.len());
                self.pomodoro_tab_selectors[new_idx as usize].set_to_active();

                SettingsTab::Pomodoro(new_idx)
            }
            SettingsTab::Color(option_idx) => {
                self.color_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::COLOR_TAB_CONFIG.len());
                self.color_tab_selectors[new_idx as usize].set_to_active();

                SettingsTab::Color(new_idx)
            }
        }
    }

    fn update_index(
        current_index: u16,
        operation: PrimitiveTabNavigationOperation,
        max_len: usize,
    ) -> u16 {
        let max_len = max_len as u16;
        if matches!(operation, PrimitiveTabNavigationOperation::Next) {
            (current_index + 1) % max_len
        } else if current_index == 0 {
            max_len - 1
        } else {
            current_index - 1
        }
    }

    fn next_settings_option(&mut self) {
        self.update_option_index(PrimitiveTabNavigationOperation::Next);
    }

    fn prev_settings_option(&mut self) {
        self.update_option_index(PrimitiveTabNavigationOperation::Prev);
    }

    pub fn set_called_from_hero(&mut self, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }

    fn render_tab(&self, lhs: Rect, rhs: Rect, buf: &mut Buffer) {
        let (selectors, idx, content) = match self.current_tab {
            SettingsTab::General(selected_idx) => (
                &self.general_tab_selectors,
                selected_idx,
                Self::GENERAL_TAB_CONFIG,
            ),
            SettingsTab::Pomodoro(selected_idx) => (
                &self.pomodoro_tab_selectors,
                selected_idx,
                Self::POMODORO_TAB_CONFIG,
            ),
            SettingsTab::Color(selected_idx) => (
                &self.color_tab_selectors,
                selected_idx,
                Self::COLOR_TAB_CONFIG,
            ),
        };

        let constraints = vec![Constraint::Length(2); selectors.len()];

        let layout = Layout::vertical(constraints).split(lhs);

        for (i, selector) in selectors.iter().enumerate() {
            if i < layout.len() {
                selector.render(layout[i], buf);
            }
        }

        if let Some(config) = content.get(idx as usize) {
            let desc_text = config.description.join("\n");
            let desc_paragraph = Paragraph::new(desc_text)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(self.tui_controller.get_color(&ThemeColor::Foreground)));
            desc_paragraph.render(rhs, buf);
        }
    }

    pub fn handle_setting_keys(&mut self, key_event: KeyEvent, tui_state: Arc<RwLock<TuiState>>) {
        self.pending_action = None;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => self.next_settings_option(),
            KeyCode::Char('k') | KeyCode::Up => self.prev_settings_option(),
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.prev_tab(),
            KeyCode::Char('1') => self.display_tab(SettingsTab::General(0)),
            KeyCode::Char('2') => self.display_tab(SettingsTab::Pomodoro(0)),
            KeyCode::Char('3') => self.display_tab(SettingsTab::Color(0)),
            KeyCode::Char('s') => {
                let mut tui_state = tui_state.write().unwrap();

                if self.was_called_from_hero() {
                    tui_state.application_state = ApplicationState::ShowingHero;
                } else {
                    tui_state.application_state = ApplicationState::Running;
                }
                self.set_called_from_hero(false);
            }
            _ => {
                self.pending_action = match self.current_tab {
                    SettingsTab::General(idx) => {
                        self.general_tab_selectors[idx as usize].handle_keys(key_event)
                    }
                    SettingsTab::Pomodoro(idx) => {
                        self.pomodoro_tab_selectors[idx as usize].handle_keys(key_event)
                    }
                    SettingsTab::Color(idx) => {
                        self.color_tab_selectors[idx as usize].handle_keys(key_event)
                    }
                };
            }
        }

        if let Some(action) = &self.pending_action {
            self.tui_controller.process_settings_action(action);
        }
    }
}

impl Dimensions for &SettingMenu {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }
}

impl Widget for &SettingMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Color Settings for this widget
        let fg_color = self.tui_controller.get_color(&ThemeColor::Foreground);
        let border_color = self.tui_controller.get_color(&ThemeColor::Borders);

        let settings_block = Block::bordered()
            .title(generate_title("tab➔".to_string(), fg_color))
            .style(Style::default().fg(fg_color))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        // TODO: refactor later
        // let [header_section, content_section] =
        //     Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
        //         .areas(settings_block.inner(area));
        let [_, header_section, content_section] = Layout::vertical([
            // This constraint is in place, to avoid writing the
            // tab labels over the border of the settings box
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let header_layout = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(header_section);

        settings_block.render(area, buf);

        for (i, (tab, area)) in SettingsTab::iter()
            .zip(header_layout.into_iter())
            .enumerate()
        {
            let is_active = tab == self.current_tab;
            let text = if is_active {
                Line::from(vec![
                    Span::from("[").style(Style::default().fg(border_color)),
                    Span::from(tab.as_ref()),
                    Span::from("]").style(Style::default().fg(border_color)),
                ])
            } else {
                Line::from(vec![
                    Span::from(format!("{}", i + 1)).style(Style::default().fg(border_color)),
                    Span::from(tab.as_ref().to_owned() + " ").style(Style::default().fg(fg_color)),
                ])
            };

            Paragraph::new(text)
                .alignment(Alignment::Center)
                .render(*area, buf);
        }

        let [interactive_section, description_section] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(content_section);

        let interactive_settings_block = Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_set(Set {
                top_left: NORMAL.vertical_right,
                ..ROUNDED
            })
            .style(Style::default().fg(fg_color))
            .border_style(Style::default().fg(border_color));

        let inner_interactive_block = interactive_settings_block.inner(interactive_section);

        let description_block = Block::bordered()
            .border_set(Set {
                top_left: NORMAL.horizontal_down,
                top_right: NORMAL.vertical_left,
                bottom_left: NORMAL.horizontal_up,
                ..ROUNDED
            })
            .style(Style::default().fg(fg_color))
            .border_style(Style::default().fg(border_color));

        let inner_description_block = description_block.inner(description_section);

        interactive_settings_block.render(interactive_section, buf);
        description_block.render(description_section, buf);

        self.render_tab(inner_interactive_block, inner_description_block, buf);
    }
}
