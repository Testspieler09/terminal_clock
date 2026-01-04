use crate::{
    TuiState,
    components::{Dimensions, fallback_terminal_too_small::FallbackView},
    helpers::{
        center_widget, center_widget_horizontally, unstable_widget_fits_frame, widget_fits_frame,
    },
};
use ratatui::{Frame, prelude::Constraint};
use tc_models::clock::ClockBehaviour;

pub(crate) fn render_clock_view(
    frame: &mut Frame,
    config: &TuiState,
    fallback_view: &mut FallbackView,
) {
    let frame_area = frame.area();

    let clock_lock = config.clock_face.lock().unwrap();
    let color_theme_lock = config.color_theme.lock().unwrap();

    let (clock_widget, clock_w, clock_h) =
        clock_lock.generate_clock_face_with_dimensions(&color_theme_lock);
    let clock_area = center_widget(
        frame_area,
        Constraint::Length(clock_w as u16),
        Constraint::Length(clock_h as u16),
    );
    let clock_overflows =
        unstable_widget_fits_frame(frame, (clock_w as u16, clock_h as u16), clock_area);

    let quote_info = config.quote.as_ref().map(|quote| {
        let qw = quote.as_ref().width();
        let qh = quote.as_ref().height();

        let quote_area = center_widget_horizontally(
            frame_area,
            Constraint::Length(qw),
            Constraint::Length(qh),
            clock_area.y + clock_area.height + 1,
        );

        let overflows = widget_fits_frame(frame, quote.as_ref(), quote_area);

        (quote, quote_area, overflows)
    });

    let needs_fallback =
        clock_overflows || quote_info.as_ref().map(|(_, _, o)| *o).unwrap_or(false);
    if needs_fallback {
        let (w, h) = if let Some((quote, _, true)) = &quote_info {
            (quote.as_ref().width(), quote.as_ref().height())
        } else {
            (clock_w as u16, clock_h as u16)
        };

        fallback_view.update_dimensions(Some(w), Some(h));
        frame.render_widget(fallback_view, clock_area);
        return;
    }

    frame.render_widget(clock_widget, clock_area);
    if let Some((quote, quote_area, _)) = quote_info {
        frame.render_widget(quote.as_ref(), quote_area);
    }
}
