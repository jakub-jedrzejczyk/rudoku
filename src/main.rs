use async_std::io;
use tui::clear_screen;

mod tui;
mod game;

fn main() {
    let g = game::Game::new(3, 3);
    let (x, y) = g.calculate_canvas_size();

    println!("{x}, {y}");

    let mut stdout = std::io::stdout();
    tui::setup(&stdout);
    g.start();
    clear_screen();
    tui::draw_vertical_line(&stdout, '|', 5, 5, 5);
    tui::flush(&stdout);
    tui::cleanup(&stdout);
}

