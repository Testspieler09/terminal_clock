#[derive(Clone)]
pub enum ApplicationState {
    /// The clock is running and displayed for the user
    RunningClock,

    /// Run the pomodoro timer. Using the clock for display
    RunningPomodoro,

    /// Run the timer in timer mode. Using the clock for display
    RunningTimer,

    /// The hero menu (TerminalClock title, Settings, Help, Quit) is rendered
    ShowingHero,

    /// The help box is rendered and displayed for the user
    ShowingHelp,

    /// The settings menu is rendered and displayed for the user
    ShowingSettings,

    /// Fireworks Easteregg
    ShowingFirework,

    /// The program finished successfully
    Finished,
}
