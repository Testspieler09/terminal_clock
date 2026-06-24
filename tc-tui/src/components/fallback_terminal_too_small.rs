use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, ThemeColor};

use crate::{
    helpers::{centered_bold_label, centered_size_line},
    tui_models::styled_widget::StyledWidget,
};

pub(crate) struct FallbackContext<'a> {
    color_theme: &'a ColorTheme,
    needed_width: u16,
    needed_height: u16,
}

impl<'a> FallbackContext<'a> {
    pub(crate) fn new(color_theme: &'a ColorTheme, needed_width: u16, needed_height: u16) -> Self {
        Self {
            color_theme,
            needed_width,
            needed_height,
        }
    }
}

pub(crate) struct FallbackView;

impl StyledWidget for &mut FallbackView {
    type Context<'a> = FallbackContext<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, ctx: Self::Context<'_>) {
        let fg_color = *ctx.color_theme.get(&ThemeColor::Foreground);
        let highlight_color = *ctx.color_theme.get(&ThemeColor::Accent);

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
        centered_size_line(ctx.needed_width, ctx.needed_height, highlight_color)
            .render(needed_size_line, buf);
    }
}
