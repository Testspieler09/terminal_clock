use ratatui::{
    prelude::{Buffer, Rect, Widget},
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};
use tc_models::{color_theme::ThemeColor, quote::Quote};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    TuiState, assets,
    tui_models::tui_traits::{Dimensions, TcWidget},
};

impl Dimensions for &Quote {
    fn width(&self) -> u16 {
        let final_text = self.final_quote_string();

        final_text.graphemes(true).count() as u16
    }

    fn height(&self) -> u16 {
        1u16
    }
}

impl TcWidget<'_> for &Quote {
    fn render(self, area: Rect, buf: &mut Buffer, state: &TuiState) {
        let theme = assets().get_color_theme(state.color_theme_idx);
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
                    .fg(*theme.get(&ThemeColor::Accent))
                    .add_modifier(Modifier::BOLD),
            ))
            .render(area, buf);
        }
    }
}
