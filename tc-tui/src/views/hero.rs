use crate::{components::logo::Logo, tui_models::TuiState};
use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Constraint, Layout, Widget},
};

pub(crate) fn render_hero_view(frame: &mut Frame, config: &TuiState) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let logo = Logo::default();
    let logo_height = *logo.height() as u16;
    let logo_width = *logo.width() as u16;

    let label_height = 12; // MenuLabel::COUNT * 3 + 3

    let [logo_section, label_section] = Layout::vertical(
        [
            Constraint::Length(logo_height),
            Constraint::Length(label_height),
        ]
        .as_ref(),
    )
    .margin((area.height - (logo_height + label_height)) / 2)
    .areas(area);

    let [logo_layout] = Layout::horizontal([Constraint::Length(logo_width)])
        .flex(Flex::Center)
        .areas(logo_section);

    logo.render(logo_layout, buf);

    let [hero_layout] = Layout::horizontal([Constraint::Length(25)])
        .flex(Flex::Center)
        .areas(label_section);

    config.hero.render(hero_layout, buf);
}
