/// Raw representation of cards, stacks, the board, trashes, and the player's selection.
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Card(pub Rank, pub Suit);

impl Card {
    pub fn new(r: Rank, s: Suit) -> Self {
        Card(r, s)
    }
}
/// Representation of the 52-card deck.

const SUITS: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

pub fn new_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for &r in RANKS.iter() {
        for &s in SUITS.iter() {
            deck.push(Card::new(r, s));
        }
    }
    deck
}

#[test]
fn test_deck_size() {
    let deck = new_deck();
    assert!(deck.len() == 52);
}

pub fn shuffle(deck: &mut Vec<Card>) {
    use rand;
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

pub fn draw(deck: &mut Vec<Card>, cards: usize) -> Vec<Card> {
    assert!(deck.len() >= cards, "Tried to overdraw");
    let mut hand = Vec::new();
    for _ in 0..cards {
        let c = match deck.pop() {
            Some(c) => c,
            None => panic!("Tried to overdraw while drawing {} cards", cards),
        };
        hand.push(c);
    }
    hand
}

#[test]
fn test_drawing() {
    for handsize in 0..52 {
        let mut deck = new_deck();
        let orig_len = deck.len();
        let hand = draw(&mut deck, handsize);
        assert!(hand.len() == handsize);
        assert!(hand.len() + deck.len() == orig_len);
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

/// A board with cards on it
#[derive(Debug)]
pub struct Spread {
    tl: CardStack,
    tc: CardStack,
    tr: CardStack,
    ml: CardStack,
    mc: CardStack,
    mr: CardStack,
    bl: CardStack,
    bc: CardStack,
    br: CardStack,
}

impl Spread {
    pub fn empty() -> Self {
        Spread {
            tl: Vec::new(),
            tc: Vec::new(),
            tr: Vec::new(),
            ml: Vec::new(),
            mc: Vec::new(),
            mr: Vec::new(),
            bl: Vec::new(),
            bc: Vec::new(),
            br: Vec::new(),
        }
    }

    pub fn get_stack(&self, pos: Position) -> &CardStack {
        match pos {
            Position(RowId::Top, ColumnId::Left) => &self.tl,
            Position(RowId::Top, ColumnId::Center) => &self.tc,
            Position(RowId::Top, ColumnId::Right) => &self.tr,
            Position(RowId::Middle, ColumnId::Left) => &self.ml,
            Position(RowId::Middle, ColumnId::Center) => &self.mc,
            Position(RowId::Middle, ColumnId::Right) => &self.mr,
            Position(RowId::Bottom, ColumnId::Left) => &self.bl,
            Position(RowId::Bottom, ColumnId::Center) => &self.bc,
            Position(RowId::Bottom, ColumnId::Right) => &self.br,
        }
    }
    pub fn get_stack_mut(&mut self, pos: Position) -> &mut CardStack {
        match pos {
            Position(RowId::Top, ColumnId::Left) => &mut self.tl,
            Position(RowId::Top, ColumnId::Center) => &mut self.tc,
            Position(RowId::Top, ColumnId::Right) => &mut self.tr,
            Position(RowId::Middle, ColumnId::Left) => &mut self.ml,
            Position(RowId::Middle, ColumnId::Center) => &mut self.mc,
            Position(RowId::Middle, ColumnId::Right) => &mut self.mr,
            Position(RowId::Bottom, ColumnId::Left) => &mut self.bl,
            Position(RowId::Bottom, ColumnId::Center) => &mut self.bc,
            Position(RowId::Bottom, ColumnId::Right) => &mut self.br,
        }
    }
}

pub type Selection = HashSet<Position>;

#[derive(Debug)]
pub enum Trashes {
    None,
    One,
    Two,
}

#[derive(Debug)]
pub struct Game {
    pub spread: Spread,
    pub selected: Selection,
    pub trashes: Trashes,
    pub bonus_card: Card,
}

impl Game {
    fn check_selection(&self) {
        for pos in &self.selected {
            let stack = self.spread.get_stack(*pos);
            assert!(stack.len() > 0, "Empty stack selected");
        }
    }
}

pub enum Msg {
    MakeMove,
    ToggleStack(Position),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    Pair,
    StraightThree,
    ThreeOfAKind,
    FiveStraight,
    FullHouse,
    Flush,
    FourOfAKind,
    StraightFlush,
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Pair => 1,
            Hand::StraightThree => 2,
            Hand::ThreeOfAKind => 3,
            Hand::FiveStraight => 5,
            Hand::FullHouse => 7,
            Hand::Flush => 9,
            Hand::FourOfAKind => 10,
            Hand::StraightFlush => 15,
        }
    }
}

fn all_eq<I, X>(xs: I) -> bool
where
    X: std::cmp::Eq + std::fmt::Debug,
    I: IntoIterator<Item = X> + std::fmt::Debug,
{
    let mut iter = xs.into_iter();
    let fst = match iter.next() {
        Some(x) => x,
        None => return true,
    };
    iter.all(|x| x == fst)
}

#[test]
fn test_all_eq() {
    let empty: Vec<u8> = vec![];
    assert!(all_eq(&empty));
    let one = vec![0];
    assert!(all_eq(&one));
    let two_match = vec![0, 0];
    assert!(all_eq(&two_match));
    let three_match = vec![1, 1, 1];
    assert!(all_eq(&three_match));
    let two_mismatch = vec![1, 0];
    assert!(!all_eq(&two_mismatch));
    let three_mismatch = vec![1, 1, 0];
    assert!(!all_eq(&three_mismatch))
}

impl Game {
    fn selected_cards(&self) -> HashSet<Card> {
        let mut cards = HashSet::new();
        for p in self.selected.iter() {
            let stack = self.spread.get_stack(*p);
            let card = stack
                .last()
                .expect("Tried to get selected card of empty stack");
            cards.insert(card.clone());
        }
        cards
    }

    pub fn selected_hand(&self) -> Option<Hand> {
        if self.selected_rows() < 2 {
            return None;
        }
        match self.selected.len() {
            2 => {
                if all_eq(self.selected_cards().iter().map(|Card(r, _)| r)) {
                    Some(Hand::Pair)
                } else {
                    None
                }
            },
            3 => { //straight, 3oC
                if all_eq(self.selected_cards().iter().map(|Card(r, _)| r)) {
                    Some(Hand::ThreeOfAKind)
                } else {
                    unimplemented!()
                }
            }
            4 => {
                if all_eq(self.selected_cards().iter().map(|Card(r, _)| r)) {
                    Some(Hand::FourOfAKind)
                } else {
                    None
                }
            }
            5 => {
                if all_eq(self.selected_cards().iter().map(|Card(_, s)| s)) {
                    Some(Hand::Flush)
                } else {
                    unimplemented!()
                }
            }
            _ => None
        }
    }
    pub fn selected_rows(&self) -> usize {
        let mut rs: HashSet<RowId> = HashSet::new();
        for Position(r, _) in self.selected.iter() {
            rs.insert(*r);
        }
        rs.len()
    }
}

#[cfg(test)]
mod test_game_logic {
    use super::*;

    // -----------------------------------------------------------
    // Testing helpers

    fn c(src: &str) -> Card {
        let mut chars = src.chars();
        let r = match chars.next().expect("need a rank") {
            'a' => Rank::Ace,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            '0' => Rank::Ten,
            'j' => Rank::Jack,
            'q' => Rank::Queen,
            'k' => Rank::King,
            _ => panic!("Invalid rank character"),
        };
        let s = match chars.next().expect("need a suit") {
            'c' => Suit::Club,
            'd' => Suit::Diamond,
            'h' => Suit::Heart,
            's' => Suit::Spade,
            _ => panic!("Invalid suit character"),
        };
        Card(r, s)
    }

    fn p(src: &str) -> Position {
        let mut chars = src.chars();
        let r = match chars.next().expect("need a row") {
            't' => RowId::Top,
            'm' => RowId::Middle,
            'b' => RowId::Bottom,
            _ => panic!("Invalid row char"),
        };
        let c = match chars.next().expect("need a column") {
            'l' => ColumnId::Left,
            'c' => ColumnId::Center,
            'r' => ColumnId::Right,
            _ => panic!("Invalid column char"),
        };
        Position(r, c)
    }

    fn insert_cards(g: &mut Game, p: Position, cs: &[Card]) {
        let s = g.spread.get_stack_mut(p);
        for c in cs {
            s.push(*c);
        }
    }

    impl Game {
        fn empty() -> Game {
            Game {
                spread: Spread::empty(),
                selected: HashSet::new(),
                bonus_card: c("as"),
                trashes: Trashes::Two,
            }
        }

        fn select(&mut self, posns: &[Position]) {
            self.selected.clear();
            for p in posns {
                self.selected.insert(*p);
            }
        }
    }

    // ----------------------------------------------------
    // Board querying

    #[test]
    fn test_rows_counting() {
        let mut g = Game::empty();
        assert_eq!(g.selected_rows(), 0);
        g.selected.insert(p("tl"));
        assert_eq!(g.selected_rows(), 1);
        g.selected.insert(p("tc"));
        assert_eq!(g.selected_rows(), 1);
        g.selected.insert(p("mr"));
        assert_eq!(g.selected_rows(), 2);
        g.selected.insert(p("bl"));
        assert_eq!(g.selected_rows(), 3);
    }

    // -------------------------------------------------------
    // Hands
    #[test]
    fn check_no_selection_has_no_move() {
        let g = Game::empty();
        assert!(g.selected_hand().is_none())
    }

    #[test]
    fn check_one_selected_card_has_no_hand() {
        let mut g = Game::empty();
        insert_cards(&mut g, p("tl"), &[c("2s")]);
        g.selected.insert(p("tl"));
        assert_eq!(g.selected.len(), 1);
        assert!(g.selected_hand().is_none());
    }

    #[test]
    fn check_pairs() {
        let g = &mut Game::empty();
        insert_cards(g, p("tl"), &[c("2s")]);
        insert_cards(g, p("tr"), &[c("2c")]);
        insert_cards(g, p("bl"), &[c("2h")]);
        insert_cards(g, p("br"), &[c("3s")]);
        g.select(&[p("tl"), p("tr")]);
        assert!(g.selected_hand().is_none(), "Pair in same row is no hand");
        g.select(&[p("tl"), p("br")]);
        assert!(g.selected_hand().is_none(), "Not-equal cards are not pair");
        g.select(&[p("tl"), p("bl")]);
        assert!(g.selected_hand().is_some());
        assert_eq!(
            g.selected_hand().expect("Expected to have selected a pair"),
            Hand::Pair
        );
    }

}
