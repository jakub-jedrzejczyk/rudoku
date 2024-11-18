use async_std::io;
use tui::clear_screen;
use logic::game;

mod logic;
mod tui;

fn main() {
    let g = game::Game::new(3, 3, std::io::stdout());

    let mut stdout = std::io::stdout();
    g.start();
    clear_screen();
}

