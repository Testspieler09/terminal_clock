mod clock_view;
mod fireworks_view;

use ratatui::DefaultTerminal;
use tc_models::clock::TimeFormat;

use crate::Result;

pub trait DebugView {
    fn name(&self) -> &'static str;
    fn run(&self, terminal: &mut DefaultTerminal) -> Result<()>;
}

pub struct ClockViewConfig {
    pub clock_face: Option<String>,
    pub theme: Option<String>,
    pub format: Option<TimeFormat>,
    pub quote: Option<String>,
}

pub struct FireworksViewConfig {
    pub theme: Option<String>,
}

pub fn all_views(
    clock_cfg: ClockViewConfig,
    fireworks_cfg: FireworksViewConfig,
) -> Vec<Box<dyn DebugView>> {
    vec![
        Box::new(fireworks_view::FireworksView(fireworks_cfg)),
        Box::new(clock_view::ClockView(clock_cfg)),
    ]
}
