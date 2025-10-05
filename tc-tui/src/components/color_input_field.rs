// TODO: [display of color while user is typing] [the input field]
// i | a | e -> edit
// ESC -> leave no save / ask for save
// enter -> leave with save

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
pub(crate) struct ColorSelector {
    tui_controller: Arc<TuiController>,
    is_active: bool,

    title: String,
    current_hex_color: String,
}

impl ColorSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        is_active: bool,
        title: String,
    ) -> ColorSelector {
        ColorSelector {
            tui_controller,
            is_active,
            title,
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
}

impl Widget for &ColorSelector {
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

        let text = Line::from(self.current_hex_color.clone());
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
