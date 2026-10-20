use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};
use tc_models::color_theme::ThemeColor;

use crate::{
    assets,
    helpers::{centered_bold_label, centered_size_line},
    tui_models::{tui::TuiState, tui_traits::TcWidget},
};

pub(crate) struct FallbackView;

impl TcWidget<'_> for FallbackView {
    fn render(self, area: Rect, buf: &mut Buffer, state: &TuiState) {
        let theme = assets().get_color_theme(state.color_theme_idx);

        let fg_color = *theme.get(&ThemeColor::Foreground);
        let highlight_color = *theme.get(&ThemeColor::Accent);

        let [
            actual_label,
            actual_size_line,
            _,
            needed_label,
            needed_size_line,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .areas(area);

        centered_bold_label("Terminal size too small:", fg_color).render(actual_label, buf);
        centered_size_line(area.width, area.height, highlight_color).render(actual_size_line, buf);

        centered_bold_label("Currently needed space:", fg_color).render(needed_label, buf);
        centered_size_line(state.needed_width, state.needed_height, highlight_color)
            .render(needed_size_line, buf);
    }
}
