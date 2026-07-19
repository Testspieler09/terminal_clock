use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Constraint,
    prelude::{Alignment, Buffer, Layout, Rect, Stylize},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use tc_models::color_theme::{ColorTheme, ThemeColor};

use crate::tui_models::{
    selectable_item::{Selectable, SelectableItem},
    settings::Setting,
    styled_widget::StyledWidget,
    tui_action::TuiAction,
    tui_error::{UpdateError, UpdateResult},
};
const SCROLL_STEP_MS: Duration = Duration::from_millis(200);
const SCROLL_PAUSE_MS: Duration = Duration::from_millis(600);

enum ScrollPhase {
    /// Waiting before scroll starts (or after looping back to the beginning).
    Pausing {
        since: Instant,
    },
    Scrolling,
    /// Holding at the end before resetting.
    EndPause {
        since: Instant,
    },
}

impl Default for ScrollPhase {
    fn default() -> Self {
        ScrollPhase::Pausing {
            since: Instant::now(),
        }
    }
}

pub(crate) struct CarouselSelector {
    /// Fields needed for event handling logic
    is_active: bool,

    /// Display fields
    setting: Setting,
    options: Vec<SelectableItem>,
    current_selection: usize,

    /// Text-scrolling state for oversized option names
    scroll_offset: usize,
    last_step: Instant,
    scroll_phase: ScrollPhase,
    /// Cached width of the option box, set during render and used by tick().
    box_width: Cell<usize>,
}

impl CarouselSelector {
    pub fn new(
        is_active: bool,
        setting: Setting,
        options: Vec<SelectableItem>,
    ) -> CarouselSelector {
        if options.is_empty() {
            panic!("A carousel selector should always contain values.");
        }

        CarouselSelector {
            is_active,
            setting,
            options,
            current_selection: 0,
            scroll_offset: 0,
            last_step: Instant::now(),
            scroll_phase: ScrollPhase::default(),
            box_width: Cell::new(0),
        }
    }

    fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
        self.last_step = Instant::now();
        self.scroll_phase = ScrollPhase::default();
    }

    pub(crate) fn tick(&mut self) {
        if !self.is_active {
            return;
        }

        let box_width = self.box_width.get();
        if box_width == 0 {
            return;
        }

        let name = self.options[self.current_selection].get_name();
        let name_len = name.chars().count();
        if name_len <= box_width {
            return;
        }
        let overflow = name_len - box_width;

        match &self.scroll_phase {
            ScrollPhase::Pausing { since } => {
                if since.elapsed() >= SCROLL_PAUSE_MS {
                    self.last_step = Instant::now();
                    self.scroll_phase = ScrollPhase::Scrolling;
                }
            }
            ScrollPhase::Scrolling => {
                if self.last_step.elapsed() >= SCROLL_STEP_MS {
                    self.last_step = Instant::now();
                    if self.scroll_offset < overflow {
                        self.scroll_offset += 1;
                    } else {
                        self.scroll_phase = ScrollPhase::EndPause {
                            since: Instant::now(),
                        };
                    }
                }
            }
            ScrollPhase::EndPause { since } => {
                if since.elapsed() >= SCROLL_PAUSE_MS {
                    self.scroll_offset = 0;
                    self.scroll_phase = ScrollPhase::Pausing {
                        since: Instant::now(),
                    };
                }
            }
        }
    }

    fn next_option(&mut self) {
        if !self.options.is_empty() {
            self.current_selection = (self.current_selection + 1) % self.options.len();
            self.reset_scroll();
        }
    }

    fn prev_option(&mut self) {
        if !self.options.is_empty() {
            if self.current_selection == 0 {
                self.current_selection = self.options.len() - 1;
            } else {
                self.current_selection -= 1;
            }
            self.reset_scroll();
        }
    }
}

impl CarouselSelector {
    pub(crate) fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match key_event.code {
            KeyCode::Char('h') | KeyCode::Left => {
                self.prev_option();
                Some(self.options[self.current_selection].get_corrosponding_action())
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.next_option();
                Some(self.options[self.current_selection].get_corrosponding_action())
            }
            _ => None,
        }
    }

    pub(crate) fn set_to_active(&mut self) {
        self.is_active = true;
    }

    pub(crate) fn set_to_inactive(&mut self) {
        self.is_active = false;
        self.reset_scroll();
    }

    pub(crate) fn update_current_selection(
        &mut self,
        selection: SelectableItem,
    ) -> UpdateResult<()> {
        if let Some(idx) = self
            .options
            .iter()
            .position(|item| *item.get_name() == *selection.get_name())
        {
            self.current_selection = idx;
            Ok(())
        } else {
            Err(UpdateError)
        }
    }
}

pub(crate) struct SettingsMenuCtx<'a> {
    pub color_theme: &'a ColorTheme,
}

impl<'a> SettingsMenuCtx<'a> {
    pub fn new(color_theme: &'a ColorTheme) -> Self {
        SettingsMenuCtx { color_theme }
    }
}

impl StyledWidget for &CarouselSelector {
    type Context<'a> = &'a SettingsMenuCtx<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, ctx: Self::Context<'_>) {
        let highlight_color = *ctx.color_theme.get(&ThemeColor::Selection);
        let default_color = *ctx.color_theme.get(&ThemeColor::Foreground);

        let style = if self.is_active {
            Style::default().fg(default_color).bg(highlight_color)
        } else {
            Style::default().fg(default_color)
        };

        let [title_section, bottom_row_section] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [button_left_section, option_section, button_right_section] = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(bottom_row_section);

        let mut spans = vec![
            Span::from(self.setting.as_ref()).style(Style::default().fg(default_color).bold()),
        ];

        let option_amount = self.options.len() - 1;
        if option_amount >= 2 {
            spans.push(Span::from(
                " ".to_owned()
                    + &(self.current_selection + 1).to_string()
                    + "/"
                    + &(option_amount + 1).to_string(),
            ))
        }

        Line::from(spans)
            .alignment(Alignment::Center)
            .style(style)
            .render(title_section, buf);

        Span::from(" ← ")
            .style(style)
            .render(button_left_section, buf);

        let name = self.options[self.current_selection].get_name();
        let box_width = option_section.width as usize;
        // Cache for tick() which runs without layout context.
        self.box_width.set(box_width);
        let name_len = name.chars().count();
        let display = if name_len <= box_width {
            name
        } else if self.is_active {
            name.chars()
                .skip(self.scroll_offset)
                .take(box_width)
                .collect()
        } else {
            // Inactive: truncate and append ellipsis to signal hidden content.
            let truncated: String = name.chars().take(box_width.saturating_sub(1)).collect();
            truncated + "…"
        };

        Line::from(display)
            .alignment(Alignment::Center)
            .style(style)
            .render(option_section, buf);

        Span::from(" → ")
            .style(style)
            .render(button_right_section, buf);
    }
}
