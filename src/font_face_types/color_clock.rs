use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use unicode_segmentation::UnicodeSegmentation;

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

    fn combine_ascii_art(artwork_1: &str, artwork_2: &str, artwork_3: &str) -> String {
        let art_1_lines: Vec<&str> = artwork_1.split('\n').collect();
        let art_2_lines: Vec<&str> = artwork_2.split('\n').collect();
        let art_3_lines: Vec<&str> = artwork_3.split('\n').collect();

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

        let mut combined_art = String::new();

        for i in 0..max_lines {
            let hour_line = padded_art_1_lines.get(i).map(String::as_str).unwrap_or("");
            let minute_line = padded_art_2_lines.get(i).map(String::as_str).unwrap_or(&"");
            let second_line = padded_art_3_lines.get(i).map(String::as_str).unwrap_or(&"");

            combined_art.push_str(hour_line);
            combined_art.push_str("");
            combined_art.push_str(minute_line);
            combined_art.push_str("");
            combined_art.push_str(second_line);
            combined_art.push('\n');
        }

        combined_art
    }

    fn draw_clockface(&self, f: &mut Frame, clock_format: &str, area: Rect) {
        let ascii_art = match clock_format {
            "HH:MM:SS" => Self::combine_ascii_art(Self::HOUR, Self::MINUTES, Self::SECONDS),
            "HH:MM" => Self::combine_ascii_art(Self::HOUR, Self::MINUTES, ""),
            "MM:HH:SS" => Self::combine_ascii_art(Self::MINUTES, Self::HOUR, Self::SECONDS),
            _ => Self::combine_ascii_art(Self::HOUR, Self::MINUTES, ""),
        };

        let paragraph = Paragraph::new(ascii_art).block(Block::default().borders(Borders::NONE));

        f.render_widget(paragraph, area);
    }
}
