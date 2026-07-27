use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AnalogClockConfig {
    clock_center: [u32; 2],
    hour_center: [u32; 2],
    minute_center: [u32; 2],
    seconds_center: [u32; 2],
}
