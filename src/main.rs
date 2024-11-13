mod tui;
mod game;

fn main() {
    let g = game::Game::new(3, 3);
    let (x, y) = g.calculate_canvas_size();

    println!("{x}, {y}");

    g.start();
}

