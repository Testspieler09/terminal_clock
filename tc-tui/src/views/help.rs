use crate::{components::logo::Logo, tui_models::TuiState};
use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Constraint, Layout, Widget},
};

pub(crate) fn render_help_view(frame: &mut Frame, config: &TuiState) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let logo = Logo::default();
    let logo_height = *logo.height() as u16;
    let logo_width = *logo.width() as u16;

    let [top_box, bottom_box] = Layout::vertical([
        Constraint::Length(logo_height),
        Constraint::Length(config.help_box.height),
    ])
    .margin((area.height - (logo_height + config.help_box.height)) / 2)
    .areas(area);

    let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
        .flex(Flex::Center)
        .areas(top_box);

    let [help_layout] = Layout::horizontal([Constraint::Length(config.help_box.width)])
        .flex(Flex::Center)
        .areas(bottom_box);

    // Render the Logo
    logo.render(logo_layout, buf);
    config.help_box.render(help_layout, buf);
}
