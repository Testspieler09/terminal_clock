use crate::{
    components::{
        carousel_selector::CarouselSelector, color_input_field::ColorSelector,
        number_input::NumberSelector,
    },
    tui_models::{
        selectable_item::SelectableItem,
        settings::Setting,
        styled_widget::StyledWidget,
        tui::TuiAssets,
        tui_action::TuiAction,
        tui_error::{UpdateError, UpdateResult},
    },
};
use ratatui::{
    crossterm::event::KeyEvent,
    prelude::{Buffer, Rect},
};
use strum::IntoEnumIterator;
use tc_models::{clock::TimeFormat, color_theme::ColorTheme};

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
    fn update_current_selection(&mut self, selection: SelectableItem) -> UpdateResult<()>;
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

    fn update_current_selection(&mut self, selection: SelectableItem) -> Result<(), UpdateError> {
        match self {
            Selector::Carousel(carousel_selector) => {
                carousel_selector.update_current_selection(selection)
            }
            Selector::Color(color_selector) => color_selector.update_current_selection(selection),
            Selector::Number(number_selector) => {
                number_selector.update_current_selection(selection)
            }
        }
    }
}

impl StyledWidget for &Selector {
    fn render(self, area: Rect, buf: &mut Buffer, color_theme: &ColorTheme) {
        match self {
            Selector::Carousel(selector) => selector.render(area, buf, color_theme),
            Selector::Color(selector) => selector.render(area, buf, color_theme),
            Selector::Number(selector) => selector.render(area, buf, color_theme),
        }
    }
}

impl SelectorType {
    fn carousel_options_for(
        &self,
        setting: Setting,
        tui_assets: &TuiAssets,
    ) -> Vec<SelectableItem> {
        match setting {
            Setting::ClockFace => tui_assets
                .clock_faces
                .iter()
                .enumerate()
                .map(|(i, _clock_face)| SelectableItem::ClockFace(i as u16))
                .collect(),
            Setting::ColorTheme => tui_assets
                .color_themes
                .iter()
                .enumerate()
                .map(|(i, _color_theme)| SelectableItem::Theme(i as u16))
                .collect(),
            Setting::ClockFormat => TimeFormat::iter()
                .map(|fmt| SelectableItem::Format(fmt))
                .collect(),
            Setting::Quote => tui_assets
                .quotes
                .iter()
                .enumerate()
                .map(|(i, quote)| SelectableItem::Quote(Some(i as u16)))
                .chain(std::iter::once(SelectableItem::Quote(None)))
                .collect(),
            _ => unreachable!(),
        }
    }

    pub fn create_selector(
        &self,
        setting: Setting,
        tui_assets: &TuiAssets,
        is_active: bool,
    ) -> Selector {
        match self {
            SelectorType::Carousel => Selector::Carousel(CarouselSelector::new(
                is_active,
                setting,
                self.carousel_options_for(setting, tui_assets),
            )),
            SelectorType::Color => Selector::Color(ColorSelector::new(is_active, setting)),
            SelectorType::Number => Selector::Number(NumberSelector::new(is_active, setting)),
        }
    }
}

pub(crate) enum Selector {
    Carousel(CarouselSelector),
    Color(ColorSelector),
    Number(NumberSelector),
    // Text(TextInput),
}
