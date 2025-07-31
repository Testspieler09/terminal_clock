use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Clone)]
pub struct HelpBox<'a> {
    visible: bool,
    content: &'a str,
}

impl<'a> Default for HelpBox<'a> {
    fn default() -> Self {
        Self {
            visible: false,
            content: "H .. toggle this help message\nQ .. Quit",
        }
    }
}

impl<'a> HelpBox<'a> {
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl<'a> Widget for HelpBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.visible {
            let block = Block::default()
                .title("┤ Help Box ├")
                .style(Style::default().bg(Color::Black))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL);

            block.clone().render(area, buf);

            let inner_area = block.inner(area);

            // Render the help text within the computed inner area
            let lines: Vec<&str> = self.content.split('\n').collect();
            for (i, line) in lines.iter().enumerate() {
                buf.set_string(
                    inner_area.x,
                    inner_area.y + i as u16,
                    line,
                    Style::default().fg(Color::Green),
                );
            }
        }
    }
}
