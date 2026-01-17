use ratatui::{buffer::Buffer, layout::Rect};
use tc_models::color_theme::ColorTheme;

pub(crate) trait StyledWidget {
    fn render(self, area: Rect, buf: &mut Buffer, color_theme: &ColorTheme);
}
