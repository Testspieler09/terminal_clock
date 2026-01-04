#[derive(Clone)]
pub(crate) enum ApplicationState {
    /// The clock is running and displayed for the user
    Running,

    /// The hero menu (TerminalClock title, Settings, Help, Quit) is rendered
    ShowingHero,

    /// The help box is rendered and displayed for the user
    ShowingHelp,

    /// The settings menu is rendered and displayed for the user
    ShowingSettings,

    /// The terminal is too small to render the content
    /// TODO: remove and use the render if fits helper
    TerminalTooSmall,

    /// The program finished successfully
    Finished,
}
