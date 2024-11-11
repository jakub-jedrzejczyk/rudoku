mod terminal_utils;

fn main() {
    let _ = terminal_utils::clear_screen();
    let size = terminal_utils::size();
    let (x, y) = size;

    let _ = terminal_utils::set_size(50, 25);
    terminal_utils::write(format!("({x}, {y})").to_string());
}

