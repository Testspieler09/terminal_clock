use crate::{color_theme::ColorTheme, tui_action::TuiAction};
use std::sync::{Arc, Mutex};

pub trait Selectable {
    fn get_name(&self) -> &str;
    fn get_corrosponding_action(self) -> TuiAction;
}

#[derive(Clone)]
pub enum SelectableItem {
    Theme(Arc<ColorTheme>),
}

impl Selectable for SelectableItem {
    fn get_name(&self) -> &str {
        match self {
            SelectableItem::Theme(item) => item.get_name(),
        }
    }

    fn get_corrosponding_action(self) -> TuiAction {
        match self {
            SelectableItem::Theme(new_theme) => {
                TuiAction::UpdateColorTheme(Arc::new(Mutex::new(new_theme.as_ref().clone())))
            }
        }
    }
}
