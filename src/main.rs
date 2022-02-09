use std::io;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::DOT,
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Terminal,
};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};

use chrono::prelude::*;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let date: DateTime<Local> = Local::now();

            let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();

            let tabs = Tabs::new(titles)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(DOT);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Max(3), Constraint::Min(3)].as_ref())
                .split(f.size());

            let block = Block::default()
                .title(date.format("%Y-%m-%d %H:%M:%S").to_string())
                .borders(Borders::ALL);

            f.render_widget(tabs, chunks[0]);
            f.render_widget(block, chunks[1]);
        })?;
    }
}
