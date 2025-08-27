use crate::components::logo::Logo;
use ratatui::{
    buffer::Buffer,
    layout::{Flex, Rect},
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Widget},
};

// FIX: this will never change -> static should not be computed every time it is rendered
pub(crate) struct HelpBox {
    height: u16,
    width: u16,
    is_visible: bool,
}

impl Default for HelpBox {
    fn default() -> Self {
        HelpBox {
            height: HelpBox::CONTENT.len() as u16 + 3, // Border (2) + Tableheader (1)
            width: HelpBox::CONTENT
                .iter()
                .map(|pair| pair[0].len() + pair[1].len() + 15)
                .max()
                .unwrap_or(0) as u16,
            is_visible: false,
        }
    }
}

impl HelpBox {
    const CONTENT: [[&str; 2]; 3] = [
        ["?, h", "Toggle this help box"],
        ["q", "Quit this program"],
        ["ESC", "Toggles main menu"],
    ];

    pub fn set_visibility(&mut self, visibility: bool) {
        self.is_visible = visibility;
    }
}

impl Widget for &HelpBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            let logo = Logo::default();
            let logo_height = *logo.height() as u16;
            let logo_width = *logo.width() as u16;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(logo_height),
                        Constraint::Length(self.height),
                    ]
                    .as_ref(),
                )
                .margin((area.height - (logo_height + self.height)) / 2)
                .split(area);

            let logo_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(logo_width)].as_ref())
                .flex(Flex::Center)
                .split(chunks[0]);

            let box_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(self.width)].as_ref())
                .flex(Flex::Center)
                .split(chunks[1]);

            // Render the Logo
            logo.render(logo_layout[0], buf);

            let block = Block::default()
                .title(Line::from(vec![
                    Span::from("┐"),
                    Span::from("help").style(Style::default().fg(Color::White)),
                    Span::from("┌"),
                ]))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .borders(Borders::ALL);

            let inner_area = block.inner(box_layout[0]);

            // Render the HelpBox
            block.render(box_layout[0], buf);

            let table_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(inner_area);

            fn center_text(buf: &mut Buffer, area: Rect, text: &str, style: Style) {
                let text_width = text.len() as u16;
                let offset = (area.width.saturating_sub(text_width)) / 2;
                buf.set_string(area.x + offset, area.y, text, style);
            }

            // Render table headers
            center_text(
                buf,
                table_layout[0],
                "Key:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

            buf.set_string(
                table_layout[1].x,
                table_layout[1].y,
                "Description:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

            for (i, line) in HelpBox::CONTENT.iter().enumerate() {
                center_text(
                    buf,
                    Rect {
                        x: table_layout[0].x,
                        y: table_layout[1].y + i as u16 + 1,
                        width: table_layout[0].width,
                        height: 1,
                    },
                    line[0],
                    Style::default(),
                );

                buf.set_string(
                    table_layout[1].x,
                    table_layout[1].y + i as u16 + 1,
                    line[1],
                    Style::default(),
                );
            }
        }
    }
}
