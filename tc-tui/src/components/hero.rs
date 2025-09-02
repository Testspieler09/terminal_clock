use crate::components::logo::{CYAN_SHADES, GRAY_SHADES, Logo};
use ratatui::{
    layout::Flex,
    prelude::{Buffer, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

#[derive(Default, EnumProperty, EnumIter, PartialEq, Eq, Clone, Copy)]
pub(crate) enum MenuLabel {
    /// inactive
    /// ┌─┐┌─┐┌┬┐┌┬┐┬┌┐┌┌─┐┌─┐
    /// └─┐├┤  │  │ │││││ ┬└─┐
    /// └─┘└─┘ ┴  ┴ ┴┘└┘└─┘└─┘
    ///
    /// active
    /// ╔═╗╔═╗╔╦╗╔╦╗╦╔╗╔╔═╗╔═╗
    /// ╚═╗╠╣  ║  ║ ║║║║║ ╦╚═╗
    /// ╚═╝╚═╝ ╩  ╩ ╩╝╚╝╚═╝╚═╝
    #[default]
    #[strum(props(
        inactive = "┌─┐┌─┐┌┬┐┌┬┐┬┌┐┌┌─┐┌─┐\n└─┐├┤  │  │ │││││ ┬└─┐\n└─┘└─┘ ┴  ┴ ┴┘└┘└─┘└─┘",
        active = "╔═╗╔═╗╔╦╗╔╦╗╦╔╗╔╔═╗╔═╗\n╚═╗╠╣  ║  ║ ║║║║║ ╦╚═╗\n╚═╝╚═╝ ╩  ╩ ╩╝╚╝╚═╝╚═╝"
    ))]
    SETTINGS,

    /// inactive
    /// ┬ ┬┌─┐┬  ┌─┐
    /// ├─┤├┤ │  ├─┘
    /// ┴ ┴└─┘┴─┘┴
    ///
    /// active
    /// ╦ ╦╔═╗╦  ╔═╗
    /// ╠═╣╠╣ ║  ╠═╝
    /// ╩ ╩╚═╝╩═╝╩
    #[strum(props(
        inactive = "┬ ┬┌─┐┬  ┌─┐\n├─┤├┤ │  ├─┘\n┴ ┴└─┘┴─┘┴  ",
        active = "╦ ╦╔═╗╦  ╔═╗\n╠═╣╠╣ ║  ╠═╝\n╩ ╩╚═╝╩═╝╩  "
    ))]
    HELP,

    /// inactive
    /// ┌─┐ ┬ ┬ ┬┌┬┐
    /// │─┼┐│ │ │ │
    /// └─┘└└─┘ ┴ ┴
    ///
    /// active
    /// ╔═╗ ╦ ╦ ╦╔╦╗
    /// ║═╬╗║ ║ ║ ║
    /// ╚═╝╚╚═╝ ╩ ╩
    #[strum(props(
        inactive = "┌─┐ ┬ ┬ ┬┌┬┐\n│─┼┐│ │ │ │ \n└─┘└└─┘ ┴ ┴ ",
        active = "╔═╗ ╦ ╦ ╦╔╦╗\n║═╬╗║ ║ ║ ║ \n╚═╝╚╚═╝ ╩ ╩  "
    ))]
    QUIT,
}

#[derive(Default)]
pub(crate) struct Hero {
    pub active_label: MenuLabel,
    is_visible: bool,
}

impl Hero {
    fn map_label_to_ascii(label: &MenuLabel, active_label: &MenuLabel) -> Vec<Line<'static>> {
        let ascii = label
            .get_str(if *active_label == *label {
                "active"
            } else {
                "inactive"
            })
            .unwrap();

        ascii
            .lines()
            .zip(if label == active_label {
                CYAN_SHADES
            } else {
                GRAY_SHADES
            })
            .map(|(line, color)| Line::from(Span::from(line).style(Style::default().fg(color))))
            .collect()
    }

    pub fn next_label(&mut self) {
        self.active_label = MenuLabel::iter()
            .cycle()
            .skip_while(|label| *label != self.active_label)
            .skip(1)
            .next()
            .unwrap();
    }

    pub fn prev_label(&mut self) {
        let label = MenuLabel::iter().collect::<Vec<_>>();
        let current_position = label
            .iter()
            .position(|tab| *tab == self.active_label)
            .unwrap();

        let previous_position = if current_position == 0 {
            label.len() - 1
        } else {
            current_position - 1
        };

        self.active_label = label[previous_position];
    }

    pub fn set_visibility(&mut self, visibility: bool) {
        self.is_visible = visibility;
    }
}

impl Widget for &Hero {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_visible {
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

            let mut box_layout = Layout::horizontal([Constraint::Length(25)])
                .flex(Flex::Center)
                .split(label_section);

            box_layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(box_layout[0]);

            for (index, label) in MenuLabel::iter().enumerate() {
                let lines = Hero::map_label_to_ascii(&label, &self.active_label);
                for (i, line) in lines.iter().enumerate() {
                    let offset = (box_layout[index].width.saturating_sub(line.width() as u16)) / 2;
                    buf.set_line(
                        box_layout[index].x + offset,
                        box_layout[index].y + i as u16,
                        &line,
                        box_layout[index].width,
                    );
                }
            }
        }
    }
}
