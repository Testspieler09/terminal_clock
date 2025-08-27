use crate::components::logo::Logo;
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::Widget,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, Display)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "general")]
    Tab1,
    #[strum(to_string = "pomodoro")]
    Tab2,
    #[strum(to_string = "color")]
    Tab3,
}

#[derive(Default)]
pub struct SettingMenu {
    current_tab: SelectedTab,
    is_visible: bool,
}

impl SettingMenu {
    pub fn set_visibility(&mut self, visibility: bool) {
        self.is_visible = visibility;
    }
}

impl Widget for &SettingMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
            let logo = Logo::default();
            logo.render(area, buf);
            buf.set_string(0, 7, "Settings Menu", Style::default());
        }
    }
}
