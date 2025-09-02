use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
};
use std::sync::Arc;
use tc_models::colorscheme::{ColorScheme, SchemeColor};

pub(crate) fn center_widget(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [mut centered_area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);

    [centered_area] = Layout::vertical([vertical])
        .flex(Flex::Center)
        .areas(centered_area);

    centered_area
}

pub(crate) fn center_widget_horizontally(
    area: Rect,
    horizontal: Constraint,
    vertical: Constraint,
    y: u16,
) -> Rect {
    let [mut centered_area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);

    [centered_area] = Layout::vertical([vertical]).areas(centered_area);

    centered_area.y = y;
    centered_area
}

pub(crate) fn generate_title(title: String, fg: Color) -> Line<'static> {
    Line::from(vec![
        Span::from("┐"),
        Span::from(title).style(Style::default().fg(fg)),
        Span::from("┌"),
    ])
}
