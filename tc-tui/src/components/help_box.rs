use crate::components::logo::Logo;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

const CONTENT: &str = "h .. Toggle this help box\nq .. Quit this program";

#[derive(Default, Clone)]
pub(crate) struct HelpBox {
    is_visible: bool,
}

impl HelpBox {
    pub fn set_visibility(&mut self, visibility: bool) {
        self.is_visible = visibility;
    }
}

impl Widget for HelpBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            Logo.render(area, buf);

            let block = Block::default()
                .title("┤ Help Box ├")
                .style(Style::default().bg(Color::Black))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL);

            block.clone().render(area, buf);

            let inner_area = block.inner(area);

            // Render the help text within the computed inner area
            for (i, line) in CONTENT.split('\n').enumerate() {
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
