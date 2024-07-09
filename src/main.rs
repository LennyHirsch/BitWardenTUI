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
    app::{Account, App},
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
    let res = run_app(&mut terminal, &mut app);

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    app.fetch_items(vec![
        Account {
            id: "123".to_string(),
            name: "Facebook".to_string(),
            user: "lenno".to_string(),
            pass: "hunter2".to_string(),
        },
        Account {
            id: "456".to_string(),
            name: "Twitter".to_string(),
            user: "tweeto".to_string(),
            pass: "birb".to_string(),
        },
        Account {
            id: "789".to_string(),
            name: "Instagram".to_string(),
            user: "Iguess".to_string(),
            pass: "vain-dood".to_string(),
        },
        Account {
            id: "12354".to_string(),
            name: "Telegram".to_string(),
            user: "lenno.hirsch".to_string(),
            pass: "hellothere".to_string(),
        },
    ]);

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
                    if app.selected < app.items.len() - 1 {
                        app.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }
}
