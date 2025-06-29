use crate::clock::Clock;
use chrono::{Local, Timelike};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct ArtBlock<'a> {
    pub ascii_art: &'a str,
    pub led_coords: &'a Vec<Vec<(u32, u32)>>,
    pub time_value: u32,
}

// TODO: check the implementation
pub fn combine_ascii_art_while_applying_led<'a>(
    art1: &ArtBlock<'a>,
    art2: &ArtBlock<'a>,
    art3: &ArtBlock<'a>,
    accent_color: Color,
) -> (Paragraph<'a>, usize, usize) {
    let arts = [art1, art2, art3];

    // Split each ascii_art into lines
    let split_lines: Vec<Vec<&str>> = arts
        .iter()
        .map(|art| art.ascii_art.lines().collect())
        .collect();

    // Compute max width per block
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

    // Pad each line to align with the max width
    let padded_lines: Vec<Vec<String>> = split_lines
        .iter()
        .zip(&max_widths)
        .map(|(lines, width)| {
            lines
                .iter()
                .map(|line| format!("{:<width$}", line, width = width))
                .collect()
        })
        .collect();

    let paragraph_width = max_widths.iter().sum();
    let max_lines = padded_lines
        .iter()
        .map(|lines| lines.len())
        .max()
        .unwrap_or(0);

    let mut styled_text = Text::default();

    for line_index in 0..max_lines {
        let mut line_spans: Vec<Span> = Vec::new();

        for (art_idx, lines) in padded_lines.iter().enumerate() {
            let line = lines.get(line_index).map(String::as_str).unwrap_or("");
            let coords = &arts[art_idx].led_coords;
            let digit = arts[art_idx].time_value as usize;

            for (char_index, ch) in line.chars().enumerate() {
                let is_led = coords
                    .get(digit)
                    .is_some_and(|coords| coords.contains(&(line_index as u32, char_index as u32)));

                let style = if is_led {
                    Style::default().fg(accent_color)
                } else {
                    Style::default()
                };

                line_spans.push(Span::styled(ch.to_string(), style));
            }
        }

        styled_text.lines.push(Line::from(line_spans));
    }

    (
        Paragraph::new(styled_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Center),
        paragraph_width,
        max_lines,
    )
}

fn art_block<'a>(
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

pub struct ColorClock {
    // The static ascii art for the clock face
    hour: String,
    minutes: String,
    seconds: String,

    // The position of the characters that are suppsoed
    // to change color to display the time
    led_coords_hours: Vec<Vec<(u32, u32)>>,
    led_coords_minutes: Vec<Vec<(u32, u32)>>,
    led_coords_seconds: Vec<Vec<(u32, u32)>>,

    accent_color: Color,
}

impl ColorClock {
    pub fn new(
        hour: String,
        minutes: String,
        seconds: String,
        led_coords_hours: Vec<Vec<(u32, u32)>>,
        led_coords_minutes: Vec<Vec<(u32, u32)>>,
        led_coords_seconds: Vec<Vec<(u32, u32)>>,
        accent_color: Color,
    ) -> Self {
        ColorClock {
            hour,
            minutes,
            seconds,
            led_coords_hours,
            led_coords_minutes,
            led_coords_seconds,
            accent_color,
        }
    }
}

impl Clock for ColorClock {
    fn draw_clockface(&self, clock_format: &str) -> (Paragraph, usize, usize) {
        let time_stamp = Local::now();
        let hour_value = time_stamp.hour();
        let minute_value = time_stamp.minute();
        let second_value = time_stamp.second();

        static EMPTY_COORDS: &Vec<Vec<(u32, u32)>> = &Vec::new();
        let empty_block = art_block("", EMPTY_COORDS, 0);

        let result = match clock_format {
            "HH:MM:SS" => (
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                art_block(&self.seconds, &self.led_coords_seconds, second_value),
            ),
            "HH:MM" => (
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                empty_block,
            ),
            "MM:HH:SS" => (
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.seconds, &self.led_coords_seconds, second_value),
            ),
            _ => (
                art_block(&self.hour, &self.led_coords_hours, hour_value),
                art_block(&self.minutes, &self.led_coords_minutes, minute_value),
                empty_block,
            ),
        };

        combine_ascii_art_while_applying_led(&result.0, &result.1, &result.2, self.accent_color)
    }
}
