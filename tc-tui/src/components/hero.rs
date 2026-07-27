use ratatui::{
    layout::Flex,
    prelude::{Buffer, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use strum::{EnumIter, IntoEnumIterator};

use crate::components::{CYAN_SHADES, Dimensions, GRAY_SHADES};

#[derive(Default, PartialEq, Eq, Clone, Copy, EnumIter)]
pub(crate) enum MenuLabel {
    #[default]
    Settings,
    Help,
    Quit,
}

impl MenuLabel {
    pub fn get_repr_for(&self, active: bool) -> &'static str {
        match (self, active) {
            (MenuLabel::Settings, false) => {
                "┌─┐┌─┐┌┬┐┌┬┐┬┌┐┌┌─┐┌─┐\n\
                 └─┐├┤  │  │ │││││ ┬└─┐\n\
                 └─┘└─┘ ┴  ┴ ┴┘└┘└─┘└─┘"
            }
            (MenuLabel::Settings, true) => {
                "╔═╗╔═╗╔╦╗╔╦╗╦╔╗╔╔═╗╔═╗\n\
                 ╚═╗╠╣  ║  ║ ║║║║║ ╦╚═╗\n\
                 ╚═╝╚═╝ ╩  ╩ ╩╝╚╝╚═╝╚═╝"
            }
            (MenuLabel::Help, false) => {
                "┬ ┬┌─┐┬  ┌─┐\n\
                 ├─┤├┤ │  ├─┘\n\
                 ┴ ┴└─┘┴─┘┴  "
            }
            (MenuLabel::Help, true) => {
                "╦ ╦╔═╗╦  ╔═╗\n\
                 ╠═╣╠╣ ║  ╠═╝\n\
                 ╩ ╩╚═╝╩═╝╩  "
            }
            (MenuLabel::Quit, false) => {
                "┌─┐ ┬ ┬ ┬┌┬┐\n\
                 │─┼┐│ │ │ │ \n\
                 └─┘└└─┘ ┴ ┴ "
            }
            (MenuLabel::Quit, true) => {
                "╔═╗ ╦ ╦ ╦╔╦╗\n\
                 ║═╬╗║ ║ ║ ║ \n\
                 ╚═╝╚╚═╝ ╩ ╩ "
            }
        }
    }
}

pub(crate) struct Hero {
    pub active_label: MenuLabel,
    height: u16,
    width: u16,
}

impl Hero {
    fn map_label_to_ascii(label: &MenuLabel, active_label: &MenuLabel) -> Vec<Line<'static>> {
        let ascii = label.get_repr_for(*active_label == *label);

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
            .nth(1)
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
                            .get_repr_for(true)
                            .lines()
                            .map(|line| line.len())
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

        MenuLabel::iter().enumerate().for_each(|(index, label)| {
            let lines = Hero::map_label_to_ascii(&label, &self.active_label);
            lines.iter().enumerate().for_each(|(i, line)| {
                let offset = (label_layout[index]
                    .width
                    .saturating_sub(line.width() as u16))
                    / 2;
                buf.set_line(
                    label_layout[index].x + offset,
                    label_layout[index].y + i as u16,
                    line,
                    label_layout[index].width,
                );
            })
        })
    }
}
