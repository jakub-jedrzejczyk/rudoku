use app::app::App;

mod app;
mod dto;
mod logic;
mod options;
mod tui;

fn main() {
    let mut stdout = std::io::stdout();

    let rudoku_app = App::new();
}

