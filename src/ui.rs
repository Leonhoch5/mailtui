use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Color, Modifier},
};

pub fn sample_messages() -> Vec<(&'static str, &'static str, bool, &'static str)> {
    // sample stuff - change to real array later
    vec![
        ("Alice", "Meeting tomorrow", false, "09:12"),
        ("Bob", "Rust project update", true, "13:45"),
        ("Charlie", "Flight booking", false, "Yesterday"),
    ]
}

pub fn message_count() -> usize {
    sample_messages().len()
}

pub fn draw(frame: &mut Frame, state: &mut ListState) {
    let size = frame.size();

    let raw = sample_messages();

    let preferred_bar_col: usize = 25;
    let term_width = size.width as usize;
    let bar_col = if term_width > preferred_bar_col + 10 {
        preferred_bar_col
    } else if term_width > 30 {
        term_width.saturating_sub(20)
    } else {
        term_width / 2
    };

    let status_col_width: usize = 8;
    let min_subject_width: usize = 10;
    let subject_col_width: usize = if term_width > bar_col + status_col_width + min_subject_width + 6 {
        (term_width - bar_col - status_col_width - 6).min(40)
    } else {
        min_subject_width
    };

    let mut items: Vec<ListItem> = Vec::new();

    for (from, subject, read, sent) in &raw {
        let dot = if *read { "○" } else { "●" };
        let mut left = format!("{} From: {}", dot, from);

        if left.chars().count() >= bar_col {
            left = left.chars().take(bar_col.saturating_sub(1)).collect();
            left.push('…');
        } else {
            let pad = bar_col.saturating_sub(left.chars().count());
            left.push_str(&" ".repeat(pad));
        }

        let mut subj = subject.to_string();
        if subj.chars().count() >= subject_col_width {
            subj = subj.chars().take(subject_col_width.saturating_sub(1)).collect();
            subj.push('…');
        } else {
            let pad = subject_col_width.saturating_sub(subj.chars().count());
            subj.push_str(&" ".repeat(pad));
        }

        let status = if *read { "Read" } else { "Unread" };
        let mut status_field = status.to_string();
        if status_field.chars().count() >= status_col_width {
            status_field = status_field.chars().take(status_col_width.saturating_sub(1)).collect();
            status_field.push('…');
        } else {
            let pad = status_col_width.saturating_sub(status_field.chars().count());
            status_field.push_str(&" ".repeat(pad));
        }

        let line = format!("{} | {} | {} | {}", left, subj, status_field, sent);
        items.push(ListItem::new(line));

        let sep = if term_width > 0 {
            "─".repeat(term_width)
        } else {
            String::from("─")
        };
        items.push(ListItem::new(sep));
    }

    // selection: default to first
    if state.selected().is_none() && !raw.is_empty() {
        state.select(Some(0));
    }

    let list = List::new(items)
        .block(Block::default().title("Inbox").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("");

    frame.render_stateful_widget(list, size, state);
// move selection to next message (each message uses 2 ListItems)
pub fn select_next(state: &mut ListState, msg_count: usize) {
    if msg_count == 0 {
        state.select(None);
        return;
    }
    let current_msg_idx = state.selected().map(|i| i / 2).unwrap_or(0);
    let next = if current_msg_idx + 1 >= msg_count {
        current_msg_idx
    } else {
        current_msg_idx + 1
    };
    state.select(Some(next * 2));
}

pub fn select_prev(state: &mut ListState, msg_count: usize) {
    if msg_count == 0 {
        state.select(None);
        return;
    }
    let current_msg_idx = state.selected().map(|i| i / 2).unwrap_or(0);
    let prev = if current_msg_idx == 0 { 0 } else { current_msg_idx - 1 };
    state.select(Some(prev * 2));
}

// handle arrow keys; call from your event loop with the KeyCode and number of messages
pub fn handle_key(state: &mut ListState, key: KeyCode, msg_count: usize) {
    match key {
        KeyCode::Up => select_prev(state, msg_count),
        KeyCode::Down => select_next(state, msg_count),
        _ => {}
    }
}
