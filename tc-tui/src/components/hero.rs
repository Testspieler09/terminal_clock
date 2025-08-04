use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

const LOGO: &str = "████████╗ ██████╗
╚══██╔══╝██╔════╝
   ██║   ██║
   ██║   ██║
   ██║   ╚██████╗
   ╚═╝    ╚═════╝ v0.0.0";

#[derive(Default, Clone)]
pub(crate) struct Hero {
    visible: bool,
}

impl Hero {
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Widget for Hero {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.visible {
            let block = Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL);

            let inner_area = block.inner(area);

            let cyan_shades = [
                Color::Rgb(0x44, 0xAE, 0xB3),
                Color::Rgb(0x24, 0x9E, 0xA0),
                Color::Rgb(0x00, 0x8B, 0x8B),
                Color::Rgb(0x00, 0x79, 0x79),
                Color::Rgb(0x00, 0x67, 0x67),
            ];

            let mut color_index = 0;

            for (i, line) in LOGO.lines().enumerate() {
                let y = inner_area.y + i as u16;
                let mut x = inner_area.x;

                let has_block = line.contains('█');
                let block_color = if has_block {
                    let c = cyan_shades.get(color_index).copied().unwrap_or(Color::Cyan);
                    color_index += 1;
                    c
                } else {
                    Color::Gray
                };

                let mut buf_str = [0u8; 4];
                for ch in line.chars() {
                    let style = Style::default().fg(if ch == '█' && has_block {
                        block_color
                    } else {
                        Color::Gray
                    });

                    buf.cell_mut((x, y))
                        .expect("")
                        .set_symbol(ch.encode_utf8(&mut buf_str))
                        .set_style(style);
                    x += 1;
                }
            }
        }
    }
}

// impl Widget for Hero {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         if self.visible {
//             let block = Block::default()
//                 .border_type(BorderType::Rounded)
//                 .borders(Borders::ALL);
//
//             let inner_area = block.inner(area);
//
//             for (i, line) in LOGO.split("\n").enumerate() {
//                 buf.set_string(
//                     inner_area.x,
//                     inner_area.y + i as u16,
//                     line,
//                     Style::default().fg(Color::Green),
//                 );
//             }
//         }
//     }
// }
