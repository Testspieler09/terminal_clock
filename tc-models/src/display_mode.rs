use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum DisplayMode {
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "decimal")]
    Decimal,
}
