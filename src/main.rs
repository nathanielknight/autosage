#![allow(dead_code)]

mod deck;
mod model;
mod newgame;
mod render;

fn main() {
    use bear_lib_terminal::terminal;

    let g = model::Game::generate();

    terminal::open("Test", 80, 30);

    render::draw_game(&g);
    terminal::refresh();

    let _ = terminal::wait_event();

    terminal::close();
}
