use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::{Alignment, Stylize},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{components::Dimensions, tui_models::styled_widget::StyledWidget};

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

pub(crate) fn centered_bold_label<'a>(text: &'a str, color: Color) -> Paragraph<'a> {
    Paragraph::new(text)
        .style(Style::default().fg(color))
        .add_modifier(Modifier::BOLD)
        .alignment(Alignment::Center)
}

pub(crate) fn centered_size_line(width: u16, height: u16, highlight: Color) -> Paragraph<'static> {
    Paragraph::new(Line::from(vec![
        Span::raw("Width = "),
        Span::styled(width.to_string(), Style::default().fg(highlight)),
        Span::raw("  Height = "),
        Span::styled(height.to_string(), Style::default().fg(highlight)),
    ]))
    .add_modifier(Modifier::BOLD)
    .alignment(Alignment::Center)
}

pub(crate) fn widget_fits_frame<W: StyledWidget + Dimensions>(
    frame: &Frame,
    widget: W,
    area: Rect,
) -> bool {
    let frame_area = frame.area();

    let widget_does_not_fit_area = widget.width() > area.width || widget.height() > area.height;

    widget_does_not_fit_area
        || area.x + area.width > frame_area.width
        || area.y + area.height > frame_area.height
}

/// This helper is marked unstable, as width and height are not derived from the widget but passed
/// explizitly.
pub(crate) fn unstable_widget_fits_frame(
    frame: &Frame,
    (width, height): (u16, u16),
    area: Rect,
) -> bool {
    let frame_area = frame.area();

    let widget_does_not_fit_area = width > area.width || height > area.height;

    widget_does_not_fit_area
        || area.x + area.width > frame_area.width
        || area.y + area.height > frame_area.height
}
