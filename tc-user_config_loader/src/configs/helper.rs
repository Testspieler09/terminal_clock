use tc_models::{
    helper::{TimeUnit, generate_led_coords_to_base},
    render_mode::RenderMode,
};

use crate::{AssetsLoadError, configs::misc::MappingConfig};

type LedMapEntry = (u8, Vec<(u32, u32)>);
type LedMap = Vec<LedMapEntry>;

fn max_bits(unit: TimeUnit, is_tens: bool) -> usize {
    match (unit, is_tens) {
        (TimeUnit::Hours, true) => 2, // tens digit: 0–2
        (_, true) => 3,               // minutes/seconds tens: 0–5
        (_, false) => 4,              // units digit: 0–9
    }
}

pub(crate) fn generate_from_ascii(
    layout: &str,
    mapping: &MappingConfig,
    unit: TimeUnit,
    render_mode: RenderMode,
    always_on: &[(u32, u32)],
) -> Result<Vec<Vec<(u32, u32)>>, AssetsLoadError> {
    let mut units_positions: Vec<(u32, u32)> = Vec::new();
    let mut tens_positions: Vec<(u32, u32)> = Vec::new();

    for (y, line) in layout.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if let Some(role) = mapping.symbols.get(&ch)
                && role.time_unit() == unit
            {
                let pos = (y as u32, x as u32);
                if role.is_tens() {
                    tens_positions.push(pos);
                } else {
                    units_positions.push(pos);
                }
            }
        }
    }

    if units_positions.is_empty() {
        return Err(AssetsLoadError::InvalidConfig(format!(
            "No units positions found for {unit:?} in ASCII layout"
        )));
    }

    match render_mode {
        RenderMode::Bits => {
            let tens_map =
                positions_to_bit_ledmap(tens_positions, max_bits(unit, true), unit, true)?;
            let units_map =
                positions_to_bit_ledmap(units_positions, max_bits(unit, false), unit, false)?;
            Ok(generate_led_coords_to_base(
                &tens_map,
                &units_map,
                always_on,
                unit,
                render_mode,
            ))
        }
        RenderMode::Digits => Err(AssetsLoadError::InvalidConfig(
            "Digits mode requires explicit coords".into(),
        )),
        RenderMode::Gauge => Err(AssetsLoadError::InvalidConfig(
            "Gauge mode requires explicit coords".into(),
        )),
    }
}

fn positions_to_bit_ledmap(
    mut positions: Vec<(u32, u32)>,
    max: usize,
    unit: TimeUnit,
    is_tens: bool,
) -> Result<LedMap, AssetsLoadError> {
    if positions.len() > max {
        let which = if is_tens { "tens" } else { "units" };
        return Err(AssetsLoadError::InvalidConfig(format!(
            "Too many {which} positions for {unit:?} ({} given, maximum is {max})",
            positions.len()
        )));
    }
    // Sort bottom-to-top (highest row index = lowest bit weight)
    positions.sort_by_key(|(y, _)| std::cmp::Reverse(*y));
    Ok(positions
        .into_iter()
        .enumerate()
        .map(|(i, pos)| (1u8 << i, vec![pos]))
        .collect())
}
