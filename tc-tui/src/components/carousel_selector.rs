use crate::tui_models::TuiController;
use ratatui::{
    prelude::{Alignment, Buffer, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use std::sync::Arc;
use tc_models::color_theme::ThemeColor;

pub(crate) struct CarouselSelector {
    tui_controller: Arc<TuiController>,
    title: String,
    options: Vec<String>,
    current_selection: usize,
    action_callback: Box<dyn Fn(usize)>, // Callback to execute action
}

impl CarouselSelector {
    pub fn new(
        tui_controller: Arc<TuiController>,
        title: String,
        options: Vec<String>,
        action_callback: Box<dyn Fn(usize)>,
    ) -> CarouselSelector {
        CarouselSelector {
            tui_controller,
            title,
            options,
            current_selection: 0,
            action_callback,
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

    pub fn select(&self) {
        (self.action_callback)(self.current_selection);
    }
}

impl Widget for &CarouselSelector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let highlight_color = self.tui_controller.get_color(&ThemeColor::Selection);
        let default_color = self.tui_controller.get_color(&ThemeColor::Foreground);

        let title = Line::from(vec![
            Span::from(&self.title).style(Style::default().fg(default_color)),
        ]);

        let mut spans = Vec::new();

        for (index, option) in self.options.iter().enumerate() {
            let style = if index == self.current_selection {
                Style::default().fg(default_color).bg(highlight_color)
            } else {
                Style::default().fg(default_color)
            };

            spans.push(Span::from(format!("← {} → ", option)).style(style));
        }

        let text = Line::from(spans);

        let paragraph = Paragraph::new(vec![title, text])
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}
