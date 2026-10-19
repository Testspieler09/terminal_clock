// TODO: [display of color while user is typing] [the input field]
// i | a | e -> edit
// ESC -> leave no save / ask for save
// enter -> leave with save

use std::str::FromStr;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, FALLBACK_COLOR_THEME, ThemeColor};

use crate::{
    TUI_ASSETS,
    tui_models::{
        selectable_item::SelectableItem, selector::SettingsSelector, settings::Setting,
        styled_widget::StyledWidget, tui_action::TuiAction, tui_error::UpdateResult,
    },
};

// TODO: add the global colorpicker component later on that gets rendered over the whole frame
pub(crate) struct ColorSelector {
    is_active: bool,

    setting: Setting,
    current_hex_color: String,
}

impl ColorSelector {
    pub fn new(is_active: bool, setting: Setting) -> ColorSelector {
        ColorSelector {
            is_active,
            setting,
            current_hex_color: ColorSelector::resolve_color(setting).to_string(),
        }
    }

    fn resolve_color(color_setting: Setting) -> &'static Color {
        let Some(assets) = TUI_ASSETS.get() else {
            return &Color::Black;
        };

        match color_setting {
            Setting::ForegroundColor => assets.color_themes[0]
                .colors
                .get(&ThemeColor::Foreground)
                .unwrap_or(&FALLBACK_COLOR_THEME[0]),
            Setting::BackgroundColor => assets.color_themes[0]
                .colors
                .get(&ThemeColor::Background)
                .unwrap_or(&FALLBACK_COLOR_THEME[1]),
            Setting::SelectionColor => assets.color_themes[0]
                .colors
                .get(&ThemeColor::Selection)
                .unwrap_or(&FALLBACK_COLOR_THEME[2]),
            Setting::AccentColor => assets.color_themes[0]
                .colors
                .get(&ThemeColor::Accent)
                .unwrap_or(&FALLBACK_COLOR_THEME[3]),
            Setting::BorderColor => assets.color_themes[0]
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

impl StyledWidget for &ColorSelector {
    type Context<'a> = &'a ColorTheme;

    fn render(self, area: Rect, buf: &mut Buffer, color_theme: &ColorTheme) {
        let highlight_color = *color_theme.get(&ThemeColor::Selection);
        let default_color = *color_theme.get(&ThemeColor::Foreground);

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

        let bg_color = Color::from_str(&self.current_hex_color)
            .expect("The current hex color can never be invalid.");
        let fg_color = match bg_color {
            Color::Rgb(r, g, b) => {
                let luminance = 0.2126 * (r as f32) + 0.7152 * (g as f32) + 0.0722 * (b as f32);
                if luminance > 128.0 {
                    Color::Black
                } else {
                    Color::White
                }
            }
            _ => default_color,
        };

        Span::from("   ").style(style).render(left_section, buf);
        Line::from(self.current_hex_color.clone())
            .alignment(Alignment::Center)
            .style(Style::default().fg(fg_color).bg(bg_color))
            .render(text_middle_section, buf);
        Span::from(" ⏎ ")
            .style(style)
            .render(button_right_section, buf);
    }
}
