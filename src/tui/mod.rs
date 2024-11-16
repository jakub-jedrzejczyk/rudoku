use std::io::{self, Write};
use crossterm::{cursor, execute, style::{self, Stylize}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand};

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

pub fn write(mut stdout: &io::Stdout, c: char) -> io::Result<()> {
    stdout.queue(style::Print(c))?;
    Ok(())
}

pub fn draw_vertical_line(mut stdout: &io::Stdout, c: char, x: u16, y: u16, len: u16) -> io::Result<()> {
    for n in y..(y + len) {
        stdout
            .queue(cursor::MoveTo(x, n))?;
        write(stdout, c);
    }

    Ok(())
}

pub fn flush(mut stdout: &io::Stdout) -> io::Result<()> {
    stdout.flush()?;
    Ok(())
}

pub fn setup(mut stdout: &io::Stdout) -> io::Result<()> {
    stdout
        .queue(cursor::Hide)?
        .queue(EnterAlternateScreen)?;
    flush(stdout);
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn cleanup(mut stdout: &io::Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    stdout
        .queue(LeaveAlternateScreen)?
        .queue(cursor::Show)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?;
    flush(stdout);
    Ok(())
}

pub fn draw_board() {

}
