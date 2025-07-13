use ratatui::layout::{Constraint, Flex, Layout, Rect};
use std::{collections::HashMap, u32};

pub fn center_widget(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

// pub fn generate_led_coords_from_hash_map<'a>(
//     led_map: HashMap<u32, &'a [(u32, u32)]>,
//     always_on: &'a [(u32, u32)],
//     intervall: u32,
// ) -> Result<&'a [&'a [(u32, u32)]], &'static str> {
//     if ![12, 24, 60].contains(&intervall) {
//         return Err("The interval can only be 12, 24 or 60.");
//     }
//
//     let mut results: Vec<&'a [(u32, u32)]> = Vec::new();
//
//     for i in 1..intervall {
//         let mut combined: Vec<(u32, u32)> = Vec::new();
//         let mut bit = 1;
//
//         while bit <= i {
//             if i & bit != 0 {
//                 if let Some(values) = led_map.get(&bit) {
//                     combined.extend_from_slice(values);
//                 }
//             }
//             bit <<= 1;
//         }
//
//         results.push(Box::leak(combined.into_boxed_slice()));
//     }
//
//     Ok(results)
// }
