use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "BitWarden TUI",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut list_items = Vec::<ListItem>::new();

    for (i, account) in app.accounts.clone().into_iter().enumerate() {
        let mut color = Color::Yellow;
        if i == app.selected {
            color = Color::Red;
        }

        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", account.name),
            Style::default().fg(color),
        ))));
    }

    let list = List::new(list_items).block(list_block);
    let mut state = ListState::default();
    state.select(app.selected.into());

    f.render_stateful_widget(list, list_chunks[0], &mut state);

    let account_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut account_items = Vec::<ListItem>::new();

    if let Some(acc) = &app.active_account {
        // TODO: FIX CLONES!
        account_items.push(ListItem::new(Line::from(Span::styled(
            format!("Name: {}", acc.name.clone()),
            Style::default().fg(Color::Yellow),
        ))));
        if let Some(user) = &acc.user {
            account_items.push(ListItem::new(Line::from(Span::styled(
                format!("Username: {}", user.clone()),
                Style::default().fg(Color::Yellow),
            ))));
        }
        if let Some(pass) = &acc.pass {
            account_items.push(ListItem::new(Line::from(Span::styled(
                format!("Password: {}", pass.clone()),
                Style::default().fg(Color::Yellow),
            ))));
        }
    } else {
        account_items.push(ListItem::new(Line::from(Span::styled(
            format!("No active account to display"),
            Style::default().fg(Color::Red),
        ))));
    }

    let account = List::new(account_items).block(account_block);

    f.render_widget(account, list_chunks[1]);
}
