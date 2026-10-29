#![allow(dead_code)]

use ratatui::prelude::{Buffer, Rect};
use tokio::time::{Duration, Instant};

use crate::tui_models::{tui::TuiState, tui_traits::TcWidget};

pub(crate) enum TimerPhase {
    Work,
    ShortBreak,
    LongBreak,
    Finished,
}

pub struct PomodoroConfig {
    pub work_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub total_sessions: u32,
    pub sessions_before_long_break: u32,
}

pub(crate) struct PomodoroState {
    pub session: u32,
    pub phase: TimerPhase,
    pub remaining_secs: u64,
}

pub struct PomodoroTimer {
    pub config: PomodoroConfig,
    state: PomodoroState,
    last_tick: Instant,
}

impl PomodoroTimer {
    pub fn new(config: PomodoroConfig) -> Self {
        let work_duration = config.work_duration;
        Self {
            config,
            state: PomodoroState {
                session: 1,
                phase: TimerPhase::Work,
                remaining_secs: work_duration * 60,
            },
            last_tick: Instant::now(),
        }
    }

    pub async fn tick(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= Duration::from_secs(1) {
            self.last_tick = now;
            if self.state.remaining_secs > 0 {
                self.state.remaining_secs -= 1;
            } else {
                self.advance_phase();
            }
        }
    }

    fn advance_phase(&mut self) {
        match self.state.phase {
            TimerPhase::Work => {
                if self.state.session >= self.config.total_sessions {
                    self.state.phase = TimerPhase::Finished;
                } else if self
                    .state
                    .session
                    .is_multiple_of(self.config.sessions_before_long_break)
                {
                    self.state.phase = TimerPhase::LongBreak;
                    self.state.remaining_secs = self.config.long_break_duration * 60;
                } else {
                    self.state.phase = TimerPhase::ShortBreak;
                    self.state.remaining_secs = self.config.short_break_duration * 60;
                }
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                self.state.session += 1;
                if self.state.session > self.config.total_sessions {
                    self.state.phase = TimerPhase::Finished;
                } else {
                    self.state.phase = TimerPhase::Work;
                    self.state.remaining_secs = self.config.work_duration * 60;
                }
            }
            TimerPhase::Finished => {}
        }
    }
}

impl TcWidget<'_> for PomodoroTimer {
    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &TuiState) {
        todo!()
    }
}
