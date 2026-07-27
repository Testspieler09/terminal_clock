use tc_models::{
    helper::{TimeUnit, generate_led_coords_to_base},
    render_mode::RenderMode,
};

use crate::{AssetsLoadError, configs::misc::MappingConfig};

type LedMapEntry = (u8, Vec<(u32, u32)>);
type LedMap = Vec<LedMapEntry>;

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
            let tens_map = positions_to_bit_ledmap(tens_positions)?;
            let units_map = positions_to_bit_ledmap(units_positions)?;
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

fn positions_to_bit_ledmap(mut positions: Vec<(u32, u32)>) -> Result<LedMap, AssetsLoadError> {
    if positions.len() > 7 {
        return Err(AssetsLoadError::InvalidConfig(format!(
            "Too many marker positions ({}) for bits mode — maximum is 7",
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
