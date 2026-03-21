use tc_models::clock::TimeFormat;

use crate::tui_models::{tui::TuiAssets, tui_action::TuiAction};

pub trait Selectable {
    fn get_name(&self, tui_assets: &TuiAssets) -> String;
    fn get_corrosponding_action(&self) -> TuiAction;
}

pub enum SelectableItem {
    Theme(u16),
    ClockFace(u16),
    Format(TimeFormat),
    Quote(Option<u16>),
}

impl Selectable for SelectableItem {
    fn get_name(&self, tui_assets: &TuiAssets) -> String {
        match self {
            SelectableItem::Theme(theme_idx) => tui_assets
                .get_color_theme(*theme_idx)
                .get_name()
                .to_string(),
            SelectableItem::ClockFace(clock_idx) => {
                tui_assets.get_clock(*clock_idx).get_name().to_string()
            }
            SelectableItem::Format(fmt) => fmt.get_str_repr().to_string(),
            SelectableItem::Quote(quote_idx) => {
                if let Some(quote) = tui_assets.get_quote(*quote_idx) {
                    quote.text.clone()
                } else {
                    "None".to_string()
                }
            }
        }
    }

    fn get_corrosponding_action(&self) -> TuiAction {
        match self {
            SelectableItem::Theme(new_theme_idx) => {
                TuiAction::UpdateColorTheme(*new_theme_idx)
                // TODO: also update the linked fields here (each color theme field in the
                // color tab)
            }
            SelectableItem::ClockFace(new_clockface_idx) => {
                TuiAction::UpdateClockFace(*new_clockface_idx)
                // TODO: also update the linked fields here (Format field)
                // also the custom color for the clock?!
            }
            SelectableItem::Format(new_format) => TuiAction::UpdateClockFormat(*new_format),
            SelectableItem::Quote(new_quote) => TuiAction::UpdateQuote(*new_quote),
        }
    }
}
