use crate::{components::logo::Logo, helpers::generate_title};
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
    called_from_hero: bool,
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
            called_from_hero: false,
        }
    }
}

impl HelpBox {
    const CONTENT: [[&str; 2]; 3] = [
        ["?, h", "Show this help box"],
        ["q", "Quit this program"],
        ["ESC", "Toggles main menu"],
    ];

    pub fn set_visibility(&mut self, visibility: bool, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
        self.is_visible = visibility;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }
}

impl Widget for &HelpBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            let logo = Logo::default();
            let logo_height = *logo.height() as u16;
            let logo_width = *logo.width() as u16;

            let [top_box, bottom_box] = Layout::vertical([
                Constraint::Length(logo_height),
                Constraint::Length(self.height),
            ])
            .margin((area.height - (logo_height + self.height)) / 2)
            .areas(area);

            let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
                .flex(Flex::Center)
                .areas(top_box);

            let [box_layout] = Layout::horizontal([Constraint::Length(self.width)])
                .flex(Flex::Center)
                .areas(bottom_box);

            // Render the Logo
            logo.render(logo_layout, buf);

            let block = Block::bordered()
                .title(generate_title("help".to_string()))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));

            let inner_area = block.inner(box_layout);

            // Render the HelpBox
            block.render(box_layout, buf);

            let table_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(inner_area);

            fn center_text(buf: &mut Buffer, area: Rect, text: &str, style: Style) {
                let text_width = text.len() as u16;
                let offset = (area.width.saturating_sub(text_width)) / 2;
                buf.set_string(area.x + offset, area.y, text, style);
            }

            // Render table headers
            // TODO: use a paragraph here with alignement center
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
