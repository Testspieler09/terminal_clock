use crate::{components::Dimensions, helpers::generate_title, tui_models::TuiController};
use ratatui::{
    prelude::{Alignment, Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::{
        border::{ROUNDED, Set},
        line::NORMAL,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::sync::Arc;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use tc_models::{clock::Clock, colorscheme::SchemeColor, quote::Quote};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, AsRefStr)]
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

impl SettingsTab {
    pub fn next_option() {}
    pub fn prev_option() {}
}

pub(crate) enum SettingsAction {
    UpdateClockFace(Arc<dyn Clock>),
    UpdateColor(SchemeColor, Color),
    UpdateQuote(Arc<Quote>),
    UpdateRefreshRate(u64),
}

pub(crate) struct SettingMenu {
    current_tab: SettingsTab,
    called_from_hero: bool,
    tui_controller: Arc<TuiController>,
    pending_action: Option<SettingsAction>,
    height: u16,
    width: u16,
}

impl SettingMenu {
    pub fn new(tui_controller: Arc<TuiController>) -> SettingMenu {
        SettingMenu {
            current_tab: SettingsTab::default(),
            called_from_hero: false,
            tui_controller,
            pending_action: None,
            height: 40u16,
            width: 75u16,
        }
    }

    pub fn display_tab(&mut self, tab: SettingsTab) {
        self.current_tab = tab;
    }

    pub fn next_label(&mut self) {
        self.current_tab = SettingsTab::iter()
            .cycle()
            .skip_while(|tab| *tab != self.current_tab)
            .skip(1)
            .next()
            .unwrap();
    }

    pub fn prev_label(&mut self) {
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
    }

    pub fn set_called_from_hero(&mut self, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }

    fn render_general_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect) {}
    fn render_pomodoro_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect) {}
    fn render_color_tab(&self, selected_idx: u16, lhs: Rect, rhs: Rect) {}
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
        let fg_color = self.tui_controller.get_color(&SchemeColor::Foreground);
        let border_color = self.tui_controller.get_color(&SchemeColor::Borders);
        let selection_color = self.tui_controller.get_color(&SchemeColor::Selection);

        let settings_block = Block::bordered()
            .title(generate_title("tab➔".to_string(), fg_color))
            .style(Style::default().fg(fg_color))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

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

        let interactive_settings_block = Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_set(Set {
                top_left: NORMAL.vertical_right,
                ..ROUNDED
            })
            .style(Style::default().fg(fg_color))
            .border_style(Style::default().fg(border_color));

        let description_block = Block::bordered()
            .border_set(Set {
                top_left: NORMAL.horizontal_down,
                top_right: NORMAL.vertical_left,
                bottom_left: NORMAL.horizontal_up,
                ..ROUNDED
            })
            .style(Style::default().fg(fg_color))
            .border_style(Style::default().fg(border_color));

        let [interactive_section, description_section] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(content_section);

        interactive_settings_block.render(interactive_section, buf);
        description_block.render(description_section, buf);

        match self.current_tab {
            SettingsTab::General(selected_entry) => {
                self.render_general_tab(selected_entry, interactive_section, description_section)
            }
            SettingsTab::Pomodoro(selected_entry) => {
                self.render_pomodoro_tab(selected_entry, interactive_section, description_section)
            }
            SettingsTab::Color(selected_entry) => {
                self.render_color_tab(selected_entry, interactive_section, description_section)
            }
        }
    }
}
