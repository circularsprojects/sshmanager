use std::io::{self, stdout};
use std::process::Command;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::{Modifier, Style},
    widgets::{Block, List, ListItem, ListState}, Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
        ListItem::new("Item 4"),
        ListItem::new("Item 5"),
    ];
    let mut state = ListState::default();
    state.select(Some(0));

    let mut ssh: &str = "";

    let list = List::new(items)
        .block(Block::default().title("Select Item"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let mut should_quit = false;
    while !should_quit {
        let _ = terminal.draw(|f| f.render_stateful_widget(&list, f.area(), &mut state));
        should_quit = handle_events(&mut state, &mut ssh)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    if ssh != "" {
        let mut command = Command::new("ssh");
        command.arg(ssh);
        terminal.show_cursor()?;
        let mut child = command.spawn()?;
        let status = child.wait()?;
        match status.code() {
            Some(code) => println!("Command finished with exit code {}", code),
            None => println!("Command did not terminate normally"),
        }
    }
    Ok(())
}

fn handle_events(state: &mut ListState, ssh: &mut &str) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    state.select(Some(state.selected().unwrap_or(0) + 1));
                }
                KeyCode::Up => {
                    state.select(Some(state.selected().unwrap_or(0).saturating_sub(1)));
                }
                KeyCode::Enter => {
                    *ssh = "root@vps";
                    return Ok(true);
                }
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                _ => {}
            }
            // if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
            //     return Ok(true);
            // }
        }
    }
    Ok(false)
}
