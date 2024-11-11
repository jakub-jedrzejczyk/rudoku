use std::io::{self, Write};
use crossterm::{terminal, QueueableCommand, cursor, style::{self, Stylize}, ExecutableCommand};

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    stdout.flush()?;
    Ok(())
}

pub fn size() -> (u16, u16) {
    let size = terminal::size();
    
    match size {
        Ok(v) => return (v.0, v.1),
        Err(_e) => return (u16::MAX, u16::MAX),
    }
}
