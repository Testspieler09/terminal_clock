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
        number_input::NumericValue,
        selectable_item::SelectableItem,
        selector::{SelectorState, SettingsSelector},
        settings::{GeneralSetting, PomodoroSetting, Setting},
        tui_action::{NumericOperation, TuiAction},
        tui_error::UpdateResult,
        tui_traits::TcWidget,
    },
};

// TODO: add the global numberpicker component later on that gets rendered over the whole frame
pub(crate) struct NumberSelector {
    selector_state: SelectorState,
    setting: Setting,
}

impl NumberSelector {
    pub fn new(selector_state: SelectorState, setting: Setting) -> NumberSelector {
        NumberSelector {
            selector_state,
            setting,
        }
    }

    fn resolve_number(&self, state: &TuiState) -> u64 {
        match self.setting {
            Setting::General(GeneralSetting::RefreshRate) => state.refresh_rate as u64,
            Setting::Pomodoro(setting) => match setting {
                PomodoroSetting::WorkDuration => state.pomodoro.config.work_duration_min,
                PomodoroSetting::SessionsBeforeLongBreak => {
                    state.pomodoro.config.sessions_before_long_break as u64
                }
                PomodoroSetting::TotalSessions => state.pomodoro.config.total_sessions as u64,
                PomodoroSetting::LongBreakDuration => state.pomodoro.config.long_break_duration_min,
                PomodoroSetting::ShortBreakDuration => {
                    state.pomodoro.config.short_break_duration_min
                }
            },
            _ => unreachable!("resolve_number called for a non-number setting"),
        }
    }

    fn get_corresponding_action(&self, op: NumericOperation) -> TuiAction {
        match self.setting {
            Setting::General(GeneralSetting::RefreshRate) => {
                TuiAction::UpdateNumericValue(op, NumericValue::RefreshRate)
            }

            Setting::Pomodoro(setting) => match setting {
                PomodoroSetting::TotalSessions => {
                    TuiAction::UpdateNumericValue(op, NumericValue::TotalSessions)
                }
                PomodoroSetting::SessionsBeforeLongBreak => {
                    TuiAction::UpdateNumericValue(op, NumericValue::SessionsBeforeLongBreak)
                }
                PomodoroSetting::WorkDuration => {
                    TuiAction::UpdateNumericValue(op, NumericValue::WorkDuration)
                }
                PomodoroSetting::ShortBreakDuration => {
                    TuiAction::UpdateNumericValue(op, NumericValue::ShortBreakDuration)
                }
                PomodoroSetting::LongBreakDuration => {
                    TuiAction::UpdateNumericValue(op, NumericValue::LongBreakDuration)
                }
            },

            _ => unreachable!("get_corresponding_action called for a non-number setting"),
        }
    }
}

impl SettingsSelector for NumberSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match key_event.code {
            KeyCode::Char('e') | KeyCode::Char('i') | KeyCode::Char('a') => {
                todo!()
            }
            KeyCode::Char('h') | KeyCode::Left => {
                Some(self.get_corresponding_action(NumericOperation::Decrement))
            }
            KeyCode::Char('l') | KeyCode::Right => {
                Some(self.get_corresponding_action(NumericOperation::Increment))
            }

            _ => None,
        }
    }

    fn set_to_active(&mut self) {
        self.selector_state = SelectorState::Active;
    }

    fn set_to_inactive(&mut self) {
        self.selector_state = SelectorState::Inactive;
    }

    fn set_to_disabled(&mut self) {
        self.selector_state = SelectorState::Disabled;
    }

    fn update_current_selection(&mut self, _selection: SelectableItem) -> UpdateResult<()> {
        unreachable!("The SettingsSelector will never update the current selection")
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
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .areas(bottom_row_section);

        let title = Line::from(vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ]);

        let text = Line::from(self.resolve_number(state).to_string());

        let style = if self.selector_state == SelectorState::Active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        title
            .alignment(Alignment::Center)
            .style(style)
            .render(area, buf);

        Span::from(" ←   ").style(style).render(left_section, buf);
        text.alignment(Alignment::Center)
            .style(style)
            .render(text_middle_section, buf);
        Span::from(" ⏎ → ")
            .style(style)
            .render(button_right_section, buf);
    }
}
