use crate::{components::Dimensions, helpers::generate_title, tui_models::TuiController};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Widget},
};
use std::sync::Arc;
use tc_models::colorscheme::SchemeColor;

// FIX: this will never change -> static should not be computed every time it is rendered
pub(crate) struct HelpBox {
    height: u16,
    width: u16,
    called_from_hero: bool,
    tui_controller: Arc<TuiController>,
}

impl HelpBox {
    const CONTENT: [[&str; 2]; 3] = [
        ["?, h", "Show this help box"],
        ["q", "Quit this program"],
        ["ESC", "Toggles main menu"],
    ];

    pub fn new(tui_controller: Arc<TuiController>) -> Self {
        HelpBox {
            height: HelpBox::CONTENT.len() as u16 + 3, // Border (2) + Tableheader (1)
            width: HelpBox::CONTENT
                .iter()
                .map(|pair| pair[0].len() + pair[1].len() + 15)
                .max()
                .unwrap() as u16,
            called_from_hero: false,
            tui_controller,
        }
    }

    pub fn set_called_from_hero(&mut self, was_called_from_hero: bool) {
        self.called_from_hero = was_called_from_hero;
    }

    pub fn was_called_from_hero(&self) -> bool {
        self.called_from_hero
    }
}

impl Dimensions for &HelpBox {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }
}

impl Widget for &HelpBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Color Settings for this widget
        let fg_color = self.tui_controller.get_color(&SchemeColor::Foreground);
        let border_color = self.tui_controller.get_color(&SchemeColor::Borders);
        let highlight_color = self.tui_controller.get_color(&SchemeColor::Accent);

        let block = Block::bordered()
            .title(generate_title("help".to_string(), fg_color))
            .style(Style::default().fg(fg_color))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let inner_area = block.inner(area);

        // Render the HelpBox
        block.render(area, buf);

        let table_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
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
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        );

        buf.set_string(
            table_layout[1].x,
            table_layout[1].y,
            "Description:",
            Style::default()
                .fg(highlight_color)
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
                Style::default().fg(fg_color),
            );

            buf.set_string(
                table_layout[1].x,
                table_layout[1].y + i as u16 + 1,
                line[1],
                Style::default().fg(fg_color),
            );
        }
    }
}
