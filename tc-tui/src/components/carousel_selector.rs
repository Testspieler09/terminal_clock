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
    selector::SettingsSelector,
    settings::Setting,
    styled_widget::StyledWidget,
    tui::TuiAssets,
    tui_action::TuiAction,
    tui_error::{UpdateError, UpdateResult},
};

pub(crate) struct CarouselSelector {
    /// Fields needed for event handling logic
    is_active: bool,

    /// Display fields
    setting: Setting,
    options: Vec<SelectableItem>,
    current_selection: usize,
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
        }
    }

    fn next_option(&mut self) {
        if !self.options.is_empty() {
            self.current_selection = (self.current_selection + 1) % self.options.len();
        }
    }

    fn prev_option(&mut self) {
        if !self.options.is_empty() {
            if self.current_selection == 0 {
                self.current_selection = self.options.len() - 1;
            } else {
                self.current_selection -= 1;
            }
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
    }

    pub(crate) fn update_current_selection(
        &mut self,
        selection: SelectableItem,
        tui_assets: &TuiAssets,
    ) -> UpdateResult<()> {
        if let Some(idx) = self
            .options
            .iter()
            .position(|item| *item.get_name(tui_assets) == *selection.get_name(tui_assets))
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
    tui_assets: &'a TuiAssets,
}

impl<'a> SettingsMenuCtx<'a> {
    pub fn new(color_theme: &'a ColorTheme, tui_assets: &'a TuiAssets) -> Self {
        SettingsMenuCtx {
            color_theme,
            tui_assets,
        }
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

        // FIX: Too much text for the box leads to unreadability e.g. the quotes should have
        // scrollable text

        // Render Title
        Line::from(spans)
            .alignment(Alignment::Center)
            .style(style)
            .render(title_section, buf);

        // Render Options
        Span::from(" ← ")
            .style(style)
            .render(button_left_section, buf);
        Line::from(self.options[self.current_selection].get_name(ctx.tui_assets))
            .alignment(Alignment::Center)
            .style(style)
            .render(option_section, buf);
        Span::from(" → ")
            .style(style)
            .render(button_right_section, buf);
    }
}
