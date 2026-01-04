use crate::helpers::{centered_bold_label, centered_size_line};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, ThemeColor};

pub(crate) struct FallbackView {
    needed_width: u16,
    needed_height: u16,
    color_theme: ColorTheme,
}

impl FallbackView {
    pub fn new(needed_width: u16, needed_height: u16, color_theme: ColorTheme) -> Self {
        FallbackView {
            needed_width,
            needed_height,
            color_theme,
        }
    }

    pub fn update_dimensions(&mut self, new_width: Option<u16>, new_height: Option<u16>) {
        if let Some(width) = new_width {
            self.needed_width = width;
        }
        if let Some(height) = new_height {
            self.needed_height = height;
        }
    }
}

impl Widget for &mut FallbackView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let fg_color = *self.color_theme.get(&ThemeColor::Foreground);
        let highlight_color = *self.color_theme.get(&ThemeColor::Accent);

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
        centered_size_line(self.needed_width, self.needed_height, highlight_color)
            .render(needed_size_line, buf);
        // Paragraph::new("Terminal size too small:")
        //     .style(Style::default().fg(fg_color))
        //     .add_modifier(Modifier::BOLD)
        //     .alignment(Alignment::Center)
        //     .render(actual_label, buf);
        // Paragraph::new(Line::from(vec![
        //     Span::from("Width = "),
        //     Span::styled(format!("{}", area.width), highlight_style),
        //     Span::from("  Height = "),
        //     Span::styled(format!("{}", area.height), highlight_style),
        // ]))
        // .add_modifier(Modifier::BOLD)
        // .alignment(Alignment::Center)
        // .render(actual_size_line, buf);
        //
        // Paragraph::new("Currently needed space:")
        //     .style(Style::default().fg(fg_color))
        //     .add_modifier(Modifier::BOLD)
        //     .alignment(Alignment::Center)
        //     .render(needed_label, buf);
        // Paragraph::new(Line::from(vec![
        //     Span::from("Width = "),
        //     Span::styled(format!("{}", self.needed_width), highlight_style),
        //     Span::from("  Height = "),
        //     Span::styled(format!("{}", self.needed_height), highlight_style),
        // ]))
        // .add_modifier(Modifier::BOLD)
        // .alignment(Alignment::Center)
        // .render(needed_size_line, buf);
    }
}
