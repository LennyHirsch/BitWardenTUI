use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    // Dividing screen for basic layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(3),    // Main section
            Constraint::Length(3), // Hints
        ])
        .split(f.size());

    // Title
    let title_block = Block::default()
        .title("Bitwarden TUI")
        .borders(Borders::ALL)
        .style(Style::default());

    // let title = Paragraph::new(Text::styled(
    //     "BitWarden TUI",
    //     Style::default().fg(Color::Green),
    // ))
    // .block(title_block);

    f.render_widget(title_block, chunks[0]);

    // Split middle section into two columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // List of accounts
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

    f.render_stateful_widget(list, main_chunks[0], &mut state);

    // Account details
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

    f.render_widget(account, main_chunks[1]);

    // Hints
    let hint_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let hint_text = Paragraph::new(Text::styled(
        "[Up], [Down] or [K], [J]: Navigate / [Enter]: Select / [Backspace]: Clear / [C]: Copy password / [X]: Clear clipboard / [Q]: Quit",
        Style::default().fg(Color::Green),
    ))
    .block(hint_block);

    f.render_widget(hint_text, chunks[2]);

    if let CurrentScreen::Login = app.current_screen {
        f.render_widget(Clear, f.size());
        let area = centered_rect(20, 20, f.size());

        let login_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let pass_block = Block::default().title("Password:").borders(Borders::ALL);
        let pass_text = Paragraph::new(app.clean_input.clone()).block(pass_block);

        f.render_widget(pass_text, login_chunk[0]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
