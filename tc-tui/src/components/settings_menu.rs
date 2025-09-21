use crate::{
    ApplicationState, TuiState,
    components::{Dimensions, carousel_selector::CarouselSelector},
    helpers::generate_title,
    tui_models::TuiController,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::{
        border::{ROUNDED, Set},
        line::NORMAL,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};
use std::sync::{Arc, RwLock};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use tc_models::{
    clock::{Clock, TimeFormat},
    color_theme::{ColorTheme, ThemeColor},
    quote::Quote,
};

enum PrimitiveOperation {
    Increment,
    Decrement,
}

#[derive(EnumIter, Clone, Copy, AsRefStr)]
pub(crate) enum SettingsTab {
    General(u16),
    Pomodoro(u16),
    Color(u16),
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

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::General(0)
    }
}

pub(crate) enum SettingsAction {
    /// General Tab
    UpdateRefreshRate(u64),
    UpdateClockFace(Arc<dyn Clock>),
    UpdateClockFormat(TimeFormat),
    UpdateQuote(Arc<Quote>),

    /// Pomodoro Tab
    UpdateTotalSession(u32),
    UpdateSessionsBeforeLongBreak(u32),
    UpdateWorkDuration(u64),
    UpdateShortBreakDuration(u64),
    UpdateLongBreakDuration(u64),

    /// Color Tab
    UpdateColorTheme(ColorTheme),
    UpdateColor(ThemeColor, Color),
}

pub(crate) trait SettingsSelector {
    fn handle_keys(key_event: KeyEvent) -> SettingsAction;
}

pub(crate) struct SettingMenu {
    current_tab: SettingsTab,

    /// All the selectors for the options available to the user
    // TODO: Needs to be more dynamic later i.e. <dyn Selector>
    general_tab_selectors: Vec<CarouselSelector>,
    pomodoro_tab_selectors: Vec<CarouselSelector>,
    color_tab_selectors: Vec<CarouselSelector>,

    /// Needed for the navigation
    called_from_hero: bool,

    /// Fields for applying the action based on the changed option
    tui_controller: Arc<TuiController>,
    pending_action: Option<SettingsAction>,

    height: u16,
    width: u16,
}

impl SettingMenu {
    const GENERAL_TAB_CONTENT: [(&str, &[&str]); 4] = [
        (
            "Refresh Rate",
            &["The rate on which the screen gets refreshed"],
        ),
        ("Clock Face", &["The clock face you want to be displayed"]),
        (
            "Clock Format",
            &[
                "The format the time is displayed in.",
                "",
                "The options are HH:MM:SS, MM:HH:SS and HH:MM",
            ],
        ),
        ("Quote", &["The quote that is supposed to be rendered"]),
    ];

    const POMODORO_TAB_CONTENT: [(&str, &[&str]); 5] = [
        ("Total Sessions", &["The total number of Pomodoro sessions"]),
        (
            "Sessions Before Long Break",
            &["The number of sessions to complete before taking a long break"],
        ),
        (
            "Work Duration",
            &["Duration of each focused work session (in minutes)"],
        ),
        (
            "Short Break Duration",
            &["Duration of a short break between work sessions (in minutes)"],
        ),
        (
            "Long Break Duration",
            &["Duration of a long break after multiple sessions (in minutes)"],
        ),
    ];

    const COLOR_TAB_CONTENT: [(&str, &[&str]); 6] = [
        (
            "Color Theme",
            &["The overall color theme used across the application"],
        ),
        (
            "Foreground Color",
            &["Color used for primary text and UI elements"],
        ),
        (
            "Background Color",
            &[
                "Background color of the application interface",
                "",
                "Set this to `None` to get a transparent background",
            ],
        ),
        (
            "Selection Color",
            &["Color used when selecting text or items"],
        ),
        (
            "Accent Color",
            &["Highlight color used for emphasis or active items"],
        ),
        ("Border Color", &["Color used for borders and outlines"]),
    ];

    pub fn new(tui_controller: Arc<TuiController>) -> SettingMenu {
        // TODO: init the tab selectors here
        let mut general_tab_selectors: Vec<CarouselSelector> = Self::GENERAL_TAB_CONTENT
            .iter()
            .map(|(title, _)| {
                let options = match *title {
                    "Refresh Rate" => vec![
                        "30 FPS".to_string(),
                        "60 FPS".to_string(),
                        "120 FPS".to_string(),
                    ],
                    "Clock Face" => vec![
                        "Analog".to_string(),
                        "Digital".to_string(),
                        "Binary".to_string(),
                    ],
                    "Clock Format" => vec![
                        "HH:MM:SS".to_string(),
                        "MM:HH:SS".to_string(),
                        "HH:MM".to_string(),
                    ],
                    "Quote" => vec![
                        "Inspirational".to_string(),
                        "Funny".to_string(),
                        "None".to_string(),
                    ],
                    _ => vec![
                        "Option 1".to_string(),
                        "Option 2".to_string(),
                        "Option 3".to_string(),
                    ],
                };
                CarouselSelector::new(
                    Arc::clone(&tui_controller),
                    title.to_string(),
                    options,
                    false,
                )
            })
            .collect();
        general_tab_selectors[0].set_to_active();

        let mut pomodoro_tab_selectors: Vec<CarouselSelector> = Self::POMODORO_TAB_CONTENT
            .iter()
            .map(|(title, _)| {
                let options = match *title {
                    "Total Sessions" => vec!["4".to_string(), "8".to_string(), "12".to_string()],
                    "Sessions Before Long Break" => {
                        vec!["3".to_string(), "4".to_string(), "5".to_string()]
                    }
                    "Work Duration" => vec![
                        "20 min".to_string(),
                        "25 min".to_string(),
                        "30 min".to_string(),
                    ],
                    "Short Break Duration" => vec![
                        "5 min".to_string(),
                        "10 min".to_string(),
                        "15 min".to_string(),
                    ],
                    "Long Break Duration" => vec![
                        "15 min".to_string(),
                        "20 min".to_string(),
                        "30 min".to_string(),
                    ],
                    _ => vec![
                        "Value 1".to_string(),
                        "Value 2".to_string(),
                        "Value 3".to_string(),
                    ],
                };
                CarouselSelector::new(
                    Arc::clone(&tui_controller),
                    title.to_string(),
                    options,
                    false,
                )
            })
            .collect();
        pomodoro_tab_selectors[0].set_to_active();

        let mut color_tab_selectors: Vec<CarouselSelector> = Self::COLOR_TAB_CONTENT
            .iter()
            .map(|(title, _)| {
                let options = match *title {
                    "Color Theme" => vec![
                        "Dark".to_string(),
                        "Light".to_string(),
                        "Custom".to_string(),
                    ],
                    "Foreground Color" => {
                        vec!["White".to_string(), "Black".to_string(), "Gray".to_string()]
                    }
                    "Background Color" => vec![
                        "Black".to_string(),
                        "None".to_string(),
                        "Dark Gray".to_string(),
                    ],
                    "Selection Color" => vec![
                        "Blue".to_string(),
                        "Green".to_string(),
                        "Yellow".to_string(),
                    ],
                    "Accent Color" => {
                        vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()]
                    }
                    "Border Color" => {
                        vec!["Gray".to_string(), "White".to_string(), "Black".to_string()]
                    }
                    _ => vec![
                        "Color 1".to_string(),
                        "Color 2".to_string(),
                        "Color 3".to_string(),
                    ],
                };
                CarouselSelector::new(
                    Arc::clone(&tui_controller),
                    title.to_string(),
                    options,
                    false,
                )
            })
            .collect();
        color_tab_selectors[0].set_to_active();

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
        }
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

    fn update_option_index(&mut self, operation: PrimitiveOperation) {
        self.current_tab = match self.current_tab {
            SettingsTab::General(option_idx) => {
                self.general_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::GENERAL_TAB_CONTENT.len());
                self.general_tab_selectors[new_idx as usize].set_to_active();
                SettingsTab::General(new_idx)
            }
            SettingsTab::Pomodoro(option_idx) => {
                self.pomodoro_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::POMODORO_TAB_CONTENT.len());
                self.pomodoro_tab_selectors[new_idx as usize].set_to_active();
                SettingsTab::Pomodoro(new_idx)
            }
            SettingsTab::Color(option_idx) => {
                self.color_tab_selectors[option_idx as usize].set_to_inactive();
                let new_idx =
                    Self::update_index(option_idx, operation, Self::COLOR_TAB_CONTENT.len());
                self.color_tab_selectors[new_idx as usize].set_to_active();
                SettingsTab::Color(new_idx)
            }
        }
    }

    fn update_index(current_index: u16, operation: PrimitiveOperation, max_len: usize) -> u16 {
        let max_len = max_len as u16;
        if matches!(operation, PrimitiveOperation::Increment) {
            (current_index + 1) % max_len
        } else if current_index == 0 {
            max_len - 1
        } else {
            current_index - 1
        }
    }

    fn next_settings_option(&mut self) {
        self.update_option_index(PrimitiveOperation::Increment);
    }

    fn prev_settings_option(&mut self) {
        self.update_option_index(PrimitiveOperation::Decrement);
    }

    pub fn set_called_from_hero(&mut self, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }

    // TODO: merge all the render tab functions into one general one, when we have the Selector
    // Trait
    fn render_general_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect, buf: &mut Buffer) {
        // TODO:
        // - refresh_rate
        // - clock_face
        // - clock-format
        // - quote
        // - rounded_corners
        // - ...
    }
    fn render_pomodoro_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect, buf: &mut Buffer) {
        // TODO:
        // - work_duration
        // - short_break_duration
        // - long_break_duration
        // - total_sessions
        // - sessions_before_long_break
    }
    fn render_color_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect, buf: &mut Buffer) {
        let constraints = vec![Constraint::Length(2); self.color_tab_selectors.len()];

        let layout = Layout::vertical(constraints).split(lhs);

        for (i, selector) in self.color_tab_selectors.iter().enumerate() {
            if i < layout.len() {
                selector.render(layout[i], buf);
            }
        }

        if let Some((_title, description)) = Self::COLOR_TAB_CONTENT.get(selected_idx as usize) {
            let desc_text = description.join("\n");
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
            KeyCode::Char('h') | KeyCode::Left => {
                /* self.pending_action = */
                match self.current_tab {
                    SettingsTab::General(idx) => {
                        self.general_tab_selectors[idx as usize].next_option()
                    }
                    SettingsTab::Pomodoro(idx) => {
                        self.pomodoro_tab_selectors[idx as usize].next_option()
                    }
                    SettingsTab::Color(idx) => self.color_tab_selectors[idx as usize].next_option(),
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                /* self.pending_action = */
                match self.current_tab {
                    SettingsTab::General(idx) => {
                        self.general_tab_selectors[idx as usize].prev_option()
                    }
                    SettingsTab::Pomodoro(idx) => {
                        self.pomodoro_tab_selectors[idx as usize].prev_option()
                    }
                    SettingsTab::Color(idx) => self.color_tab_selectors[idx as usize].prev_option(),
                }
            }
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
            _ => {}
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
        let selection_color = self.tui_controller.get_color(&ThemeColor::Selection);

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
                    Span::from("[").style(Style::default().fg(selection_color)),
                    Span::from(tab.as_ref()),
                    Span::from("]").style(Style::default().fg(selection_color)),
                ])
            } else {
                Line::from(vec![
                    Span::from(format!("{}", i + 1)).style(Style::default().fg(selection_color)),
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

        match self.current_tab {
            SettingsTab::General(selected_entry) => self.render_general_tab(
                selected_entry,
                inner_interactive_block,
                inner_description_block,
                buf,
            ),
            SettingsTab::Pomodoro(selected_entry) => self.render_pomodoro_tab(
                selected_entry,
                inner_interactive_block,
                inner_description_block,
                buf,
            ),
            SettingsTab::Color(selected_entry) => self.render_color_tab(
                selected_entry,
                inner_interactive_block,
                inner_description_block,
                buf,
            ),
        }
    }
}
