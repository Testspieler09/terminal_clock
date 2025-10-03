use crate::{
    TuiController, components::carousel_selector::CarouselSelector, tui_models::settings::Setting,
};
use ratatui::{
    crossterm::event::KeyEvent,
    prelude::{Buffer, Rect},
    widgets::Widget,
};
use std::sync::Arc;
use tc_models::tui_action::TuiAction;

pub(crate) enum SelectorType {
    Carousel,
    // Color,
    // Number,
    // Text,
}

pub(crate) trait SettingsSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction>;
    fn set_to_active(&mut self);
    fn set_to_inactive(&mut self);
}

impl SettingsSelector for Selector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match self {
            Selector::Carousel(selector) => selector.handle_keys(key_event),
        }
    }

    fn set_to_active(&mut self) {
        match self {
            Selector::Carousel(selector) => selector.set_to_active(),
        }
    }

    fn set_to_inactive(&mut self) {
        match self {
            Selector::Carousel(selector) => selector.set_to_inactive(),
        }
    }
}

impl Widget for &Selector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Selector::Carousel(selector) => selector.render(area, buf),
        }
    }
}

impl SelectorType {
    pub fn create_selector(
        &self,
        tui_controller: Arc<TuiController>,
        setting: Setting,
        is_active: bool,
    ) -> Selector {
        match self {
            SelectorType::Carousel => Selector::Carousel(CarouselSelector::new(
                Arc::clone(&tui_controller),
                setting.as_ref().to_string(),
                tui_controller.carousel_options_for(setting),
                is_active,
            )),
        }
    }
}

pub(crate) enum Selector {
    Carousel(CarouselSelector),
    // Color(ColorSelector),
    // Number(NumberSelector),
    // Text(TextInput),
}
