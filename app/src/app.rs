use std::io;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{self, Event, KeyCode}};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::ListState;

use crate::ui;

pub fn run() -> Result<(), io::Error> {
    // ask user which provider to use via TUI and attempt login if requested
    match crate::ui::login::prompt_provider() {
        Ok(crate::ui::login::Provider::Google) => {
            let client_id = std::env::var("MAIL_OAUTH_CLIENT_ID").ok();
            let client_secret = std::env::var("MAIL_OAUTH_CLIENT_SECRET").ok();
            if let (Some(id), Some(sec)) = (client_id, client_secret) {
                match crate::auth::oauth_wrapper::oauth_login(&id, &sec) {
                    Ok(token) => println!("OAuth token obtained (length {}), continuing...", token.len()),
                    Err(e) => eprintln!("OAuth login failed: {}", e),
                }
            } else {
                println!("MAIL_OAUTH_CLIENT_ID/MAIL_OAUTH_CLIENT_SECRET not set; set them or choose Skip.");
            }
        }
        Ok(crate::ui::login::Provider::Outlook) => {
            println!("Outlook login is not implemented yet. Skipping.");
        }
        Ok(crate::ui::login::Provider::Skip) | Err(_) => {
            println!("Skipping login");
        }
    }

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

                match key.code {
                    KeyCode::Enter => {
                        let sel = list_state.selected().map(|i| i/2).unwrap_or(0);
                        if let Some(mail) = ui::get_message(sel) {
                            // fullscreen view loop
                            loop {
                                terminal.draw(|f| ui::render_message_fullscreen(f, &mail))?;
                                if event::poll(std::time::Duration::from_millis(100))? {
                                    if let Event::Key(k) = event::read()? {
                                        match k.code {
                                            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => break,
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    other => ui::handle_key(&mut list_state, other, ui::message_count()),
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
