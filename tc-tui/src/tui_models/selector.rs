use crate::{
    TuiController,
    components::{
        carousel_selector::CarouselSelector, color_input_field::ColorSelector,
        number_input::NumberSelector,
    },
    tui_models::settings::Setting,
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
    Color,
    Number,
    // Text,
}

pub(crate) trait SettingsSelector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction>;
    fn set_to_active(&mut self);
    fn set_to_inactive(&mut self);
    // TODO: implement this to update the SettingsSelector
    // fn set_current_selection(&mut self, selection: SelectableItem) -> Result;
}

impl SettingsSelector for Selector {
    fn handle_keys(&mut self, key_event: KeyEvent) -> Option<TuiAction> {
        match self {
            Selector::Carousel(selector) => selector.handle_keys(key_event),
            Selector::Color(selector) => selector.handle_keys(key_event),
            Selector::Number(selector) => selector.handle_keys(key_event),
        }
    }

    fn set_to_active(&mut self) {
        match self {
            Selector::Carousel(selector) => selector.set_to_active(),
            Selector::Color(selector) => selector.set_to_active(),
            Selector::Number(selector) => selector.set_to_active(),
        }
    }

    fn set_to_inactive(&mut self) {
        match self {
            Selector::Carousel(selector) => selector.set_to_inactive(),
            Selector::Color(selector) => selector.set_to_inactive(),
            Selector::Number(selector) => selector.set_to_inactive(),
        }
    }
}

impl Widget for &Selector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Selector::Carousel(selector) => selector.render(area, buf),
            Selector::Color(selector) => selector.render(area, buf),
            Selector::Number(selector) => selector.render(area, buf),
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
                is_active,
                setting.as_ref().to_string(),
                tui_controller.carousel_options_for(setting),
            )),
            SelectorType::Color => Selector::Color(ColorSelector::new(
                Arc::clone(&tui_controller),
                is_active,
                setting.as_ref().to_string(),
            )),
            SelectorType::Number => Selector::Number(NumberSelector::new(
                Arc::clone(&tui_controller),
                is_active,
                setting.as_ref().to_string(),
            )),
        }
    }
}

pub(crate) enum Selector {
    Carousel(CarouselSelector),
    Color(ColorSelector),
    Number(NumberSelector),
    // Text(TextInput),
}
