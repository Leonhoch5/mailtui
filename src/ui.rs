use ratatui::{Frame, widgets::{Block, Borders}, layout::Rect};

pub fn draw(frame: &mut Frame) {
    let size: Rect = frame.size();
    let block = Block::default()
        .title("MailTUI")
        .borders(Borders::ALL);
    frame.render_widget(block, size);
}
