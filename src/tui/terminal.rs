use std::io::{self, Write};
use crossterm::{
    cursor,
    style,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};

use crate::{logic, options::display::{self, DisplayOptions}};

pub struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    pub fn new(stdout: io::Stdout) -> Terminal {
        Terminal {
            stdout,
        }
    }

    pub fn field_width(&self, size_x: u16, display_options: &DisplayOptions) -> u8 {
        let single_digit = 9;
        let digits = if size_x % single_digit == 0 { size_x / single_digit } else { size_x / single_digit + 1 };

        return (digits + display_options.get_padding_left() + display_options.get_padding_right()) as u8;
    }

    pub fn field_height(&self, size_y: u16, display_options: &DisplayOptions) -> u8 {
        let single_digit = 9;
        let digits = if size_y % single_digit == 0 { size_y / single_digit } else { size_y / single_digit + 1 };
    
    
        return (digits + display_options.get_padding_top() + display_options.get_padding_bottom()) as u8;
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        self.stdout.flush()?;
        return Ok(());
    }

    pub fn size(&self) -> (u16, u16) {
        let size = terminal::size();
        
        return match size {
            Ok(v) => (v.0, v.1),
            Err(_e) => (u16::MAX, u16::MAX),
        };
    }

    pub fn write(&mut self, c: char) -> io::Result<()> {
        self.stdout.queue(style::Print(c))?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn setup(&mut self) -> io::Result<()> {
        self.stdout
            .queue(EnterAlternateScreen)?;
        self.flush()?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        self.stdout
            .queue(LeaveAlternateScreen)?;
            //.queue(terminal::Clear(terminal::ClearType::All))?
            //.queue(cursor::MoveTo(0, 0))?;
        self.flush()?;
        Ok(())
    }

    fn calculate_canvas_size(&self, x: u16, y: u16, display_options: &DisplayOptions) -> (u16, u16) {
        let ibx = (x - 1) * display_options.get_inner_border_vertical();
        let obx = 2 * display_options.get_outer_border_vertical();
        let iby = (y - 1) * display_options.get_inner_border_horizontal();
        let oby = 2 * display_options.get_outer_border_horizontal();

        let size_x = (x * (display_options.get_padding_left() + display_options.get_padding_right())) + ibx + obx;
        let size_y = (y * (display_options.get_padding_top() + display_options.get_padding_bottom())) + iby + oby;

        return (size_x, size_y);
    }

    fn calculate_canvas_offset(&self, x: u16, y: u16, display_options: &DisplayOptions) -> (u16, u16) {
        if !display_options.get_center_board() {
            return (0, 0);
        }

        let (termial_x, terminal_y) = self.size();
        let (canvas_x, canvas_y) = self.calculate_canvas_size(x, y, display_options);

        let offset_x = (termial_x - canvas_x) / 2;
        let offset_y = (terminal_y - canvas_y) / 2;

        return (offset_x, offset_y);
    }

    pub fn draw_vertical(&mut self, x_pos: u16, y_pos: u16, length: u16, c: char) -> io::Result<()> {
        for i in 0..length {
            self.stdout
                .queue(cursor::MoveTo(x_pos + i, y_pos))?;
            self.write(c)?;
        }

        Ok(())
    }

    pub fn draw_horizontal(&mut self, x_pos: u16, y_pos: u16, length: u16, c: char) -> io::Result<()> {
        for i in 0..length {
            self.stdout
                .queue(cursor::MoveTo(x_pos, y_pos + i))?;
            self.write(c)?;
        }

        Ok(())
    }

    pub fn draw_outer_bound_vertical(&mut self, display_options: &DisplayOptions, x: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
        for i in 0..display_options.get_outer_border_vertical() {
            let mut x_pos: u16 = 0;
            for _k in 0..x {
                self.draw_vertical(x_pos + x_offset, i + y_offset, display_options.get_outer_border_horizontal(), display_options.get_outer_intersection_char()).unwrap();
                x_pos += display_options.get_outer_border_horizontal();
                self.draw_vertical(x_pos + x_offset, i + y_offset, display_options.get_field_width(), display_options.get_outer_horizontal_char()).unwrap();
                x_pos += display_options.get_field_width();
            }
            self.draw_vertical(x_pos + x_offset, i + y_offset, display_options.get_outer_border_horizontal(), display_options.get_outer_intersection_char()).unwrap();
        }

        Ok(())
    }

    pub fn draw_outer_bound_horizontal(&mut self, settings: &Settings, y: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
        for i in 0..settings.get_outer_border_horizontal() {
            let mut y_pos: u16 = 0;
            for _k in 0..y {
                self.draw_horizontal(i + x_offset, y_pos + y_offset, settings.get_outer_border_vertical(), settings.get_outer_intersection_char()).unwrap();
                y_pos += settings.get_outer_border_vertical();
                self.draw_horizontal(i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_outer_vertical_char()).unwrap();
                y_pos += settings.get_field_height();
            }
            
            self.draw_horizontal(&stdout, i + x_offset, y_pos + y_offset, settings.get_outer_border_vertical(), settings.get_outer_intersection_char()).unwrap();
        }

        Ok(())
    }

    pub fn draw_outer_bound(&mut self, settings: &Settings, x: u16, y: u16) -> io::Result<()> {
        let (x_canvas_offset, y_canvas_offset) = self.calculate_canvas_offset(x, y, settings);
        self.draw_outer_bound_vertical(settings, x, x_canvas_offset, y_canvas_offset).unwrap();
        self.draw_outer_bound_horizontal(settings, y, x_canvas_offset, y_canvas_offset).unwrap();
        let y_offset = settings.get_outer_border_horizontal() + x * settings.get_field_height() + (x - 1) * settings.get_inner_border_horizontal();
        let x_offset = settings.get_outer_border_vertical() + y * settings.get_field_width() + (y - 1) * settings.get_inner_border_vertical();
        self.draw_outer_bound_vertical(settings, x, x_canvas_offset, y_canvas_offset + y_offset).unwrap();
        self.draw_outer_bound_horizontal(settings, y, x_canvas_offset + x_offset, y_canvas_offset).unwrap();
        Ok(())
    }

    pub fn draw_inner_bound_vertical(&mut self, settings: &Settings, x: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
        for i in 0..settings.get_inner_border_vertical() {
            let mut x_pos: u16 = 0;
            for _k in 0..(x - 1) {
                self.draw_vertical(x_pos + x_offset, i + y_offset, settings.get_field_width(), settings.get_inner_horizontal_char()).unwrap();
                x_pos += settings.get_field_width();
                self.draw_vertical(x_pos + x_offset, i + y_offset, settings.get_inner_border_horizontal(), settings.get_inner_intersection_char()).unwrap();
                x_pos += settings.get_inner_border_vertical();
            }
            self.draw_vertical(x_pos + x_offset, i + y_offset, settings.get_field_width(), settings.get_inner_horizontal_char()).unwrap();
        }

        Ok(())
    }

    pub fn draw_inner_bound_horizontal(&mut self, settings: &Settings, y: u16, x_offset: u16, y_offset: u16) -> io::Result<()> {
        for i in 0..settings.get_inner_border_horizontal() {
            let mut y_pos: u16 = 0;
            for _k in 0..(y - 1) {
                self.draw_horizontal(i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_inner_vertical_char()).unwrap();
                y_pos += settings.get_field_height();
                self.draw_horizontal(i + x_offset, y_pos + y_offset, settings.get_inner_border_vertical(), settings.get_inner_intersection_char()).unwrap();
                y_pos += settings.get_inner_border_horizontal();
            }
            self.draw_horizontal(i + x_offset, y_pos + y_offset, settings.get_field_height(), settings.get_inner_vertical_char()).unwrap();
        }

        Ok(())
    }

    pub fn draw_inner_bound(&mut self, settings: &Settings, x: u16, y: u16) -> io::Result<()> {
        let (x_canvas_offset, y_canvas_offset) = self.calculate_canvas_offset(x, y, settings);
        let x_offset = x_canvas_offset + settings.get_outer_border_vertical();
        let y_offset = y_canvas_offset + settings.get_outer_border_horizontal();
        
        let mut x_mod = settings.get_field_width();
        let mut y_mod = settings.get_field_height();
        for _i in 0..(x - 1) {
            self.draw_inner_bound_vertical(settings, x, x_offset, y_offset + y_mod).unwrap();
            y_mod += settings.get_field_height() + settings.get_inner_border_horizontal();
        }
        for _i in 0..(y - 1) {
            self.draw_inner_bound_horizontal(settings, y, x_offset + x_mod, y_offset).unwrap();
            x_mod += settings.get_field_width() + settings.get_inner_border_vertical();
        }

        Ok(())
    }

    pub fn draw_board(&mut self, game: &logic::game::Game) {
        let x = game.get_size_x();
        let y = game.get_size_y();
        let settings = game.get_settings();
        self.draw_outer_bound(settings, x, y).unwrap();
        self.draw_inner_bound(settings, x, y).unwrap();
        self.flush().unwrap();
    }

    pub fn calculate_cell_offset(&self, size_x: u16, size_y: u16, x: u16, y: u16, settings: &Settings) -> (u16, u16) {
        let (x_canvas_offset, y_canvas_offset) = self.calculate_canvas_offset(size_x, size_y, settings);
        let x_offset = x_canvas_offset + settings.get_outer_border_vertical() + (x * settings.get_field_width()) + (x * settings.get_inner_border_vertical());
        let y_offset = y_canvas_offset + settings.get_outer_border_horizontal() + (y * settings.get_field_height()) + (y * settings.get_inner_border_horizontal());
        
        return (x_offset, y_offset);
    } 

    pub fn draw_cell(&mut self, game: &logic::game::Game, x: u16, y: u16) -> io::Result<()> {
        let (x_offset, y_offset) = self.calculate_cell_offset(game.get_size_x(), game.get_size_y(), x, y, game.get_settings());
        let c = game.get_cell(x, y).get_char();
        self.stdout
            .queue(cursor::MoveTo(x_offset, y_offset))?;
        self.write(c)?;

        Ok(())
    }

    pub fn draw_cells(&mut self, game: &logic::game::Game) {
        let size_x = game.get_size_x();
        let size_y = game.get_size_y();

        for x in 0..size_x {
            for y in 0..size_y {
                self.draw_cell(game, x, y);
            }
        }

        self.flush();
    }

    pub fn move_cursor(&mut self, game: &logic::game::Game) -> io::Result<()> {
        let x = game.get_cursor_x();
        let y = game.get_cursor_y();

        let (x_offset, y_offset) = self.calculate_cell_offset(game.get_size_x(), game.get_size_y(), x, y, game.get_settings());

        self.stdout
            .queue(cursor::MoveTo(x_offset, y_offset))?;
        self.flush();

        Ok(())

    }
}


