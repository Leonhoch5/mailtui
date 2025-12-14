use std::io;
use crossterm::{event::{self, Event, KeyCode}, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem}, layout::{Layout, Constraint, Direction}, style::{Style, Color, Modifier}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Google,
    Outlook,
    Skip,
}

pub fn prompt_provider() -> Result<Provider, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = vec!["Google", "Outlook (not implemented)", "Skip"];
    let mut selected: usize = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let title = Block::default().title("Choose login provider").borders(Borders::ALL);
            f.render_widget(title, chunks[0]);

            let list_items: Vec<ListItem> = items.iter().map(|s| ListItem::new(*s)).collect();
            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol("▶ ");

            let mut state = ratatui::widgets::ListState::default();
            state.select(Some(selected));
            f.render_stateful_widget(list, chunks[1], &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if selected == 0 {
                            selected = items.len() - 1;
                        } else {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % items.len();
                    }
                    KeyCode::Enter => break,
                    KeyCode::Char('q') | KeyCode::Esc => { selected = 2; break; }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    let provider = match selected {
        0 => Provider::Google,
        1 => Provider::Outlook,
        _ => Provider::Skip,
    };
    Ok(provider)
}
