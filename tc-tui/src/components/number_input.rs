// TODO: select from presets with custom value option
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::ThemeColor;

use crate::{
    TuiState, assets,
    tui_models::{
        selectable_item::SelectableItem,
        selector::SettingsSelector,
        settings::{GeneralSetting, PomodoroSetting, Setting},
        tui_action::TuiAction,
        tui_error::UpdateResult,
        tui_traits::TcWidget,
    },
};

// TODO: add the global numberpicker component later on that gets rendered over the whole frame
pub(crate) struct NumberSelector {
    is_active: bool,
    setting: Setting,
}

impl NumberSelector {
    pub fn new(is_active: bool, setting: Setting) -> NumberSelector {
        NumberSelector { is_active, setting }
    }

    fn resolve_number(&self, state: &TuiState) -> String {
        match self.setting {
            Setting::General(GeneralSetting::RefreshRate) => state.refresh_rate.to_string(),
            Setting::Pomodoro(setting) => match state.pomodoro.as_ref() {
                Some(timer) => match setting {
                    PomodoroSetting::WorkDuration => timer.config.work_duration.to_string(),
                    PomodoroSetting::SessionsBeforeLongBreak => {
                        timer.config.sessions_before_long_break.to_string()
                    }
                    PomodoroSetting::TotalSessions => timer.config.total_sessions.to_string(),
                    PomodoroSetting::LongBreakDuration => {
                        timer.config.long_break_duration.to_string()
                    }
                    PomodoroSetting::ShortBreakDuration => {
                        timer.config.short_break_duration.to_string()
                    }
                },
                None => "/".to_string(),
            },
            _ => unreachable!("resolve_number called for a non-number setting"),
        }
    }
}

impl SettingsSelector for NumberSelector {
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

impl TcWidget<'_> for &NumberSelector {
    fn render(self, area: Rect, buf: &mut Buffer, state: &TuiState) {
        let theme = assets().get_color_theme(state.color_theme_idx);

        let highlight_color = *theme.get(&ThemeColor::Selection);
        let default_color = *theme.get(&ThemeColor::Foreground);

        let [_, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [left_section, text_middle_section, button_right_section] = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(bottom_row_section);

        let title = Line::from(vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ]);

        let text = Line::from(self.resolve_number(state));

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        title
            .alignment(Alignment::Center)
            .style(style)
            .render(area, buf);

        Span::from("   ").style(style).render(left_section, buf);
        text.alignment(Alignment::Center)
            .style(style)
            .render(text_middle_section, buf);
        Span::from(" ⏎ ")
            .style(style)
            .render(button_right_section, buf);
    }
}
