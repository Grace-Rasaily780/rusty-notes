use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::{
    io::{stdout},
    path::PathBuf,
    time::Duration,
};
use walkdir::WalkDir;

pub fn list_notes(base: &str) -> Vec<String> {
    WalkDir::new(base)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "note").unwrap_or(false))
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect()
}

pub fn interactive_list(notes: Vec<PathBuf>) -> anyhow::Result<Option<PathBuf>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let items: Vec<ListItem> = notes
                .iter()
                .map(|p| {
                    let name = p.file_name().unwrap().to_string_lossy().to_string();
                    ListItem::new(name)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Notes"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut state);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        if let Some(i) = state.selected() {
                            if i < notes.len() - 1 {
                                state.select(Some(i + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(i) = state.selected() {
                            if i > 0 {
                                state.select(Some(i - 1));
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(i) = state.selected() {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;
                            return Ok(Some(notes[i].clone()));
                        }
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        terminal.show_cursor()?;
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
