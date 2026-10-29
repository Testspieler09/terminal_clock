use std::ops::ControlFlow;

use chrono::Local;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{io, time::Duration};

use crate::{
    components::hero::MenuLabel,
    tui_models::{
        application::ApplicationState,
        tui::{TuiComponents, TuiState},
        tui_action::TuiAction,
        tui_constants::{FIREWORK_FRAME_MS, SETTINGS_MENU_TICK_MS},
    },
};

pub(crate) fn handle_events(
    state: &mut TuiState,
    components: &mut TuiComponents,
) -> io::Result<bool> {
    match state.action_rx.recv_timeout(Duration::from_millis(20)) {
        Ok(action) => process_settings_action(state, &action),
        // TODO: check if this is to harsh
        // Also wrap in custom error?!
        Err(e) => panic!(
            "Sender should never close before application.\nError: {}",
            e
        ),
    }

    if matches!(state.application_state, ApplicationState::Finished) {
        return Ok(true);
    }

    let fw = &mut components.firework_animation;

    if let Some(year) = fw.pending_trigger(Local::now()) {
        fw.start(year);
        state.application_state = ApplicationState::ShowingFirework;
    }

    if matches!(state.application_state, ApplicationState::ShowingFirework) && fw.is_finished() {
        state.application_state = ApplicationState::Running;
    }

    let refresh_rate = state.refresh_rate;
    let app_state = state.application_state.clone();

    // FIX: I think this should be refactored, as it is to inversive regarding user config
    //
    // Use a short poll timeout when settings are visible so the scroll
    // animation in carousel selectors can advance at its own pace.
    let poll_timeout = match app_state {
        ApplicationState::ShowingFirework => FIREWORK_FRAME_MS,
        ApplicationState::ShowingSettings => SETTINGS_MENU_TICK_MS,
        _ => refresh_rate,
    };

    if event::poll(Duration::from_millis(poll_timeout as u64))?
        && let Event::Key(key_event) = event::read()?
    {
        if matches!(key_event.kind, event::KeyEventKind::Release) {
            return Ok(false);
        }

        if let ControlFlow::Break(_) = handle_global_keys(state, key_event, components) {
            return Ok(matches!(
                state.application_state,
                ApplicationState::Finished
            ));
        }

        match app_state {
            ApplicationState::Running => handle_normal_keys(state, key_event, components),
            ApplicationState::ShowingHero => handle_hero_keys(state, key_event, components),
            ApplicationState::ShowingSettings => components
                .settings_menu
                .handle_setting_keys(key_event, state),
            ApplicationState::ShowingHelp
            | ApplicationState::ShowingFirework
            | ApplicationState::Finished => {}
        }
    }

    Ok(matches!(
        state.application_state,
        ApplicationState::Finished
    ))
}

pub(crate) fn process_settings_action(state: &mut TuiState, action: &TuiAction) {
    match action {
        TuiAction::ClockFace(new_clock_face_idx) => {
            state.clock_state.clock_face_idx = *new_clock_face_idx
        }
        TuiAction::ClockFormat(new_format) => state.clock_state.clock_time_fmt = *new_format,
        TuiAction::RefreshRate(new_refresh_rate) => state.refresh_rate = *new_refresh_rate,
        TuiAction::Quote(new_quote) => state.quote_idx = *new_quote,
        TuiAction::TotalSession(_) => {}
        TuiAction::WorkDuration(_) => {}
        TuiAction::LongBreakDuration(_) => {}
        TuiAction::ShortBreakDuration(_) => {}
        TuiAction::SessionsBeforeLongBreak(_) => {}
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
