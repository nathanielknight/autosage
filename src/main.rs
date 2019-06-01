#![allow(dead_code)]

use bear_lib_terminal::terminal;

mod deck;
mod model;
mod newgame;
mod render;
mod update;

fn parse_evt(evt_o: &Option<terminal::Event>) -> Option<model::Msg> {
    if let Some(evt) = evt_o {
        match evt {
            terminal::Event::KeyPressed {
                key: k,
                ctrl: _,
                shift: _,
            } => Some(model::Msg::NewGame),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_msg(k: terminal::KeyCode) -> Option<model::Msg> {
    None
}

fn main() {
    let mut g = model::Game::generate();

    terminal::open("Test", 80, 30);

    render::draw_game(&g);
    terminal::refresh();

    loop {
        let t_evt = match terminal::wait_event() {
            Some(e) => e,
            None => continue,
        };
        match t_evt {
            terminal::Event::Close => break, // leave main loop on quit
            terminal::Event::KeyPressed {
                key: k,
                ctrl: _,
                shift: _,
            } => {
                // If a key was pressed that matches an input method, update the game
                if let Some(msg) = parse_msg(k) {
                    update::update(msg, &mut g);
                }
            }
            _ => continue, // Ignore other messages
        }
        render::draw_game(&g);
        terminal::refresh();
    }
    terminal::close();
}
