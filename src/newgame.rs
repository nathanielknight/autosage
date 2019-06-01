use std::collections::HashSet;

use crate::deck;
use crate::model::*;

const PILE_SIZES: [(Position, usize); 9] = [
    (Position(RowId::Top, ColumnId::Left), 8),
    (Position(RowId::Top, ColumnId::Center), 8),
    (Position(RowId::Top, ColumnId::Right), 8),
    (Position(RowId::Middle, ColumnId::Left), 7),
    (Position(RowId::Middle, ColumnId::Center), 6),
    (Position(RowId::Middle, ColumnId::Right), 5),
    (Position(RowId::Bottom, ColumnId::Left), 4),
    (Position(RowId::Bottom, ColumnId::Center), 3),
    (Position(RowId::Bottom, ColumnId::Right), 2),
];

#[test]
fn test_pile_sizes() {
    let mut total = 0;
    for (_, size) in &PILE_SIZES {
        total += size;
    }
    assert!(total == 51);
}

impl Game {
    pub fn generate() -> Game {
        let mut spread = Spread::empty();
        let mut d = deck::new();
        deck::shuffle(&mut d);
        for (pos, cnt) in &PILE_SIZES {
            let hand = deck::draw(&mut d, *cnt);
            let stack: &mut Vec<_> = spread.get_stack_mut(*pos);
            stack.extend(hand);
        }
        assert!(d.len() == 1);
        let bonus_card = d.pop().expect("Standard draw didn't leave a bonus card?");
        Game {
            spread,
            selected: HashSet::new(),
            trashes: Trashes::Two,
            bonus_card,
        }
    }

    pub fn reset(&mut self) {
        self.selected.clear();
        self.spread = Spread::empty();
        self.trashes = Trashes::Two;

        let mut d = deck::new();
        deck::shuffle(&mut d);
        for (pos, cnt) in &PILE_SIZES {
            let hand = deck::draw(&mut d, *cnt);
            let stack: &mut Vec<_> = self.spread.get_stack_mut(*pos);
            stack.extend(hand);
        }
        assert!(d.len() == 1);
        self.bonus_card = d.pop().expect("Standard draw didn't leave a bonus card?");
    }
}
