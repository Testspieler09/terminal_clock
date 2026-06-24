use tc_models::{
    helper::{TimeUnit, generate_led_coords_to_base},
    render_mode::RenderMode,
};

use crate::configs::misc::MappingConfig;

fn generate_from_ascii(
    layout: &str,
    mapping: &Option<MappingConfig>,
    unit: TimeUnit,
    render_mode: RenderMode,
    always_on: &[(u32, u32)],
) -> Vec<Vec<(u32, u32)>> {
    let mapping = mapping
        .as_ref()
        .expect("Mapping config required for ASCII clocks");

    let max_value = match unit {
        TimeUnit::Minutes | TimeUnit::Seconds => 60,
        TimeUnit::Hours => 24,
    };

    let mut positions: Vec<(u32, u32)> = Vec::new();

    for (y, line) in layout.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if let Some(mapped_unit) = mapping.symbols.get(&ch) {
                if *mapped_unit == unit {
                    positions.push((y as u32, x as u32));
                }
            }
        }
    }

    if positions.is_empty() {
        panic!("No positions found for {:?}", unit);
    }

    match render_mode {
        RenderMode::Bits => generate_bits_final(positions, max_value, always_on),
        RenderMode::Digits => {
            panic!("Digits mode requires explicit coords")
        }
    }
}

fn generate_bits_final(
    mut positions: Vec<(u32, u32)>,
    max_value: u32,
    always_on: &[(u32, u32)],
) -> Vec<Vec<(u32, u32)>> {
    positions.sort_by_key(|(y, _)| std::cmp::Reverse(*y));

    let weights: Vec<u32> = (0..positions.len()).map(|i| 1 << i).collect();

    let mut result = Vec::with_capacity(max_value as usize);

    for value in 0..max_value {
        let mut coords = Vec::new();

        coords.extend_from_slice(always_on);

        for (i, &(y, x)) in positions.iter().enumerate() {
            if (value & weights[i]) != 0 {
                coords.push((y, x));
            }
        }

        result.push(coords);
    }

    result
}

pub(crate) fn resolve_coords(
    coords: Option<[Vec<(u8, Vec<(u32, u32)>)>; 2]>,
    layout: &str,
    mapping: &Option<MappingConfig>,
    always_on: &[(u32, u32)],
    unit: TimeUnit,
    render_mode: RenderMode,
) -> Vec<Vec<(u32, u32)>> {
    if let Some(coords) = coords {
        generate_led_coords_to_base(&coords[0], &coords[1], always_on, unit, render_mode)
    } else {
        generate_from_ascii(layout, mapping, unit, render_mode, always_on)
    }
}
