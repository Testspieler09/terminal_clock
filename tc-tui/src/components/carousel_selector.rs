use crate::{components::settings_menu::SettingsAction, tui_models::TuiController};
use ratatui::{
    layout::Constraint,
    prelude::{Alignment, Buffer, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use std::sync::Arc;
use tc_models::color_theme::ThemeColor;

pub(crate) struct CarouselSelector {
    /// Fields needed for event handling logic
    tui_controller: Arc<TuiController>,
    is_active: bool,

    /// Display fields
    title: String,
    options: Vec<String>,
    current_selection: usize,
}

impl CarouselSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        title: String,
        options: Vec<String>,
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

    pub fn next_option(&mut self) {
        if !self.options.is_empty() {
            self.current_selection = (self.current_selection + 1) % self.options.len();
        }
    }

    pub fn prev_option(&mut self) {
        if !self.options.is_empty() {
            if self.current_selection == 0 {
                self.current_selection = self.options.len() - 1;
            } else {
                self.current_selection -= 1;
            }
        }
    }

    pub fn set_to_active(&mut self) {
        self.is_active = true;
    }

    pub fn set_to_inactive(&mut self) {
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
        let text = Line::from(self.options[self.current_selection].clone());
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
