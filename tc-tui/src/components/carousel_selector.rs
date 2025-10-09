use crate::tui_models::{
    selector::SettingsSelector,
    settings::Setting,
    tui::TuiController,
    tui_error::{UpdateError, UpdateResult},
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Constraint,
    prelude::{Alignment, Buffer, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
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
    setting: Setting,
    options: Vec<SelectableItem>,
    current_selection: usize,
}

impl CarouselSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        is_active: bool,
        setting: Setting,
        options: Vec<SelectableItem>,
    ) -> CarouselSelector {
        if options.is_empty() {
            panic!("A carousel selector should always contain values.");
        }

        CarouselSelector {
            tui_controller,
            is_active,
            setting,
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
                self.prev_option();
                Some(
                    self.options[self.current_selection]
                        .clone()
                        .get_corrosponding_action(),
                )
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.next_option();
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

    fn update_current_selection(&mut self, selection: SelectableItem) -> UpdateResult<()> {
        if let Some(idx) = self
            .options
            .iter()
            .position(|item| *item.get_name() == *selection.get_name())
        {
            self.current_selection = idx;
            Ok(())
        } else {
            Err(UpdateError)
        }
    }
}

impl Widget for &CarouselSelector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let highlight_color = self.tui_controller.get_color(&ThemeColor::Selection);
        let default_color = self.tui_controller.get_color(&ThemeColor::Foreground);

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        let [title_section, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [button_left_section, option_section, button_right_section] = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(bottom_row_section);

        let mut spans = vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ];

        let option_amount = self.options.len() - 1;
        if option_amount >= 2 {
            spans.push(Span::from(
                " ".to_owned()
                    + &(self.current_selection + 1).to_string()
                    + "/"
                    + &(option_amount + 1).to_string(),
            ))
        }

        // FIX: Too much text for the box leads to unreadability e.g. the quotes should have
        // scrollable text

        // Render Title
        Line::from(spans)
            .alignment(Alignment::Center)
            .style(style)
            .render(title_section, buf);

        // Render Options
        Span::from(" ← ")
            .style(style)
            .render(button_left_section, buf);
        Line::from(self.options[self.current_selection].get_name())
            .alignment(Alignment::Center)
            .style(style)
            .render(option_section, buf);
        Span::from(" → ")
            .style(style)
            .render(button_right_section, buf);
    }
}
