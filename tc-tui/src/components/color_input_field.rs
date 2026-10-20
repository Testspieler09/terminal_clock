// TODO: [display of color while user is typing] [the input field]
// i | a | e -> edit
// ESC -> leave no save / ask for save
// enter -> leave with save

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::{FALLBACK_COLOR_THEME, ThemeColor};

use crate::{
    assets,
    tui_models::{
        selectable_item::SelectableItem, selector::SettingsSelector, settings::Setting,
        tui::TuiState, tui_action::TuiAction, tui_error::UpdateResult, tui_traits::TcWidget,
    },
};

// TODO: add the global colorpicker component later on that gets rendered over the whole frame
pub(crate) struct ColorSelector {
    is_active: bool,
    setting: Setting,
}

impl ColorSelector {
    pub fn new(is_active: bool, setting: Setting) -> ColorSelector {
        ColorSelector { is_active, setting }
    }

    fn resolve_color(&self, color_theme_idx: u16) -> &'static Color {
        let theme = assets().get_color_theme(color_theme_idx);

        match self.setting {
            Setting::ForegroundColor => theme
                .colors
                .get(&ThemeColor::Foreground)
                .unwrap_or(&FALLBACK_COLOR_THEME[0]),
            Setting::BackgroundColor => theme
                .colors
                .get(&ThemeColor::Background)
                .unwrap_or(&FALLBACK_COLOR_THEME[1]),
            Setting::SelectionColor => theme
                .colors
                .get(&ThemeColor::Selection)
                .unwrap_or(&FALLBACK_COLOR_THEME[2]),
            Setting::AccentColor => theme
                .colors
                .get(&ThemeColor::Accent)
                .unwrap_or(&FALLBACK_COLOR_THEME[3]),
            Setting::BorderColor => theme
                .colors
                .get(&ThemeColor::Borders)
                .unwrap_or(&FALLBACK_COLOR_THEME[4]),
            _ => unreachable!(),
        }
    }
}

impl SettingsSelector for ColorSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match key_event.code {
            KeyCode::Char('e') | KeyCode::Char('i') | KeyCode::Char('a') => {
                todo!()
            }
            _ => None,
        }
    }

    fn set_to_active(&mut self) {
        self.is_active = true;
    }

    fn set_to_inactive(&mut self) {
        self.is_active = false;
    }

    fn update_current_selection(&mut self, _selection: SelectableItem) -> UpdateResult<()> {
        todo!()
    }
}

impl TcWidget<'_> for &ColorSelector {
    fn render(self, area: Rect, buf: &mut Buffer, state: &TuiState) {
        let theme = assets().get_color_theme(state.color_theme_idx);

        let highlight_color = *theme.get(&ThemeColor::Selection);
        let default_color = *theme.get(&ThemeColor::Foreground);
        let background_color = *theme.get(&ThemeColor::Background);

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        let [title_row_section, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [left_section, text_middle_section, button_right_section] = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(bottom_row_section);

        Line::from(vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ])
        .alignment(Alignment::Center)
        .style(style)
        .render(title_row_section, buf);

        let current_color = self.resolve_color(state.color_theme_idx);
        let bg_color = current_color;
        let fg_color = match bg_color {
            Color::Rgb(r, g, b) => {
                let luminance = 0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32);
                if luminance > 128.0 {
                    background_color
                } else {
                    default_color
                }
            }
            _ => default_color,
        };

        Span::from("   ").style(style).render(left_section, buf);
        Line::from(current_color.to_string())
            .alignment(Alignment::Center)
            .style(Style::default().fg(fg_color).bg(*bg_color))
            .render(text_middle_section, buf);
        Span::from(" ⏎ ")
            .style(style)
            .render(button_right_section, buf);
    }
}
