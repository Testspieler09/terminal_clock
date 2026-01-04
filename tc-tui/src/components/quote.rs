use crate::components::Dimensions;
use tc_models::quote::Quote;
use unicode_segmentation::UnicodeSegmentation;

impl Dimensions for &Quote {
    fn width(&self) -> u16 {
        let final_text = self.final_quote_string();

        final_text.graphemes(true).count() as u16
    }

    fn height(&self) -> u16 {
        1u16
    }
}
