use std::ops::ControlFlow;

use chrono::Local;
use futures::StreamExt;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{
    io,
    time::{Duration, sleep},
};

use crate::{
    components::hero::MenuLabel,
    tui_models::{
        application::ApplicationState,
        tui::{TuiComponents, TuiState},
        tui_action::TuiAction,
        tui_constants::{FIREWORK_FRAME_MS, SETTINGS_MENU_TICK_MS},
    },
};

pub(crate) async fn handle_events(
    state: &mut TuiState,
    components: &mut TuiComponents,
) -> io::Result<bool> {
    if matches!(state.application_state, ApplicationState::Finished) {
        return Ok(true);
    }

    // FIX: I think this should be refactored, as it is to inversive regarding user config
    //
    // Use a short poll timeout when settings are visible so the scroll
    // animation in carousel selectors can advance at its own pace.
    let tick = Duration::from_millis(match state.application_state {
        ApplicationState::ShowingFirework => FIREWORK_FRAME_MS,
        ApplicationState::ShowingSettings => SETTINGS_MENU_TICK_MS,
        _ => state.refresh_rate,
    } as u64);

    tokio::select! {
        maybe_event = state.events.next() => {
            match maybe_event {
                Some(Ok(Event::Key(key))) if key.kind != event::KeyEventKind::Release => {
                    if handle_global_keys(state, key, components).is_continue() {
                        dispatch_key(state, components, key).await;
                    }
                }
                Some(Ok(Event::Resize(w, h))) => {
                    process_settings_action(state, &TuiAction::UpdateFallback((w, h)));
                }
                // Some(Ok(Event::Mouse(_))) => should also be dispatched to dispatch key but
                // rename the fn first
                Some(Ok(_)) => {}
                Some(Err(e)) => return Err(e),
                None => state.application_state = ApplicationState::Finished,
            }
        }

        _ = sleep(tick) => {}
    }

    while let Ok(action) = state.action_rx.try_recv() {
        process_settings_action(state, &action);
    }

    let fw = &mut components.firework_animation;
    if let Some(year) = fw.pending_trigger(Local::now()) {
        fw.start(year);
        state.application_state = ApplicationState::ShowingFirework;
    }
    if matches!(state.application_state, ApplicationState::ShowingFirework) && fw.is_finished() {
        state.application_state = ApplicationState::Running;
    }

    Ok(matches!(
        state.application_state,
        ApplicationState::Finished
    ))
}

async fn dispatch_key(state: &mut TuiState, components: &mut TuiComponents, key: KeyEvent) {
    match state.application_state.clone() {
        ApplicationState::Running => handle_normal_keys(state, key, components),
        ApplicationState::ShowingHero => handle_hero_keys(state, key, components),
        ApplicationState::ShowingSettings => {
            components.settings_menu.handle_setting_keys(key, state)
        }
        ApplicationState::ShowingHelp
        | ApplicationState::ShowingFirework
        | ApplicationState::Finished => {}
    }
}

pub(crate) fn process_settings_action(state: &mut TuiState, action: &TuiAction) {
    match action {
        TuiAction::ClockFace(new_clock_face_idx) => {
            state.clock_state.clock_face_idx = *new_clock_face_idx
        }
        TuiAction::ClockFormat(new_format) => state.clock_state.clock_time_fmt = *new_format,
        TuiAction::RefreshRate(new_refresh_rate) => state.refresh_rate = *new_refresh_rate,
        TuiAction::Quote(new_quote) => state.quote_idx = *new_quote,
        TuiAction::TotalSession(new_session_amount) => {
            state.pomodoro.config.total_sessions = *new_session_amount
        }
        TuiAction::WorkDuration(new_work_duration) => {
            state.pomodoro.config.work_duration_min = *new_work_duration
        }
        TuiAction::LongBreakDuration(new_long_break) => {
            state.pomodoro.config.long_break_duration_min = *new_long_break
        }
        TuiAction::ShortBreakDuration(new_short_break) => {
            state.pomodoro.config.short_break_duration_min = *new_short_break
        }
        TuiAction::SessionsBeforeLongBreak(new_sessions_before_long_break) => {
            state.pomodoro.config.sessions_before_long_break = *new_sessions_before_long_break
        }
        TuiAction::ColorTheme(theme) => state.color_theme_idx = *theme,
        TuiAction::UpdateFallback((w, h)) => {
            state.needed_width = *w;
            state.needed_height = *h;
        }
    }
}

fn handle_global_keys(
    state: &mut TuiState,
    key_event: KeyEvent,
    components: &mut TuiComponents,
) -> ControlFlow<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => match state.application_state {
            ApplicationState::ShowingHero => {
                state.application_state = ApplicationState::Running;
                ControlFlow::Break(())
            }
            ApplicationState::ShowingHelp => {
                if components.help_box.was_called_from_hero() {
                    state.application_state = ApplicationState::ShowingHero;
                } else {
                    state.application_state = ApplicationState::Running;
                }
                components.help_box.set_called_from_hero(false);
                ControlFlow::Break(())
            }
            ApplicationState::ShowingSettings => {
                if components.settings_menu.was_called_from_hero() {
                    state.application_state = ApplicationState::ShowingHero;
                } else {
                    state.application_state = ApplicationState::Running;
                }
                components.settings_menu.set_called_from_hero(false);
                ControlFlow::Break(())
            }
            ApplicationState::Running => {
                if matches!(key_event.code, KeyCode::Char('q')) {
                    state.application_state = ApplicationState::Finished;
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            }
            ApplicationState::ShowingFirework => {
                components.firework_animation.skip();
                state.application_state = ApplicationState::Finished;
                ControlFlow::Break(())
            }
            _ => ControlFlow::Continue(()),
        },
        _ => ControlFlow::Continue(()),
    }
}

fn handle_normal_keys(state: &mut TuiState, key_event: KeyEvent, components: &mut TuiComponents) {
    match key_event.code {
        KeyCode::Esc => {
            state.application_state = ApplicationState::ShowingHero;
        }
        KeyCode::Char('?') | KeyCode::Char('h') => {
            components.help_box.set_called_from_hero(false);
            state.application_state = ApplicationState::ShowingHelp;
        }
        KeyCode::Char('s') => {
            components.settings_menu.set_called_from_hero(false);
            state.application_state = ApplicationState::ShowingSettings;
        }
        _ => {}
    }
}

fn handle_hero_keys(state: &mut TuiState, key_event: KeyEvent, components: &mut TuiComponents) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => components.hero.next_label(),
        KeyCode::Char('k') | KeyCode::Up => components.hero.prev_label(),
        KeyCode::Enter => match components.hero.active_label {
            MenuLabel::Quit => state.application_state = ApplicationState::Finished,
            MenuLabel::Help => {
                components.help_box.set_called_from_hero(true);
                state.application_state = ApplicationState::ShowingHelp;
            }
            MenuLabel::Settings => {
                components.settings_menu.set_called_from_hero(true);
                state.application_state = ApplicationState::ShowingSettings;
            }
        },
        _ => {}
    }
}
