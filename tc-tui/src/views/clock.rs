use crate::{
    TuiState,
    components::{Dimensions, fallback_terminal_too_small::FallbackView},
    helpers::{
        center_widget, center_widget_horizontally, unstable_widget_fits_frame, widget_fits_frame,
    },
    tui_models::{styled_widget::StyledWidget, tui::TuiAssets},
};
use ratatui::{Frame, prelude::Constraint};
use tc_models::clock::ClockBehaviour;

pub(crate) fn render_clock_view(
    frame: &mut Frame,
    config: &TuiState,
    tui_assets: &TuiAssets,
    fallback_view: &mut FallbackView,
) {
    let frame_area = frame.area();

    let clock = tui_assets.get_clock(config.clock_state.clock_face_idx);
    let color_theme = tui_assets.get_color_theme(config.color_theme_idx);

    let (clock_widget, clock_w, clock_h) =
        clock.generate_clock_face_with_dimensions(color_theme, config.clock_state.clock_time_fmt);
    let clock_area = center_widget(
        frame_area,
        Constraint::Length(clock_w as u16),
        Constraint::Length(clock_h as u16),
    );
    let clock_overflows =
        unstable_widget_fits_frame(frame, (clock_w as u16, clock_h as u16), clock_area);

    let quote = tui_assets.get_quote(config.quote_idx);

    let quote_info = quote.as_ref().map(|quote| {
        let qw = quote.width();
        let qh = quote.height();

        let quote_area = center_widget_horizontally(
            frame_area,
            Constraint::Length(qw),
            Constraint::Length(qh),
            clock_area.y + clock_area.height + 1,
        );

        let overflows = widget_fits_frame(frame, *quote, quote_area);

        (quote, quote_area, overflows)
    });

    let needs_fallback =
        clock_overflows || quote_info.as_ref().map(|(_, _, o)| *o).unwrap_or(false);
    if needs_fallback {
        let (w, h) = if let Some((quote, _, true)) = &quote_info {
            (quote.width(), quote.height())
        } else {
            (clock_w as u16, clock_h as u16)
        };

        fallback_view.update_dimensions(Some(w), Some(h));
        fallback_view.render(frame_area, frame.buffer_mut(), color_theme);
        return;
    }

    frame.render_widget(clock_widget, clock_area);
    if let Some((quote, quote_area, _)) = quote_info {
        quote.render(quote_area, frame.buffer_mut(), color_theme);
    }
}
