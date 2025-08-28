use crate::components::logo::Logo;
use ratatui::widgets::Tabs;
use ratatui::{
    layout::Flex,
    prelude::{Buffer, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Widget},
};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

#[derive(Default, EnumIter, Clone, Copy, PartialEq, Eq, AsRefStr)]
pub enum SettingsTab {
    #[default]
    General,
    Pomodoro,
    Color,
}

#[derive(Default)]
pub struct SettingMenu {
    current_tab: SettingsTab,
    is_visible: bool,
    called_from_hero: bool,
}

impl SettingMenu {
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

    pub fn set_visibility(&mut self, visibility: bool, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
        self.is_visible = visibility;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }
}

impl Widget for &SettingMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            let logo = Logo::default();
            let logo_height = *logo.height() as u16;
            let logo_width = *logo.width() as u16;

            let box_height = 20;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(logo_height),
                        Constraint::Length(box_height),
                    ]
                    .as_ref(),
                )
                .margin((area.height - (logo_height + box_height)) / 2)
                .split(area);

            let logo_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(logo_width)].as_ref())
                .flex(Flex::Center)
                .split(chunks[0]);

            logo.render(logo_layout[0], buf);

            let settings_block = Block::default()
                .title(Line::from(vec![
                    Span::from("┐"),
                    Span::from("tab➔").style(Style::default().fg(Color::White)),
                    Span::from("┌"),
                ]))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .borders(Borders::ALL);

            // TODO: do not use the tabs struct, do it custom
            // [     1General        [Pomodoro]         3Color     ]
            // -----------------------------------------------------
            // |                 |                                 |
            let tabs = Tabs::new(
                SettingsTab::iter()
                    .enumerate()
                    .map(|(i, tab)| {
                        if tab == self.current_tab {
                            Line::from(vec![
                                Span::from("[").style(Style::default().fg(Color::DarkGray)),
                                Span::from(tab.as_ref().to_owned()),
                                Span::from("]").style(Style::default().fg(Color::DarkGray)),
                            ])
                        } else {
                            Line::from(vec![
                                Span::from(format!("{}", i + 1))
                                    .style(Style::default().fg(Color::DarkGray)),
                                Span::from(tab.as_ref().to_owned() + " "),
                            ])
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .block(settings_block)
            .highlight_style(Style::default())
            .divider("");

            let box_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(50)].as_ref())
                .flex(Flex::Center)
                .split(chunks[1]);

            tabs.render(box_layout[0], buf);

            match self.current_tab {
                SettingsTab::General => {}
                SettingsTab::Pomodoro => {}
                SettingsTab::Color => {}
            }
        }
    }
}
