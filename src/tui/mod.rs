use std::io::{self, Write};
use crossterm::{cursor, style, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand};

use crate::logic::{self, settings::Settings};

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

pub fn flush(mut stdout: &io::Stdout) -> io::Result<()> {
    stdout.flush()?;
    Ok(())
}

pub fn setup(mut stdout: &io::Stdout) -> io::Result<()> {
    stdout
        .queue(cursor::Hide)?
        .queue(EnterAlternateScreen)?;
    flush(stdout)?;
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
    flush(stdout)?;
    Ok(())
}

fn calculate_canvas_size(x: u16, y: u16, settings: &Settings) -> (u16, u16) {
    let ibx = (x - 1) * settings.get_inner_border_vertical();
    let obx = 2 * settings.get_outer_border_vertical();
    let iby = (y - 1) * settings.get_inner_border_horizontal();
    let oby = 2 * settings.get_outer_border_horizontal();

    let size_x = (x * settings.get_field_width()) + ibx + obx;
    let size_y = (y * settings.get_field_height()) + iby + oby;

    return (size_x, size_y);
}

fn calculate_canvas_offset(x: u16, y: u16, settings: &Settings) -> (u16, u16) {
    if !settings.get_center_board() {
        return (0, 0);
    }

    let (termial_x, terminal_y) = size();
    let (canvas_x, canvas_y) = calculate_canvas_size(x, y, settings);

    let offset_x = (termial_x - canvas_x) / 2;
    let offset_y = (terminal_y - canvas_y) / 2;

    return (offset_x, offset_y);
}

pub fn draw_vertical(mut stdout: &io::Stdout, x_pos: u16, y_pos: u16, length: u16, c: char) -> io::Result<()> {
    for i in 0..length {
        stdout
            .queue(cursor::MoveTo(x_pos + i, y_pos))?;
        write(stdout, c)?;
    }

    Ok(())
}

pub fn draw_horizontal(mut stdout: &io::Stdout, x_pos: u16, y_pos: u16, length: u16, c: char) -> io::Result<()> {
    for i in 0..length {
        stdout
            .queue(cursor::MoveTo(x_pos, y_pos + i))?;
        write(stdout, c)?;
    }

    Ok(())
}

pub fn draw_outer_bound_vertical(stdout: &io::Stdout, settings: &Settings, x: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
    for i in 0..settings.get_outer_border_vertical() {
        let mut x_pos: u16 = 0;
        for _k in 0..x {
            draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_outer_border_horizontal(), settings.get_outer_intersection_char()).unwrap();
            x_pos += settings.get_outer_border_horizontal();
            draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_field_width(), settings.get_outer_horizontal_char()).unwrap();
            x_pos += settings.get_field_width();
        }
        draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_outer_border_horizontal(), settings.get_outer_intersection_char()).unwrap();
    }

    Ok(())
}

pub fn draw_outer_bound_horizontal(stdout: &io::Stdout, settings: &Settings, y: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
    for i in 0..settings.get_outer_border_horizontal() {
        let mut y_pos: u16 = 0;
        for _k in 0..y {
            draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_outer_border_vertical(), settings.get_outer_intersection_char()).unwrap();
            y_pos += settings.get_outer_border_vertical();
            draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_outer_vertical_char()).unwrap();
            y_pos += settings.get_field_height();
        }
        draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_outer_border_vertical(), settings.get_outer_intersection_char()).unwrap();
    }

    Ok(())
}

pub fn draw_outer_bound(stdout: &io::Stdout, settings: &Settings, x: u16, y: u16) -> io::Result<()> {
    let (x_canvas_offset, y_canvas_offset) = calculate_canvas_offset(x, y, settings);
    draw_outer_bound_vertical(&stdout, settings, x, x_canvas_offset, y_canvas_offset).unwrap();
    draw_outer_bound_horizontal(&stdout, settings, y, x_canvas_offset, y_canvas_offset).unwrap();
    let y_offset = settings.get_outer_border_horizontal() + x * settings.get_field_height() + (x - 1) * settings.get_inner_border_horizontal();
    let x_offset = settings.get_outer_border_vertical() + y * settings.get_field_width() + (y - 1) * settings.get_inner_border_vertical();
    draw_outer_bound_vertical(&stdout, settings, x, x_canvas_offset, y_canvas_offset + y_offset).unwrap();
    draw_outer_bound_horizontal(&stdout, settings, y, x_canvas_offset + x_offset, y_canvas_offset).unwrap();
    Ok(())
}

pub fn draw_inner_bound_vertical(stdout: &io::Stdout, settings: &Settings, x: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
    for i in 0..settings.get_inner_border_vertical() {
        let mut x_pos: u16 = 0;
        for _k in 0..(x - 1) {
            draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_field_width(), settings.get_inner_horizontal_char()).unwrap();
            x_pos += settings.get_field_width();
            draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_inner_border_horizontal(), settings.get_inner_intersection_char()).unwrap();
            x_pos += settings.get_inner_border_vertical();
        }
        draw_vertical(&stdout, x_pos + x_offset, i + y_offset, settings.get_field_width(), settings.get_inner_horizontal_char()).unwrap();
    }

    Ok(())
}

pub fn draw_inner_bound_horizontal(stdout: &io::Stdout, settings: &Settings, y: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
    for i in 0..settings.get_inner_border_horizontal() {
        let mut y_pos: u16 = 0;
        for _k in 0..(y - 1) {
            draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_inner_vertical_char()).unwrap();
            y_pos += settings.get_field_height();
            draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_inner_border_vertical(), settings.get_inner_intersection_char()).unwrap();
            y_pos += settings.get_inner_border_horizontal();
        }
        draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_inner_vertical_char()).unwrap();
    }

    Ok(())
}

pub fn draw_inner_bound(stdout: &io::Stdout, settings: &Settings, x: u16, y: u16) -> io::Result<()> {
    let (x_canvas_offset, y_canvas_offset) = calculate_canvas_offset(x, y, settings);
    let x_offset = x_canvas_offset + settings.get_outer_border_vertical();
    let y_offset = y_canvas_offset + settings.get_outer_border_horizontal();
    
    let mut x_mod = settings.get_field_width();
    let mut y_mod = settings.get_field_height();
    for i in 0..(x - 1) {
        draw_inner_bound_vertical(&stdout, settings, x, x_offset, y_offset + y_mod).unwrap();
        y_mod += settings.get_field_height() + settings.get_inner_border_horizontal();
    }
    for i in 0..(y - 1) {
        draw_inner_bound_horizontal(&stdout, settings, y, x_offset + x_mod, y_offset).unwrap();
        x_mod += settings.get_field_width() + settings.get_inner_border_vertical();
    }

    Ok(())
}

pub fn draw_board(stdout: &io::Stdout, game: &logic::game::Game) {
    let x = game.get_x();
    let y = game.get_y();
    let settings = game.get_settings();
    draw_outer_bound(&stdout, settings, x, y).unwrap();
    draw_inner_bound(&stdout, settings, x, y).unwrap();
    flush(&stdout).unwrap();
}
