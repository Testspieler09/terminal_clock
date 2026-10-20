use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Widget, WidgetRef},
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    components::fallback_terminal_too_small::FallbackView,
    helpers::unstable_widget_fits_frame,
    tui_models::{
        tui::TuiState,
        tui_constants::{CYAN_SHADES, FULL_LOGO, GRAY_SHADES},
        tui_traits::{Dimensions, TcWidget},
    },
};

pub(crate) struct Logo {
    height: u16,
    width: u16,
    logo_art: Text<'static>,
}

impl Logo {
    fn init() -> Text<'static> {
        let mut spans_vec = Vec::new();
        let mut color_index = 0;

        FULL_LOGO.lines().for_each(|line| {
            let mut line_spans = Vec::new();

            let line_cyan = CYAN_SHADES.get(color_index).copied().unwrap_or(Color::Cyan);
            let line_gray = GRAY_SHADES.get(color_index).copied().unwrap_or(Color::Gray);

            color_index += 1;

            // Process each character in the line
            let mut current_style = None;
            let mut current_text = String::new();

            for ch in line.chars() {
                let style = Style::default().fg(if ch == '█' {
                    line_cyan
                } else if matches!(ch, 'v' | '.' | '0'..='9') {
                    Color::Yellow
                } else {
                    line_gray
                });

                // If we encounter a new style or this is the first character
                if current_style != Some(style) {
                    // Push the accumulated text with its style
                    if !current_text.is_empty() {
                        line_spans.push(Span::styled(current_text, current_style.unwrap()));
                        current_text = String::new();
                    }
                    current_style = Some(style);
                }

                current_text.push(ch);
            }

            // Add the last span
            if let (false, Some(s)) = (current_text.is_empty(), current_style) {
                line_spans.push(Span::styled(current_text, s));
            }

            spans_vec.push(Line::from(line_spans));
        });

        Text::from(spans_vec)
    }

    fn render_component_with_logo_impl<'a, W, F>(
        &self,
        component: W,
        frame: &mut Frame,
        state: &'a TuiState,
        render_component: F,
    ) where
        W: Dimensions,
        F: FnOnce(W, Rect, &mut Buffer, &'a TuiState),
    {
        let area = frame.area();

        let component_height = component.height();
        let component_width = component.width();

        let logo_height = self.height;
        let logo_width = self.width;

        let (w, h) = (component_width + logo_width, component_height + logo_height);

        let needs_fallback = unstable_widget_fits_frame(frame, (w, h), area);

        let buf = frame.buffer_mut();

        if needs_fallback {
            FallbackView.render(area, buf, state);
            return;
        }

        let [logo_section, setting_section] = Layout::vertical([
            Constraint::Length(logo_height),
            Constraint::Length(component_height),
        ])
        .margin((area.height - (logo_height + component_height)) / 2)
        .areas(area);

        let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
            .flex(Flex::Center)
            .areas(logo_section);

        let [settings_layout] = Layout::horizontal([Constraint::Length(component_width)])
            .flex(Flex::Center)
            .areas(setting_section);

        self.render(logo_layout, buf);
        render_component(component, settings_layout, buf, state);
    }

    pub fn render_component_with_logo<W>(&self, component: W, frame: &mut Frame, state: &TuiState)
    where
        W: Widget + Dimensions,
    {
        self.render_component_with_logo_impl(
            component,
            frame,
            state,
            |component, area, buf, _state| {
                component.render(area, buf);
            },
        );
    }

    pub fn render_styled_component_with_logo<'a, W>(
        &self,
        component: W,
        frame: &mut Frame,
        state: &'a TuiState,
    ) where
        W: TcWidget<'a> + Dimensions,
    {
        self.render_component_with_logo_impl(
            component,
            frame,
            state,
            |component, area, buf, state| {
                component.render(area, buf, state);
            },
        );
    }
}

impl Default for Logo {
    fn default() -> Self {
        Logo {
            height: FULL_LOGO.lines().count() as u16 + 2,
            width: FULL_LOGO
                .lines()
                .map(|i| i.graphemes(true).count() + 2)
                .max()
                .unwrap() as u16,
            logo_art: Self::init(),
        }
    }
}

impl Dimensions for Logo {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }
}

impl Widget for &Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the static paragraph
        self.logo_art.render_ref(area, buf);
    }
}
