use ratatui::{
    prelude::{Buffer, Rect, Widget},
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};
use tc_models::{
    color_theme::{ColorTheme, ThemeColor},
    quote::Quote,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{components::Dimensions, tui_models::styled_widget::StyledWidget};

impl Dimensions for &Quote {
    fn width(&self) -> u16 {
        let final_text = self.final_quote_string();

        final_text.graphemes(true).count() as u16
    }

    fn height(&self) -> u16 {
        1u16
    }
}

impl StyledWidget for &Quote {
    type Context<'a> = &'a ColorTheme;

    fn render(self, area: Rect, buf: &mut Buffer, color_theme: Self::Context<'_>) {
        let final_text = self.final_quote_string();

        if let Some(color) = self.accent_color {
            Paragraph::new(Span::styled(
                final_text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
            .render(area, buf);
        } else {
            Paragraph::new(Span::styled(
                final_text,
                Style::default()
                    .fg(*color_theme.get(&ThemeColor::Accent))
                    .add_modifier(Modifier::BOLD),
            ))
            .render(area, buf);
        }
    }
}
