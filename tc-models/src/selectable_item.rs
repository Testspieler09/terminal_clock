use crate::{color_theme::ColorTheme, tui_action::TuiAction};
use std::sync::Arc;

pub trait Selectable {
    fn get_name(&self) -> &str;
    fn get_corrosponding_action(&self) -> TuiAction;
}

pub enum SelectableItem {
    Theme(Arc<ColorTheme>),
}

impl Selectable for SelectableItem {
    fn get_name(&self) -> &str {
        match self {
            SelectableItem::Theme(item) => item.get_name(),
        }
    }

    fn get_corrosponding_action(&self) -> TuiAction {
        match self {
            SelectableItem::Theme(item) => TuiAction::UpdateColorTheme(item.clone()),
        }
    }
}
