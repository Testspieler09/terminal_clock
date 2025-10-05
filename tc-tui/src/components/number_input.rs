// TODO: select from presets with custom value option
use crate::tui_models::{selector::SettingsSelector, tui::TuiController};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use std::sync::Arc;
use tc_models::{color_theme::ThemeColor, tui_action::TuiAction};

// TODO: add the global colorpicker component later on that gets rendered over the whole frame
pub(crate) struct NumberSelector {
    tui_controller: Arc<TuiController>,
    is_active: bool,

    title: String,
    current_selected_number: u32,
}

impl NumberSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        is_active: bool,
        title: String,
    ) -> NumberSelector {
        NumberSelector {
            tui_controller,
            is_active,
            title,
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
}

impl Widget for &NumberSelector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let highlight_color = self.tui_controller.get_color(&ThemeColor::Selection);
        let default_color = self.tui_controller.get_color(&ThemeColor::Foreground);

        let [_, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [_, button_right_section] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)])
                .areas(bottom_row_section);

        let title = Line::from(vec![
            Span::from(&self.title).style(Style::default().fg(default_color).bold()),
        ]);

        let text = Line::from(self.current_selected_number.to_string());
        Span::from("⏎ ").render(button_right_section, buf);

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        let paragraph = Paragraph::new(vec![title, text])
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(style);

        paragraph.render(area, buf);
    }
}
