use ratatui::widgets::Paragraph;

pub trait Clock {
    fn draw_clockface(&self, clock_format: &str) -> (Paragraph, usize, usize);
}
