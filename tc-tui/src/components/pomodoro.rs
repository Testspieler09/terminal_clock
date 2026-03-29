use ratatui::prelude::{Buffer, Rect};
use tc_models::color_theme::ColorTheme;
use tokio::time::{Duration, Instant};

use crate::tui_models::styled_widget::StyledWidget;

pub(crate) enum TimerPhase {
    Work,
    ShortBreak,
    LongBreak,
    Finished,
}

pub(crate) struct PomodoroConfig {
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

pub(crate) struct PomodoroTimer {
    config: PomodoroConfig,
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
                } else if self.state.session % self.config.sessions_before_long_break == 0 {
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

impl StyledWidget for PomodoroTimer {
    type Context<'a> = &'a ColorTheme;

    fn render(self, area: Rect, buf: &mut Buffer, ctx: Self::Context<'_>) {
        todo!()
    }
}
