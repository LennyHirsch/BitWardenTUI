use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cursive::event::Key;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        },
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{error::Error, io};
mod app;
mod ui;
use crate::{
    app::{list_accounts, parse_items, Account, App, CurrentScreen},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Login if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Backspace => {
                        app.pass_input.pop();
                        app.clean_input.pop();
                    }
                    KeyCode::Char(value) => {
                        app.pass_input.push(value);
                        app.clean_input.push('•');
                    }
                    KeyCode::Enter => {
                        app.unlock();
                        app.fetch_items(parse_items(list_accounts()));
                    }
                    _ => {}
                },
                CurrentScreen::Main if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        if app.pass_copied {
                            app.clear_clipboard();
                        }
                        return Ok(true);
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        if app.selected < app.accounts.len() - 1 {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        app.update_active_account(app.selected);
                    }
                    KeyCode::Backspace => {
                        app.active_account = None;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.copy_pass();
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        app.clear_clipboard();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
