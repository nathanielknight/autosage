use crate::model::*;

pub fn update(msg: Msg, game: &mut Game) {
    match msg {
        Msg::MakeMove => {
            let mv_opt = game.selected_move();
            match mv_opt {
                Some(mv) => match mv {
                    Move::Trash(p) => {
                        game.spend_one_trash();
                        game.spread.get_stack_mut(p).pop();
                        game.selected.clear();
                    }
                    Move::PlayHand(h) => {
                        game.play_hand(h);
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
