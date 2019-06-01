use crate::model::*;

fn pop_card_at(spread: &mut Spread, p: Position) {
    let stack = spread.get_stack_mut(p);
    stack.pop();
}

pub fn update(msg: Msg, game: &mut Game) {
    match msg {
        Msg::MakeMove => {
            let mv_opt = game.selected_move();
            match mv_opt {
                Some(mv) => match mv {
                    Move::Trash(p) => {
                        game.spend_one_trash();
                        pop_card_at(&mut game.spread, p);
                        game.selected.clear();
                    }
                    Move::PlayHand(h) => {
                        assert!(game.selected_hand().unwrap() == h);
                        for p in game.selected.iter() {
                            pop_card_at(&mut game.spread, *p);
                        }
                        game.restore_one_trash();
                        game.selected.clear();
                    }
                },
                None => (),
            }
        }
        Msg::NewGame => {
            game.reset();
        }
        Msg::ToggleStack(p) => {
            if game.selected.contains(&p) {
                game.selected.remove(&p);
            } else {
                let s = game.spread.get_stack(p);
                if !s.is_empty() {
                    game.selected.insert(p);
                }
            }
        }
    }
}
