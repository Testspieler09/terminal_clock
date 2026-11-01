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
        selector::{SelectorState, SettingsSelector},
        settings::{GeneralSetting, PomodoroSetting, Setting},
        tui_action::TuiAction,
        tui_error::UpdateResult,
        tui_traits::TcWidget,
    },
};

pub(crate) struct NumberSelectorConfig {
    // FIX: override value should be removed
    override_value: u64,
    preset_interval: u16,
    upper_clamp: u64,
    fallback: u32,
}

// TODO: add the global numberpicker component later on that gets rendered over the whole frame
pub(crate) struct NumberSelector {
    selector_state: SelectorState,
    setting: Setting,
    config: NumberSelectorConfig,
}

impl NumberSelectorConfig {
    pub fn derive(setting: Setting) -> Self {
        // match setting {}
        NumberSelectorConfig {
            override_value: 100,
            preset_interval: 100,
            upper_clamp: 1000,
            fallback: 200,
        }
    }
}

impl NumberSelector {
    pub fn new(selector_state: SelectorState, setting: Setting) -> NumberSelector {
        NumberSelector {
            selector_state,
            setting,
            config: NumberSelectorConfig::derive(setting),
        }
    }

    fn resolve_number(&self, state: &TuiState) -> String {
        match self.setting {
            Setting::General(GeneralSetting::RefreshRate) => state.refresh_rate.to_string(),
            Setting::Pomodoro(setting) => match setting {
                PomodoroSetting::WorkDuration => {
                    state.pomodoro.config.work_duration_min.to_string()
                }
                PomodoroSetting::SessionsBeforeLongBreak => {
                    state.pomodoro.config.sessions_before_long_break.to_string()
                }
                PomodoroSetting::TotalSessions => state.pomodoro.config.total_sessions.to_string(),
                PomodoroSetting::LongBreakDuration => {
                    state.pomodoro.config.long_break_duration_min.to_string()
                }
                PomodoroSetting::ShortBreakDuration => {
                    state.pomodoro.config.short_break_duration_min.to_string()
                }
            },
            _ => unreachable!("resolve_number called for a non-number setting"),
        }
    }

    fn increase_value(&mut self) {
        self.config.override_value = self
            .config
            .override_value
            .saturating_add(self.config.preset_interval as u64)
            .min(self.config.upper_clamp);
    }

    fn decrease_value(&mut self) {
        self.config.override_value = self
            .config
            .override_value
            .saturating_sub(self.config.preset_interval as u64);
    }

    fn get_corrosponding_action(&self) -> TuiAction {
        match self.setting {
            Setting::General(GeneralSetting::RefreshRate) => TuiAction::RefreshRate(
                u16::try_from(self.config.override_value).unwrap_or(self.config.fallback as u16),
            ),
            Setting::Pomodoro(setting) => match setting {
                PomodoroSetting::TotalSessions => TuiAction::TotalSession(
                    u32::try_from(self.config.override_value).unwrap_or(self.config.fallback),
                ),
                PomodoroSetting::SessionsBeforeLongBreak => TuiAction::SessionsBeforeLongBreak(
                    u32::try_from(self.config.override_value).unwrap_or(self.config.fallback),
                ),
                PomodoroSetting::WorkDuration => {
                    TuiAction::WorkDuration(self.config.override_value)
                }
                PomodoroSetting::ShortBreakDuration => {
                    TuiAction::ShortBreakDuration(self.config.override_value)
                }
                PomodoroSetting::LongBreakDuration => {
                    TuiAction::LongBreakDuration(self.config.override_value)
                }
            },
            _ => unreachable!("get_corrosponding_action called for a non-number setting"),
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
                self.decrease_value();
                Some(self.get_corrosponding_action())
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.increase_value();
                Some(self.get_corrosponding_action())
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

        let text = Line::from(self.resolve_number(state));

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
