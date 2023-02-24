use std::io;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

mod app;
mod event;
mod ui;

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::default();
    run_app(&mut terminal, &mut app).unwrap();

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &mut App) -> io::Result<()> {
    app.selected_state.set_current(0);
    loop {
        terminal.draw(|f| ui(f, &mut app)).unwrap();

        if let Ok(Event::Key(key)) = crossterm::event::read() {
            let result = event::main(&mut app, key);
            match result {
                Some(r) => return r,
                None => (),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    ui::main(f, app);
}
