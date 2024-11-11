mod terminal_utils;

fn main() {
    let _ = terminal_utils::clear_screen();
    let size = terminal_utils::size();
    let x = size.0;
    let y = size.1;

    println!("({x}, {y})");
}

