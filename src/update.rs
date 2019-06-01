use crate::model::*;

pub fn update(msg: Msg, game: &mut Game) {
    match msg {
        Msg::MakeMove => {
            let mv = game
                .selected_move()
                .expect("MakeMove msg sent when no move was selected");
            match mv {
                Move::Trash(p) => {
                    game.spend_one_trash();
                    let stack = game.spread.get_stack_mut(p);
                    stack.pop();
                }
                Move::PlayHand(h) => unimplemented!(),
            }
        }
        Msg::NewGame => unimplemented!(),
        Msg::ToggleStack(p) => {
            if game.selected.contains(&p) {
                game.selected.remove(&p);
            } else {
                game.selected.insert(p);
            }
        }
    }
}
