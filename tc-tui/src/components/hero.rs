use crate::components::logo::Logo;
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::Widget,
};

#[derive(Default)]
pub(crate) struct Hero {
    is_visible: bool,
}

impl Hero {
    pub fn set_visibility(&mut self, visibility: bool) {
        self.is_visible = visibility;
    }
}

impl Widget for &Hero {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            Logo.render(area, buf);
        }
    }
}
