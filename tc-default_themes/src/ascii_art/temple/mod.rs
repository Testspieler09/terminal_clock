use crate::Clock;
use ratatui::style::Color;
use tc_user_config_loader::clock_types::color_clock::ColorClock;
use tc_user_config_loader::clock_types::helper::{generate_binary_led_coords, TimeUnit};

pub(crate) fn temple() -> Box<dyn Clock> {
    let hour: String = include_str!("H_temple.ascii").to_string();
    let minutes: String = include_str!("MS_temple.ascii").to_string();
    let seconds = minutes.clone();

    let led_bit_mapping_hour_face: [&[(u8, (u32, u32))]; 2] = [
        &[(1, (11, 1)), (2, (8, 0))],
        &[
            (1, (11, 19)),
            (2, (9, 19)),
            (4, (7, 19)),
            (8, (5, 19)),
            (8, (5, 8)),
        ],
    ];

    let led_bit_mapping_minute_face: [&[(u8, (u32, u32))]; 2] = [
        &[(1, (11, 1)), (2, (9, 1)), (4, (7, 0))],
        &[(1, (11, 19)), (2, (9, 19)), (4, (7, 19)), (8, (5, 19))],
    ];

    let led_coords_hours = generate_binary_led_coords(
        led_bit_mapping_hour_face[0],
        led_bit_mapping_hour_face[1],
        &[(0, 14)],
        TimeUnit::Hours,
    );
    let led_coords_minutes = generate_binary_led_coords(
        led_bit_mapping_minute_face[0],
        led_bit_mapping_minute_face[1],
        &[(0, 14)],
        TimeUnit::Minutes,
    );

    Box::new(ColorClock::new(
        hour,
        minutes,
        seconds,
        led_coords_hours,
        led_coords_minutes.clone(),
        led_coords_minutes,
        Color::Blue,
    ))
}
