use crate::components::{CYAN_SHADES, Dimensions, GRAY_SHADES};
use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Widget, WidgetRef},
};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) struct Logo {
    height: u16,
    width: u16,
    logo_art: Text<'static>,
}

impl Logo {
    const FULL_LOGO: &str = concat!(
        "████████╗ ██████╗\n",
        "╚══██╔══╝██╔════╝\n",
        "   ██║   ██║\n",
        "   ██║   ██║\n",
        "   ██║   ╚██████╗\n",
        "   ╚═╝    ╚═════╝ v",
        env!("CARGO_PKG_VERSION")
    );

    fn init() -> Text<'static> {
        let mut spans_vec = Vec::new();
        let mut color_index = 0;

        Logo::FULL_LOGO.lines().for_each(|line| {
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
                } else if matches!(ch, 'v' | '.' | '0'..'9') {
                    Color::Yellow
                } else {
                    line_gray
                });

                // If we encounter a new style or this is the first character
                if current_style.map_or(true, |s| s != style) {
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
            if !current_text.is_empty() && current_style.is_some() {
                line_spans.push(Span::styled(current_text, current_style.unwrap()));
            }

            spans_vec.push(Line::from(line_spans));
        });

        Text::from(spans_vec)
    }

    pub fn render_component_with_logo<W: Widget + Dimensions>(
        &self,
        component: W,
        frame: &mut Frame,
    ) {
        // TODO: add the fallback logic here as well
        let area = frame.area();
        let buf = frame.buffer_mut();

        let component_height = component.height();
        let component_width = component.width();

        let logo_height = self.height;
        let logo_width = self.width;

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
        component.render(settings_layout, buf);
    }
}

impl Default for Logo {
    fn default() -> Self {
        Logo {
            height: Logo::FULL_LOGO.lines().count() as u16 + 2,
            width: Logo::FULL_LOGO
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
