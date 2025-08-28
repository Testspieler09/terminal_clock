use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) struct Logo {
    height: usize,
    width: usize,
}

/// Mainly used for Logo and Hero
pub(super) const CYAN_SHADES: [Color; 6] = [
    Color::Rgb(0x44, 0xAE, 0xB3),
    Color::Rgb(0x24, 0x9E, 0xA0),
    Color::Rgb(0x00, 0x8B, 0x8B),
    Color::Rgb(0x00, 0x79, 0x79),
    Color::Rgb(0x00, 0x67, 0x67),
    Color::Rgb(0x00, 0x55, 0x55),
];

/// Mainly used for Logo and Hero
pub(super) const GRAY_SHADES: [Color; 6] = [
    Color::Rgb(0xBB, 0xBB, 0xBB),
    Color::Rgb(0xAA, 0xAA, 0xAA),
    Color::Rgb(0x99, 0x99, 0x99),
    Color::Rgb(0x88, 0x88, 0x88),
    Color::Rgb(0x77, 0x77, 0x77),
    Color::Rgb(0x66, 0x66, 0x66),
];

impl Default for Logo {
    fn default() -> Self {
        Logo {
            height: Logo::FULL_LOGO.lines().count() + 2,
            width: Logo::FULL_LOGO
                .lines()
                .map(|i| i.graphemes(true).count() + 2)
                .max()
                .unwrap_or(0),
        }
    }
}

impl Logo {
    const FULL_LOGO: &str = concat!(
        "████████╗ ██████╗\n",
        "╚══██╔══╝██╔════╝\n",
        "   ██║   ██║\n",
        "   ██║   ██║\n",
        "   ██║   ╚██████╗\n",
        "   ╚═╝    ╚═════╝ v",
        env!("CARGO_PKG_VERSION")
    );

    pub fn height(&self) -> &usize {
        &self.height
    }

    pub fn width(&self) -> &usize {
        &self.width
    }
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let inner_area = block.inner(area);

        let mut color_index = 0;

        for (i, line) in Logo::FULL_LOGO.lines().enumerate() {
            let y = inner_area.y + i as u16;
            let mut x = inner_area.x;

            let line_cyan = CYAN_SHADES.get(color_index).copied().unwrap_or(Color::Cyan);
            let line_gray = GRAY_SHADES.get(color_index).copied().unwrap_or(Color::Gray);

            color_index += 1;

            let mut buf_str = [0u8; 4];
            for ch in line.chars() {
                let style = Style::default().fg(if ch == '█' {
                    line_cyan
                } else if matches!(ch, 'v' | '.' | '0'..'9') {
                    Color::Yellow
                } else {
                    line_gray
                });

                // FIX: expect should be replaced with error handling
                buf.cell_mut((x, y))
                    .expect("")
                    .set_symbol(ch.encode_utf8(&mut buf_str))
                    .set_style(style);
                x += 1;
            }
        }
    }
}
