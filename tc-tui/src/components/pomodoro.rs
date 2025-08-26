use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};
use tokio::time::{Duration, Instant};

pub(crate) enum TimerPhase {
    Work,
    ShortBreak,
    LongBreak,
    Finished,
}

#[derive(Clone)]
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
    is_visible: bool,
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
            is_visible: false,
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
            TimerPhase::Finished => {
                // nothing to do
            }
        }
    }

    /// Render a ratatui Paragraph to display the timer
    pub fn render(&self) -> Paragraph<'_> {
        let (label, color) = match self.state.phase {
            TimerPhase::Work => ("Work", Color::Green),
            TimerPhase::ShortBreak => ("Short Break", Color::Blue),
            TimerPhase::LongBreak => ("Long Break", Color::Magenta),
            TimerPhase::Finished => ("Done!", Color::Yellow),
        };

        let minutes = self.state.remaining_secs / 60;
        let seconds = self.state.remaining_secs % 60;

        let time_str = format!("{minutes:02}:{seconds:02}");
        let session_str = format!(
            "Session {}/{}",
            self.state.session, self.config.total_sessions
        );

        let spans = vec![
            Line::from(Span::styled(label, Style::default().fg(color).bold())),
            Line::from(Span::raw(time_str)),
            Line::from(Span::raw(session_str)),
        ];

        Paragraph::new(spans)
    }
}
