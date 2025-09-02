pub mod default_quotes;

pub(crate) const COLORSCHEMES: [&str; 2] = [
    include_str!("./colorschemes/tokyo_night.toml"),
    include_str!("./colorschemes/terminal_clock.toml"),
];
pub(crate) const CLOCK_FACES: [&str; 1] = [include_str!("./clock_faces/temple.toml")];
