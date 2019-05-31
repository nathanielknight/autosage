/// Raw representation of cards, stacks, the board, trashes, and the player's selection.
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[test]
fn test_suit_ranking() {
    // This also checks that Enum ordering works the way I think it does.
    assert!(Suit::Spade == Suit::Spade);
    assert!(Suit::Club < Suit::Diamond);
    assert!(Suit::Diamond < Suit::Heart);
    assert!(Suit::Heart < Suit::Spade);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
pub struct Card(pub Rank, pub Suit);

impl Card {
    pub fn new(r: Rank, s: Suit) -> Self {
        Card(r, s)
    }
}

pub type CardStack = Vec<Card>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RowId {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ColumnId {
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position(pub RowId, pub ColumnId);

pub type Spread = HashMap<Position, CardStack>;

pub enum Trashes {
    None,
    One,
    Two,
}

pub struct Game {
    pub spread: Spread,
    pub selected: HashSet<Position>,
    pub trashes: Trashes,
    pub bonus_card: Card,
}
