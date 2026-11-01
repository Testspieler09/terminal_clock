use chrono::{DateTime, Local, Timelike};
use ratatui::prelude::{Buffer, Rect};
use tokio::time::{Duration, Instant};

use crate::tui_models::{tui::TuiState, tui_traits::TcWidget};

#[derive(PartialEq, Eq)]
pub(crate) enum TimerPhase {
    NotStarted,
    Work,
    ShortBreak,
    LongBreak,
    Finished,
}

pub struct PomodoroConfig {
    pub total_sessions: u32,
    pub work_duration_min: u64,
    pub short_break_duration_min: u64,
    pub long_break_duration_min: u64,
    pub sessions_before_long_break: u32,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            total_sessions: 8,
            work_duration_min: 25,
            short_break_duration_min: 5,
            long_break_duration_min: 15,
            sessions_before_long_break: 4,
        }
    }
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

impl Default for PomodoroTimer {
    fn default() -> Self {
        Self::new(PomodoroConfig::default())
    }
}

impl PomodoroTimer {
    pub fn new(config: PomodoroConfig) -> Self {
        let work_duration = config.work_duration_min;
        Self {
            config,
            state: PomodoroState {
                session: 1,
                phase: TimerPhase::NotStarted,
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
                    self.state.remaining_secs = self.config.long_break_duration_min * 60;
                } else {
                    self.state.phase = TimerPhase::ShortBreak;
                    self.state.remaining_secs = self.config.short_break_duration_min * 60;
                }
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                self.state.session += 1;
                if self.state.session > self.config.total_sessions {
                    self.state.phase = TimerPhase::Finished;
                } else {
                    self.state.phase = TimerPhase::Work;
                    self.state.remaining_secs = self.config.work_duration_min * 60;
                }
            }
            TimerPhase::NotStarted | TimerPhase::Finished => {}
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        matches!(
            self.state.phase,
            TimerPhase::Work | TimerPhase::ShortBreak | TimerPhase::LongBreak
        )
    }

    pub(crate) fn get_timestamp(&self) -> DateTime<Local> {
        if self.state.phase == TimerPhase::NotStarted || self.state.phase == TimerPhase::Finished {
            unreachable!("get_timestamp was called without the pomodoro timer active.");
        }

        let hours = self.state.remaining_secs / 3600;
        let minutes = (self.state.remaining_secs % 3600) / 60;
        let seconds = self.state.remaining_secs % 60;

        Local::now()
            .with_hour(hours as u32)
            .unwrap()
            .with_minute(minutes as u32)
            .unwrap()
            .with_second(seconds as u32)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }
}

impl TcWidget<'_> for PomodoroTimer {
    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &TuiState) {
        todo!()
    }
}
