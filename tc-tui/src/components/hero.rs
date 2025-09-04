use crate::components::{CYAN_SHADES, Dimensions, GRAY_SHADES};
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

pub(crate) struct Hero {
    pub active_label: MenuLabel,
    height: u16,
    width: u16,
}

impl Hero {
    fn map_label_to_ascii(label: &MenuLabel, active_label: &MenuLabel) -> Vec<Line<'static>> {
        let ascii = label
            .get_str(if *active_label == *label {
                "active"
            } else {
                "inactive"
            })
            .expect(
                "All MenuLabel enum variants should have an active and inactive representation",
            );

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
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            active_label: MenuLabel::default(),
            height: {
                // Each MenuLabel has 3 lines, and we display them vertically
                MenuLabel::iter().count() as u16 * 3
            },
            width: {
                MenuLabel::iter()
                    .map(|label| {
                        label
                            .get_str("active")
                            .expect("All labels should have a string representation")
                            .lines()
                            .filter_map(|line| Some(line.len()))
                            .max()
                            .expect("The labels string representation should not be empty")
                    })
                    .max()
                    .unwrap() as u16
            },
        }
    }
}

impl Dimensions for &Hero {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }
}

impl Widget for &Hero {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut label_layout = Layout::horizontal([Constraint::Length(25)])
            .flex(Flex::Center)
            .split(area);

        label_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(label_layout[0]);

        for (index, label) in MenuLabel::iter().enumerate() {
            let lines = Hero::map_label_to_ascii(&label, &self.active_label);
            for (i, line) in lines.iter().enumerate() {
                let offset = (label_layout[index]
                    .width
                    .saturating_sub(line.width() as u16))
                    / 2;
                buf.set_line(
                    label_layout[index].x + offset,
                    label_layout[index].y + i as u16,
                    &line,
                    label_layout[index].width,
                );
            }
        }
    }
}
