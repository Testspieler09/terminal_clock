use crate::tui_models::tui_action::TuiAction;
use tc_models::clock::TimeFormat;

pub trait Selectable {
    fn get_name(&self) -> &str;
    fn get_corrosponding_action(&self) -> TuiAction;
}

pub enum SelectableItem {
    Theme(u16),
    ClockFace(u16),
    Format(TimeFormat),
    Quote(Option<u16>),
}

impl Selectable for SelectableItem {
    fn get_name(&self) -> &str {
        // TODO: get the thing via the controller or assets by index
        match self {
            SelectableItem::Theme(theme_idx) => "Theme name",
            SelectableItem::ClockFace(clock_idx) => "Clock name",
            SelectableItem::Format(fmt) => fmt.get_str_repr(),
            SelectableItem::Quote(quote_idx) => {
                if let Some(idx) = quote_idx {
                    "Quote name"
                } else {
                    "None"
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
