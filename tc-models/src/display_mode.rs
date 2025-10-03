use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum DisplayMode {
    Binary,
    Decimal,
}
