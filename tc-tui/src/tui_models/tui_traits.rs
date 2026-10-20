use ratatui::{buffer::Buffer, layout::Rect};

use crate::tui_models::tui::TuiState;

pub(crate) trait TcWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer, state: &'a TuiState);
}

pub(crate) trait Dimensions {
    fn height(&self) -> u16;
    fn width(&self) -> u16;
}
