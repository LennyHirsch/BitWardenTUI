use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{error::Error, io};
mod app;
mod ui;
use crate::{
    app::{get_account, list_accounts, parse_items, unlock, Account, App},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    unlock();
    let accounts = parse_items(list_accounts());

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app, accounts);

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    accounts: Vec<Account>,
) -> io::Result<bool> {
    app.fetch_items(accounts);

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected < app.accounts.len() - 1 {
                        app.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    app.update_active_account(app.selected);
                }
                _ => {}
            }
        }
    }
}
