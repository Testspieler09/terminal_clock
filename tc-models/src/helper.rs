use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) struct ArtBlock<'a> {
    pub ascii_art: &'a str,
    pub led_coords: &'a Vec<Vec<(u32, u32)>>,
    pub time_value: u32,
}

pub(crate) fn art_block<'a>(
    ascii_art: &'a str,
    led_coords: &'a Vec<Vec<(u32, u32)>>,
    time_value_p: u32,
) -> ArtBlock<'a> {
    ArtBlock {
        ascii_art,
        led_coords,
        time_value: time_value_p,
    }
}

pub(crate) fn combine_ascii_art_while_applying_led<'a>(
    art1: &ArtBlock<'a>,
    art2: &ArtBlock<'a>,
    art3: &ArtBlock<'a>,
    accent_color: Color,
) -> (Paragraph<'a>, usize, usize) {
    let arts = [art1, art2, art3];

    let split_lines: Vec<Vec<&str>> = arts
        .iter()
        .map(|art| art.ascii_art.lines().collect())
        .collect();

    let max_widths: Vec<usize> = split_lines
        .iter()
        .map(|lines| {
            lines
                .iter()
                .map(|l| l.graphemes(true).count())
                .max()
                .unwrap_or(0)
        })
        .collect();

    let max_lines = split_lines
        .iter()
        .map(|lines| lines.len())
        .max()
        .unwrap_or(0);
    let paragraph_width = max_widths.iter().sum();

    let mut styled_text = Text::default();
    let mut line_spans = Vec::with_capacity(paragraph_width);

    for line_index in 0..max_lines {
        line_spans.clear();

        for (art_idx, lines) in split_lines.iter().enumerate() {
            let width = max_widths[art_idx];
            let line = lines.get(line_index).copied().unwrap_or("");
            let graphemes: Vec<&str> = line.graphemes(true).collect();
            let padding = width.saturating_sub(graphemes.len());

            let coords = &arts[art_idx].led_coords;
            let digit = arts[art_idx].time_value as usize;
            let active = coords.get(digit);

            for (char_index, grapheme) in graphemes.iter().enumerate() {
                let is_led = active
                    .map(|set| set.contains(&(line_index as u32, char_index as u32)))
                    .unwrap_or(false);

                let style = if is_led {
                    Style::default().fg(accent_color)
                } else {
                    Style::default()
                };

                line_spans.push(Span::styled(*grapheme, style));
            }

            // Add trailing padding if needed
            for _ in 0..padding {
                line_spans.push(Span::raw(" "));
            }
        }

        styled_text.lines.push(Line::from(line_spans.clone()));
    }

    (
        Paragraph::new(styled_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Center),
        paragraph_width,
        max_lines,
    )
}

// Helpers for external and internal use
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}

pub fn generate_binary_led_coords(
    tens_bits: &[(u8, (u32, u32))],
    units_bits: &[(u8, (u32, u32))],
    always_on: &[(u32, u32)],
    unit: TimeUnit,
) -> Vec<Vec<(u32, u32)>> {
    let max_value = match unit {
        TimeUnit::Minutes | TimeUnit::Seconds => 60,
        TimeUnit::Hours => 24,
    };

    let mut result = Vec::with_capacity(max_value);

    for value in 0..max_value {
        let tens = value / 10;
        let units = value % 10;

        let mut coords = Vec::with_capacity(8);
        coords.extend_from_slice(always_on);

        // Apply tens digit bits
        for &(bit, coord) in tens_bits {
            if tens & bit as usize != 0 {
                coords.push(coord);
            }
        }

        // Apply units digit bits
        for &(bit, coord) in units_bits {
            if units & bit as usize != 0 {
                coords.push(coord);
            }
        }

        result.push(coords);
    }

    result
}
