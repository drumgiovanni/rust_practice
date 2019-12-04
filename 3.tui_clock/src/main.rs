#[allow(dead_code)]
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders, Text, Paragraph};
use tui::style::{Style, Modifier, Color};
use tui::layout::{Layout, Constraint, Direction, Alignment};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(f.size());
            let text = [
                    Text::raw("This is a line \n"),
                    Text::styled("This is a line   \n", Style::default().fg(Color::Red)),
                    Text::styled("This is a line\n", Style::default().bg(Color::Blue)),
                    Text::styled(
                        "This is a longer line\n",
                        Style::default().modifier(Modifier::CROSSED_OUT),
                    ),
                    Text::styled(
                        "This is a line\n",
                        Style::default().fg(Color::Green).modifier(Modifier::ITALIC),
                    ),
                ];
            let block = Block::default()
                 .title("Block")
                 .borders(Borders::ALL);
            Paragraph::new(text.iter())
                    .block(block.clone().title("Left, no wrap"))
                    .alignment(Alignment::Left)
                    .render(&mut f, chunks[0]);
            Paragraph::new(text.iter())
                    .block(block.clone().title("Left, wrap"))
                    .alignment(Alignment::Left)
                    .wrap(true)
                    .render(&mut f, chunks[1]);
        })?;
    }
    Ok(())
}
