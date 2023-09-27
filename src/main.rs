// reference: https://ratatui.rs/concepts/application-patterns/component-architecture.html
// https://github.com/a-kenji/tui-term

use std::io;
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// custom imports
use ratatui_sandbox::my_app;

fn main() -> ()
{
    let mut terminal = setup_terminal();

    let mut app = my_app::App::new();
    app.run(&mut terminal);
    restore_terminal(&mut terminal);
}

fn setup_terminal() -> Terminal<CrosstermBackend<io::Stdout>>
{
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    Terminal::new(CrosstermBackend::new(stdout)).unwrap()
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
{
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
}
