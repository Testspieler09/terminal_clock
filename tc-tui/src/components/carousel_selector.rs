use ratatui::{
    prelude::{Buffer, Rect},
    widgets::Widget,
};
use std::sync::Arc;

use crate::tui_models::TuiState;

pub(crate) struct CarouselSelector {
    // options: Vec<String>,
    // selected_option: String,
    tui_state: Arc<TuiState>,
}

impl CarouselSelector {
    fn next_option(&mut self) {}
    fn prev_option(&mut self) {}
}

impl Widget for &CarouselSelector {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
