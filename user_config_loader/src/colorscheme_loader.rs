use crate::colorschema::ColorScheme;

struct SchemeConfig;

pub struct ColorSchemeLoader;

impl ColorSchemeLoader {
    pub fn load_colorscheme() -> ColorScheme {
        vec![""].into()
    }

    fn from(config: SchemeConfig) -> ColorScheme {
        todo!()
    }
}
