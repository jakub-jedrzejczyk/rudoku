use std::io::{self, Write};
use crossterm::{execute, terminal, QueueableCommand, cursor, style::{self, Stylize}, ExecutableCommand};

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    stdout.flush()?;
    return Ok(());
}

pub fn size() -> (u16, u16) {
    let size = terminal::size();
    
    return match size {
        Ok(v) => (v.0, v.1),
        Err(_e) => (u16::MAX, u16::MAX),
    };
}

pub fn set_size(width: u16, height: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::SetSize(width, height))?;
    Ok(())
}

pub fn write(str: String) {
    let mut stdout = io::stdout();
    write!(stdout, "{str}");
}
