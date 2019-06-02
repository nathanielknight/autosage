use bear_lib_terminal::terminal;

mod deck;
mod model;
mod newgame;
mod render;
mod update;

fn parse_msg(k: terminal::KeyCode) -> Option<model::Msg> {
    use model::*;
    use terminal::KeyCode;
    match k {
        KeyCode::Q | KeyCode::Num7 => Some(Msg::ToggleStack(Position(RowId::Top, ColumnId::Left))),
        KeyCode::W | KeyCode::Num8 => {
            Some(Msg::ToggleStack(Position(RowId::Top, ColumnId::Center)))
        }
        KeyCode::E | KeyCode::Num9 => Some(Msg::ToggleStack(Position(RowId::Top, ColumnId::Right))),
        KeyCode::A | KeyCode::Num4 => {
            Some(Msg::ToggleStack(Position(RowId::Middle, ColumnId::Left)))
        }
        KeyCode::S | KeyCode::Num5 => {
            Some(Msg::ToggleStack(Position(RowId::Middle, ColumnId::Center)))
        }
        KeyCode::D | KeyCode::Num6 => {
            Some(Msg::ToggleStack(Position(RowId::Middle, ColumnId::Right)))
        }
        KeyCode::Z | KeyCode::Num1 => {
            Some(Msg::ToggleStack(Position(RowId::Bottom, ColumnId::Left)))
        }
        KeyCode::X | KeyCode::Num2 => {
            Some(Msg::ToggleStack(Position(RowId::Bottom, ColumnId::Center)))
        }
        KeyCode::C | KeyCode::Num3 => {
            Some(Msg::ToggleStack(Position(RowId::Bottom, ColumnId::Right)))
        }
        KeyCode::Space => Some(Msg::MakeMove),
        KeyCode::P => Some(Msg::NewGame),
        KeyCode::Escape => {
            terminal::close();
            None
        }
        KeyCode::H => {
            render_help();
            None
        }
        _ => None,
    }
}

fn render_help() {
    terminal::clear(None);
    let msg = r#"
    Auto Sage
    written by Nathaniel Knight
    nathanielknight.ca

    Based on Sage Solitaire by Zach Gage
    sagesolitaire.com

    Select cards with       Play move with
    Q W E                   SPC
    A S D
    Z X C

    Quit with               New Game
    Esc                     P

    Press any key to return

    Built with the BearLibTerminal Library
    http://foo.wyrd.name/en:bearlibterminal
    "#;
    terminal::print_xy(3, 3, msg);
    terminal::refresh();
    terminal::wait_event();
}

fn main() {
    let mut g = model::Game::generate();

    terminal::open("Auto-Sage", 52, 27);

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
        terminal::clear(None);
        render::draw_game(&g);
        terminal::refresh();
    }
    terminal::close();
}
