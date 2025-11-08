use std::io;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{self, Event, KeyCode}};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::ListState;

use crate::ui;

pub fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = ListState::default();

    loop {
        terminal.draw(|f| ui::draw(f, &mut list_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }

                ui::handle_key(&mut list_state, key.code, ui::message_count());
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
