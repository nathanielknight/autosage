#![allow(dead_code)]

mod deck;
mod model;
mod newgame;

fn main() {
    use bear_lib_terminal::terminal;

    terminal::open("Test", 80, 30);
    terminal::print_xy(0, 0, "Hello from rust");
    terminal::refresh();

    let _ = terminal::wait_event();

    terminal::close();
}
