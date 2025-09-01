use crate::{components::logo::Logo, helpers::generate_title};
use ratatui::{
    layout::Flex,
    prelude::{Alignment, Buffer, Constraint, Layout, Rect},
    style::Style,
    symbols::{
        border::{ROUNDED, Set},
        line::NORMAL,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::sync::Arc;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use tc_models::colorscheme::{ColorScheme, SchemeColor};

#[derive(Default, EnumIter, Clone, Copy, PartialEq, Eq, AsRefStr)]
pub enum SettingsTab {
    #[default]
    General,
    Pomodoro,
    Color,
}

pub struct SettingMenu {
    current_tab: SettingsTab,
    is_visible: bool,
    called_from_hero: bool,
    colorscheme: Arc<ColorScheme>,
}

impl SettingMenu {
    pub fn new(colorscheme: Arc<ColorScheme>) -> SettingMenu {
        SettingMenu {
            current_tab: SettingsTab::default(),
            is_visible: false,
            called_from_hero: false,
            colorscheme,
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

            // Color Settings for this widget
            let border_color = self.colorscheme.get(&SchemeColor::Comment);
            let tab_highlighting_color = self.colorscheme.get(&SchemeColor::Red);

            let box_height = 20;

            let [logo_section, setting_section] = Layout::vertical([
                Constraint::Length(logo_height),
                Constraint::Length(box_height),
            ])
            .margin((area.height - (logo_height + box_height)) / 2)
            .areas(area);

            let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
                .flex(Flex::Center)
                .areas(logo_section);

            let [box_layout] = Layout::horizontal([Constraint::Length(50)])
                .flex(Flex::Center)
                .areas(setting_section);

            logo.render(logo_layout, buf);

            let settings_block = Block::bordered()
                .title(generate_title("tab➔".to_string()))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(*border_color));

            let [_, header_section, content_section] = Layout::vertical([
                // This constraint is in place, to avoid writing the
                // tab labels over the border of the settings box
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(box_layout);

            let header_layout = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(header_section);

            settings_block.render(box_layout, buf);

            for (i, (tab, area)) in SettingsTab::iter()
                .zip(header_layout.into_iter())
                .enumerate()
            {
                let is_active = tab == self.current_tab;
                let text = if is_active {
                    Line::from(vec![
                        Span::from("[").style(Style::default().fg(*tab_highlighting_color)),
                        Span::from(tab.as_ref()),
                        Span::from("]").style(Style::default().fg(*tab_highlighting_color)),
                    ])
                } else {
                    Line::from(vec![
                        Span::from(format!("{}", i + 1))
                            .style(Style::default().fg(*tab_highlighting_color)),
                        Span::from(tab.as_ref().to_owned() + " "),
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
                .border_style(Style::default().fg(*border_color));

            let description_block = Block::bordered()
                .border_set(Set {
                    top_left: NORMAL.horizontal_down,
                    top_right: NORMAL.vertical_left,
                    bottom_left: NORMAL.horizontal_up,
                    ..ROUNDED
                })
                .border_style(Style::default().fg(*border_color));

            let [interactive_section, description_section] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
                    .areas(content_section);

            interactive_settings_block.render(interactive_section, buf);
            description_block.render(description_section, buf);

            match self.current_tab {
                SettingsTab::General => {}
                SettingsTab::Pomodoro => {}
                SettingsTab::Color => {}
            }
        }
    }
}
