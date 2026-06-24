use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalogClockConfig {
    // name: String,
    //
    // hour_hand_frames: Vec<&'static str>,
    // minute_hand_frames: Vec<&'static str>,
    // second_hand_frames: Vec<&'static str>,
    //
    // clock_base: &'static str,
    clock_center: [u32; 2],
    hour_center: [u32; 2],
    minute_center: [u32; 2],
    seconds_center: [u32; 2],
}

// impl AnalogClockConfig {
//     pub fn set_name_if_not_already_set(&mut self, name: String) {
//         self.name = name
//     }
// }
