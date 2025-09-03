use crate::components::logo::Logo;
use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Constraint, Layout, Widget},
};

use crate::tui_models::TuiState;

pub(crate) fn render_settings_view(frame: &mut Frame, config: &TuiState) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let logo = Logo::default();
    let logo_height = *logo.height() as u16;
    let logo_width = *logo.width() as u16;

    let box_height = 30;
    let box_width = 70;

    let [logo_section, setting_section] = Layout::vertical([
        Constraint::Length(logo_height),
        Constraint::Length(box_height),
    ])
    .margin((area.height - (logo_height + box_height)) / 2)
    .areas(area);

    let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
        .flex(Flex::Center)
        .areas(logo_section);

    let [settings_layout] = Layout::horizontal([Constraint::Length(box_width)])
        .flex(Flex::Center)
        .areas(setting_section);

    logo.render(logo_layout, buf);
    config.settings_menu.render(settings_layout, buf);
}
