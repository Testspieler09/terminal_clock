// TODO: [display of color while user is typing] [the input field]
// i | a | e -> edit
// ESC -> leave no save / ask for save
// enter -> leave with save

use crate::tui_models::{
    selectable_item::SelectableItem, selector::SettingsSelector, settings::Setting,
    styled_widget::StyledWidget, tui::TuiAssets, tui_action::TuiAction, tui_error::UpdateResult,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, ThemeColor};

// TODO: add the global colorpicker component later on that gets rendered over the whole frame
pub(crate) struct ColorSelector {
    is_active: bool,

    setting: Setting,
    current_hex_color: String,
}

impl ColorSelector {
    pub fn new(is_active: bool, setting: Setting) -> ColorSelector {
        ColorSelector {
            is_active,
            setting,
            current_hex_color: "#343434".to_string(), // TODO: fetch from tui_controller
        }
    }
}

impl SettingsSelector for ColorSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match key_event.code {
            KeyCode::Char('e') | KeyCode::Char('i') | KeyCode::Char('a') => {
                todo!()
            }
            _ => None,
        }
    }

    fn set_to_active(&mut self) {
        self.is_active = true;
    }

    fn set_to_inactive(&mut self) {
        self.is_active = false;
    }

    fn update_current_selection(
        &mut self,
        selection: SelectableItem,
        tui_assets: &TuiAssets,
    ) -> UpdateResult<()> {
        todo!()
    }
}

impl StyledWidget for &ColorSelector {
    type Context<'a> = &'a ColorTheme;

    fn render(self, area: Rect, buf: &mut Buffer, color_theme: &ColorTheme) {
        let highlight_color = *color_theme.get(&ThemeColor::Selection);
        let default_color = *color_theme.get(&ThemeColor::Foreground);

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        let [title_row_section, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [_, button_right_section] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)])
                .areas(bottom_row_section);

        Line::from(vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ])
        .alignment(Alignment::Center)
        .style(style)
        .render(title_row_section, buf);

        Line::from(self.current_hex_color.clone())
            .alignment(Alignment::Center)
            .style(style)
            .render(bottom_row_section, buf);
        Span::from("⏎ ")
            .style(style)
            .render(button_right_section, buf);
    }
}
