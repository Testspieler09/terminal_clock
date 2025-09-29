use crate::{components::settings_menu::SettingsSelector, tui_models::TuiController};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Constraint,
    prelude::{Alignment, Buffer, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use std::sync::Arc;
use tc_models::{
    color_theme::ThemeColor,
    selectable_item::{Selectable, SelectableItem},
    tui_action::TuiAction,
};

pub(crate) struct CarouselSelector {
    /// Fields needed for event handling logic
    tui_controller: Arc<TuiController>,
    is_active: bool,

    /// Display fields
    title: String,
    options: Vec<SelectableItem>,
    current_selection: usize,
}

// TODO: we need some way of setting the current_selection over the tui_controller
impl CarouselSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        title: String,
        options: Vec<SelectableItem>,
        is_active: bool,
    ) -> CarouselSelector {
        if options.is_empty() {
            panic!("A carousel selector should always contain values.");
        }

        CarouselSelector {
            tui_controller,
            is_active,
            title,
            options,
            current_selection: 0,
        }
    }

    fn next_option(&mut self) {
        if !self.options.is_empty() {
            self.current_selection = (self.current_selection + 1) % self.options.len();
        }
    }

    fn prev_option(&mut self) {
        if !self.options.is_empty() {
            if self.current_selection == 0 {
                self.current_selection = self.options.len() - 1;
            } else {
                self.current_selection -= 1;
            }
        }
    }
}

impl SettingsSelector for CarouselSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match key_event.code {
            KeyCode::Char('h') | KeyCode::Left => {
                self.next_option();
                Some(
                    self.options[self.current_selection]
                        .clone()
                        .get_corrosponding_action(),
                )
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.prev_option();
                Some(
                    self.options[self.current_selection]
                        .clone()
                        .get_corrosponding_action(),
                )
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

impl Widget for &CarouselSelector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let highlight_color = self.tui_controller.get_color(&ThemeColor::Selection);
        let default_color = self.tui_controller.get_color(&ThemeColor::Foreground);

        let [_, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [button_left_section, _, button_right_section] = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(bottom_row_section);

        let title = Line::from(vec![
            // TODO: n / N if active
            Span::from(&self.title).style(Style::default().fg(default_color).bold()),
        ]);

        Span::from(" ←").render(button_left_section, buf);
        let text = Line::from(self.options[self.current_selection].get_name());
        Span::from("→ ").render(button_right_section, buf);

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
