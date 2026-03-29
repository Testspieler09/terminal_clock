// TODO: select from presets with custom value option
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, ThemeColor};

use crate::tui_models::{
    selectable_item::SelectableItem, selector::SettingsSelector, settings::Setting,
    styled_widget::StyledWidget, tui::TuiAssets, tui_action::TuiAction, tui_error::UpdateResult,
};

// TODO: add the global colorpicker component later on that gets rendered over the whole frame
pub(crate) struct NumberSelector {
    is_active: bool,

    setting: Setting,
    current_selected_number: u32,
}

impl NumberSelector {
    pub fn new(is_active: bool, setting: Setting) -> NumberSelector {
        NumberSelector {
            is_active,
            setting,
            current_selected_number: 0, // TODO: fetch from tui_controller
        }
    }
}

impl SettingsSelector for NumberSelector {
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

impl StyledWidget for &NumberSelector {
    type Context<'a> = &'a ColorTheme;

    fn render(self, area: Rect, buf: &mut Buffer, color_theme: &ColorTheme) {
        let highlight_color = *color_theme.get(&ThemeColor::Selection);
        let default_color = *color_theme.get(&ThemeColor::Foreground);

        let [_, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [left_section, text_middle_section, button_right_section] = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(bottom_row_section);

        let title = Line::from(vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ]);

        let text = Line::from(self.current_selected_number.to_string());

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        Line::from(title)
            .alignment(Alignment::Center)
            .style(style)
            .render(area, buf);

        Span::from("   ").style(style).render(left_section, buf);
        Line::from(text)
            .alignment(Alignment::Center)
            .style(style)
            .render(text_middle_section, buf);
        Span::from(" ⏎ ")
            .style(style)
            .render(button_right_section, buf);
    }
}
