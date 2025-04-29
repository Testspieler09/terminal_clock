use crate::helpers::center_widget;
use chrono::{Local, Timelike};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use unicode_segmentation::UnicodeSegmentation;

fn combine_ascii_art_while_applying_led<'a>(
    artwork_1_triple: (&'a str, &[&[(u32, u32)]], u32),
    artwork_2_triple: (&'a str, &[&[(u32, u32)]], u32),
    artwork_3_triple: (&'a str, &[&[(u32, u32)]], u32),
) -> (Paragraph<'a>, usize, usize) {
    let art_1_lines: Vec<&str> = artwork_1_triple.0.split('\n').collect();
    let art_2_lines: Vec<&str> = artwork_2_triple.0.split('\n').collect();
    let art_3_lines: Vec<&str> = artwork_3_triple.0.split('\n').collect();

    let max_art_1_width = art_1_lines
        .iter()
        .map(|line| line.graphemes(true).count())
        .max()
        .unwrap_or(0);
    let max_art_2_width = art_2_lines
        .iter()
        .map(|line| line.graphemes(true).count())
        .max()
        .unwrap_or(0);
    let max_art_3_width = art_3_lines
        .iter()
        .map(|line| line.graphemes(true).count())
        .max()
        .unwrap_or(0);

    let paragraph_width = max_art_1_width + max_art_2_width + max_art_3_width;

    let padded_art_1_lines: Vec<String> = art_1_lines
        .into_iter()
        .map(|line| format!("{:<width$}", line, width = max_art_1_width))
        .collect();
    let padded_art_2_lines: Vec<String> = art_2_lines
        .into_iter()
        .map(|line| format!("{:<width$}", line, width = max_art_2_width))
        .collect();
    let padded_art_3_lines: Vec<String> = art_3_lines
        .into_iter()
        .map(|line| format!("{:<width$}", line, width = max_art_3_width))
        .collect();

    let max_lines = padded_art_1_lines
        .len()
        .max(padded_art_2_lines.len())
        .max(padded_art_3_lines.len());

    let mut styled_text = Text::default();

    for i in 0..max_lines {
        let art_1_line = padded_art_1_lines.get(i).map(String::as_str).unwrap_or("");
        let art_2_line = padded_art_2_lines.get(i).map(String::as_str).unwrap_or("");
        let art_3_line = padded_art_3_lines.get(i).map(String::as_str).unwrap_or("");

        let mut line_spans: Vec<Span> = Vec::new();

        for (line_index, &line) in [art_1_line, art_2_line, art_3_line].iter().enumerate() {
            for (char_index, ch) in line.chars().enumerate() {
                let is_led_position = match line_index % 3 {
                    0 => artwork_1_triple.1[artwork_1_triple.2 as usize]
                        .contains(&(i as u32, char_index as u32)),
                    1 => artwork_2_triple.1[artwork_2_triple.2 as usize]
                        .contains(&(i as u32, char_index as u32)),
                    2 => artwork_3_triple.1[artwork_3_triple.2 as usize]
                        .contains(&(i as u32, char_index as u32)),
                    _ => false,
                };

                let style = if is_led_position {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };

                line_spans.push(Span::styled(ch.to_string(), style));
            }
        }
        styled_text.extend(vec![Line::from(line_spans)]);
    }

    (
        Paragraph::new(styled_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Center),
        paragraph_width,
        max_lines,
    )
}

pub trait ColorClock {
    // The static ascii art for the clock face
    const HOUR: &'static str;
    const MINUTES: &'static str;
    const SECONDS: &'static str;

    // The position of the characters that are suppsoed
    // to change color to display the time
    const LED_COORDS_HOURS: &[&[(u32, u32)]];
    const LED_COORDS_MINUTES: &[&[(u32, u32)]];
    const LED_COORDS_SECONDS: &[&[(u32, u32)]];

    fn draw_clockface(&self, f: &mut Frame, clock_format: &str, area: Rect) {
        let time_stamp = Local::now();
        let hour_value = time_stamp.hour();
        let minute_value = time_stamp.minute();
        let second_value = time_stamp.second();

        let (ascii_art_paragraph, paragraph_width, paragraph_height): (
            Paragraph<'_>,
            usize,
            usize,
        ) = match clock_format {
            "HH:MM:SS" => combine_ascii_art_while_applying_led(
                (Self::HOUR, Self::LED_COORDS_HOURS, hour_value),
                (Self::MINUTES, Self::LED_COORDS_MINUTES, minute_value),
                (Self::SECONDS, Self::LED_COORDS_SECONDS, second_value),
            ),
            "HH:MM" => combine_ascii_art_while_applying_led(
                (Self::HOUR, Self::LED_COORDS_HOURS, hour_value),
                (Self::MINUTES, Self::LED_COORDS_MINUTES, minute_value),
                ("", &[], second_value),
            ),
            "MM:HH:SS" => combine_ascii_art_while_applying_led(
                (Self::MINUTES, Self::LED_COORDS_MINUTES, minute_value),
                (Self::HOUR, Self::LED_COORDS_HOURS, hour_value),
                (Self::SECONDS, Self::LED_COORDS_SECONDS, second_value),
            ),
            _ => combine_ascii_art_while_applying_led(
                (Self::HOUR, Self::LED_COORDS_HOURS, hour_value),
                (Self::MINUTES, Self::LED_COORDS_MINUTES, minute_value),
                ("", &[], second_value),
            ),
        };

        let area = center_widget(
            area,
            Constraint::Length(paragraph_width as u16),
            Constraint::Length(paragraph_height as u16),
        );

        f.render_widget(ascii_art_paragraph, area);
    }
}
