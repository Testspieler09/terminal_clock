use clap::Parser;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState},
};
use tc_models::clock::TimeFormat;
use tc_tui::{TuiAssets, debug_views};

/// Preview TUI views and animations without running the full clock.
#[derive(Parser)]
#[command(name = "debug-view", version)]
struct Args {
    /// Launch a specific view directly by index (skips the menu)
    #[arg(short, long)]
    view: Option<usize>,

    /// Clock face name to use in the clock view (e.g. "seven_segment_led")
    #[arg(long)]
    clock_face: Option<String>,

    /// Color theme name to use in the clock view (e.g. "tokyo_night")
    #[arg(long)]
    theme: Option<String>,

    /// Time format: HH:MM:SS, HH:MM, or MM:HH:SS
    #[arg(long)]
    format: Option<TimeFormat>,

    /// Quote text to display, or "none" to disable
    #[arg(long)]
    quote: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let assets = TuiAssets::try_new(tc_user_config_loader::get_user_config_path()?)?;

    let clock_cfg = debug_views::ClockViewConfig {
        clock_face: args.clock_face,
        theme: args.theme,
        format: args.format,
        quote: args.quote,
    };
    let views = debug_views::all_views(clock_cfg);

    let mut terminal = ratatui::init();
    let result = match args.view {
        Some(idx) => launch_view(&mut terminal, &views, &assets, idx),
        None => run_menu(&mut terminal, &views, &assets),
    };
    ratatui::restore();
    result
}

fn run_menu(
    terminal: &mut DefaultTerminal,
    views: &[Box<dyn debug_views::DebugView>],
    assets: &TuiAssets,
) -> Result<()> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|frame| render_menu(frame, views, &mut list_state))?;

        use ratatui::crossterm::event::{self, Event, KeyCode};
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => {
                    let i = list_state.selected().map_or(0, |i| (i + 1) % views.len());
                    list_state.select(Some(i));
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    let i = list_state
                        .selected()
                        .map_or(0, |i| if i == 0 { views.len() - 1 } else { i - 1 });
                    list_state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(idx) = list_state.selected() {
                        launch_view(terminal, views, assets, idx)?;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render_menu(
    frame: &mut Frame,
    views: &[Box<dyn debug_views::DebugView>],
    list_state: &mut ListState,
) {
    let items: Vec<ListItem> = views
        .iter()
        .map(|v| ListItem::new(Line::from(v.name())))
        .collect();

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" debug-view  [up/down j/k] [Enter to launch] [q to quit] "),
        )
        .highlight_style(Style::new().reversed());

    frame.render_stateful_widget(list, frame.area(), list_state);
}

fn launch_view(
    terminal: &mut DefaultTerminal,
    views: &[Box<dyn debug_views::DebugView>],
    assets: &TuiAssets,
    idx: usize,
) -> Result<()> {
    match views.get(idx) {
        Some(view) => view.run(terminal, assets),
        None => {
            eprintln!("FAILED  unknown view index {idx}");
            Ok(())
        }
    }
}
