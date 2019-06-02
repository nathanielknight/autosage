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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Rank {
    fn successor(self) -> Rank {
        match self {
            Rank::Ace => Rank::Two,
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Card(pub Rank, pub Suit);

impl Card {
    pub fn new(r: Rank, s: Suit) -> Self {
        Card(r, s)
    }
}

/// Representation of the 52-card deck.
pub const SUITS: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
pub const RANKS: [Rank; 13] = [
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
    pub tl: CardStack,
    pub tc: CardStack,
    pub tr: CardStack,
    pub ml: CardStack,
    pub mc: CardStack,
    pub mr: CardStack,
    pub bl: CardStack,
    pub bc: CardStack,
    pub br: CardStack,
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

#[derive(Debug, PartialEq)]
pub enum Trashes {
    None,
    One,
    Two,
}

#[derive(Debug)]
pub struct Game {
    pub spread: Spread,
    pub selected: HashSet<Position>,
    pub trashes: Trashes,
    pub bonus_card: Card,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    Pair,
    StraightThree,
    ThreeOfAKind,
    StraightFive,
    FullHouse,
    Flush,
    FourOfAKind,
    StraightFlush,
}

impl Hand {
    pub fn points(&self) -> u32 {
        match self {
            Hand::Pair => 1,
            Hand::StraightThree => 2,
            Hand::ThreeOfAKind => 3,
            Hand::StraightFive => 5,
            Hand::FullHouse => 7,
            Hand::Flush => 9,
            Hand::FourOfAKind => 10,
            Hand::StraightFlush => 15,
        }
    }
}

pub enum Msg {
    MakeMove,
    ToggleStack(Position),
    NewGame,
}

pub enum Move {
    Trash(Position),
    PlayHand(Hand),
}

// -------------------------------------------------
// Predicates

// NB: Tests for predicates are in the test module below

fn all_eq<I, X>(xs: I) -> bool
where
    X: std::cmp::Eq,
    I: IntoIterator<Item = X>,
{
    let mut iter = xs.into_iter();
    let fst = match iter.next() {
        Some(x) => x,
        None => return true,
    };
    iter.all(|x| x == fst)
}

fn is_straight(cards: &HashSet<Card>) -> bool {
    let mut sorted_cards = {
        let mut cs: Vec<_> = cards.iter().collect();
        cs.sort();
        cs
    };
    // Check specifically if the ace should be high
    if sorted_cards.len() > 2 {
        let Card(br, _) = sorted_cards[0];
        let Card(tr, _) = sorted_cards.last().unwrap();
        if *br == Rank::Ace && *tr == Rank::King {
            let ace = sorted_cards.remove(0);
            let Card(r, _) = ace;
            assert!(*r == Rank::Ace);
            sorted_cards.push(ace);
        }
    }
    // Otherwise, check normally
    for idx in 0..(sorted_cards.len() - 1) {
        let Card(r1, _) = sorted_cards[idx];
        let Card(r2, _) = sorted_cards[idx + 1];
        if *r2 != r1.successor() {
            return false;
        }
    }
    true
}

// -----------------------------------
// Hand detection

fn check_fullhouse(cards: &HashSet<Card>) -> bool {
    use std::collections::HashMap;
    let mut ranks: HashMap<Rank, usize> = HashMap::new();
    for Card(r, _) in cards.iter() {
        let cnt = ranks.entry(*r).or_insert(0);
        *cnt += 1;
    }
    let mut vals: Vec<usize> = ranks.values().map(|u| *u).collect();
    vals.sort();
    vals == vec![2, 3]
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
            }
            3 => {
                //straight, 3oC
                if all_eq(self.selected_cards().iter().map(|Card(r, _)| r)) {
                    Some(Hand::ThreeOfAKind)
                } else if is_straight(&self.selected_cards()) {
                    Some(Hand::StraightThree)
                } else {
                    None
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
                if check_fullhouse(&self.selected_cards()) {
                    return Some(Hand::FullHouse);
                }
                let straight = is_straight(&self.selected_cards());
                let flush = all_eq(self.selected_cards().iter().map(|Card(_, s)| s));
                match (straight, flush) {
                    (true, true) => Some(Hand::StraightFlush),
                    (true, false) => Some(Hand::StraightFive),
                    (false, true) => Some(Hand::Flush),
                    (false, false) => None,
                }
            }
            _ => None,
        }
    }

    pub fn spend_one_trash(&mut self) {
        match self.trashes {
            Trashes::Two => {
                self.trashes = Trashes::One;
            }
            Trashes::One => {
                self.trashes = Trashes::None;
            }
            Trashes::None => {
                panic!("Tried to spend a trash when none were available");
            }
        }
    }

    pub fn restore_one_trash(&mut self) {
        match self.trashes {
            Trashes::Two => (),
            Trashes::One => {
                self.trashes = Trashes::Two;
            }
            Trashes::None => {
                self.trashes = Trashes::One;
            }
        }
    }

    fn selected_rows(&self) -> usize {
        let mut rs: HashSet<RowId> = HashSet::new();
        for Position(r, _) in self.selected.iter() {
            rs.insert(*r);
        }
        rs.len()
    }

    pub fn selected_move(&self) -> Option<Move> {
        let scs = self.selected_cards();
        if scs.len() == 0 {
            return None;
        }
        if scs.len() == 1 {
            if self.trashes == Trashes::None {
                return None;
            }
            assert_eq!(self.selected.len(), 1);
            let pos = self.selected.iter().next().unwrap();
            return Some(Move::Trash(*pos));
        }
        self.selected_hand().map(|h| Move::PlayHand(h))
    }

    pub fn remaining_cards(&self) -> HashSet<Card> {
        let mut cards = HashSet::new();
        let mut add_stack = |st: &Vec<Card>| {
            for c in st {
                cards.insert(c.clone());
            }
        };
        add_stack(&self.spread.tl);
        add_stack(&self.spread.tc);
        add_stack(&self.spread.tr);
        add_stack(&self.spread.ml);
        add_stack(&self.spread.mc);
        add_stack(&self.spread.mr);
        add_stack(&self.spread.bl);
        add_stack(&self.spread.bc);
        add_stack(&self.spread.br);
        cards
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

    macro_rules! cards {
        ( $( $csrc:expr ),* ) => {
            {
                let mut cards = HashSet::new();
                $(
                    cards.insert(c($csrc));
                )*
                cards
            }
        }
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

    fn insert_card(g: &mut Game, p_src: &str, c_src: &str) {
        let s = g.spread.get_stack_mut(p(p_src));
        s.push(c(c_src));
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

        fn select(&mut self, p_srcs: &str) {
            let p_atoms = p_srcs.split(' ');
            self.selected.clear();
            for p_src in p_atoms {
                self.selected.insert(p(p_src));
            }
        }
    }
    // ----------------------------------------------------

    // Predicate tests

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

    #[test]
    fn test_is_straight() {
        assert!(is_straight(&cards!("as", "2s", "3s")));
        assert!(is_straight(&cards!("3s", "as", "2s")));
        assert!(is_straight(&cards!("9d", "0c", "jh", "qs")));
        assert!(is_straight(&cards!("jd", "qh", "ks", "ac")));
    }
    #[test]
    fn test_is_not_straight() {
        assert!(!is_straight(&cards!("as", "2s", "5s")));
        assert!(!is_straight(&cards!("kd", "as", "2h")));
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
        insert_card(&mut g, "tl", "2s");
        g.selected.insert(p("tl"));
        assert_eq!(g.selected.len(), 1);
        assert!(g.selected_hand().is_none());
    }

    #[test]
    fn check_pairs() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "2s");
        insert_card(g, "tr", "2c");
        insert_card(g, "bl", "2h");
        insert_card(g, "br", "3s");
        g.select("tl tr");
        assert!(g.selected_hand().is_none(), "Pair in same row is no hand");
        g.select("tl br");
        assert!(g.selected_hand().is_none(), "Not-equal cards are not pair");
        g.select("tl bl");
        assert!(g.selected_hand().is_some());
        assert_eq!(
            g.selected_hand().expect("Expected to have selected a pair"),
            Hand::Pair
        );
    }

    #[test]
    fn check_3_of_a_kind() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "as");
        insert_card(g, "tc", "ad");
        insert_card(g, "tr", "ah");
        insert_card(g, "ml", "ac");
        insert_card(g, "mc", "2h");
        g.select("tl tc tr");
        assert!(
            g.selected_hand().is_none(),
            "One row can't be a three-of-a-kind"
        );
        g.select("tl tc ml");
        assert_eq!(g.selected_hand().unwrap(), Hand::ThreeOfAKind);
        g.select("tl tc mc");
        assert!(
            g.selected_hand().is_none(),
            "Three cards with a mach isn't a three-off-a-kind"
        );
        g.select("tl tc ml mc");
        assert!(
            g.selected_hand().is_none(),
            "Four with three matches isn't a three-of-a-kind"
        );
    }

    #[test]
    fn test_small_straight() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "as");
        insert_card(g, "tc", "2s");
        insert_card(g, "tr", "3s");
        insert_card(g, "ml", "3c");
        insert_card(g, "mc", "ks");
        g.select("tl tc tr");
        assert!(g.selected_hand().is_none(), "One row cant be a straight");
        g.select("tl tc ml");
        assert_eq!(g.selected_hand().unwrap(), Hand::StraightThree);
        g.select("tl tc mc");
        assert!(g.selected_hand().is_none(), "Can't wrap a straight");
    }

    #[test]
    fn test_four_of_a_kind() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "as");
        insert_card(g, "tc", "ad");
        insert_card(g, "tr", "ah");
        insert_card(g, "ml", "ac");
        insert_card(g, "mc", "2c");
        g.select("tl tc tr ml");
        assert_eq!(g.selected_hand().unwrap(), Hand::FourOfAKind);
        g.select("tl tc tr mc");
        assert!(g.selected_hand().is_none());
    }

    #[test]
    fn test_straight() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "ah");
        insert_card(g, "tc", "2h");
        insert_card(g, "tr", "3h");
        insert_card(g, "ml", "4d");
        insert_card(g, "mc", "5d");
        insert_card(g, "mr", "ks");
        g.select("tl tc tr ml mc");
        assert_eq!(g.selected_hand().unwrap(), Hand::StraightFive);
        g.select("tl tc tr ml mr");
        assert!(g.selected_hand().is_none());
    }

    #[test]
    fn test_straight_flush() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "ah");
        insert_card(g, "tc", "2h");
        insert_card(g, "tr", "3h");
        insert_card(g, "ml", "4h");
        insert_card(g, "mc", "5h");
        insert_card(g, "mr", "ks");
        insert_card(g, "bl", "5d");

        g.select("tl tc tr ml mc");
        assert_eq!(g.selected_hand().unwrap(), Hand::StraightFlush);

        g.select("tl tc tr ml mr");
        assert!(g.selected_hand().is_none());

        g.select("tl tc tr ml bl");
        assert_eq!(
            g.selected_hand().unwrap(),
            Hand::StraightFive,
            "Mixed suit means straight, not straight flush"
        );
    }

    #[test]
    fn test_full_house() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "as");
        insert_card(g, "tc", "ac");
        insert_card(g, "tr", "ad");
        insert_card(g, "ml", "8s");
        insert_card(g, "mc", "8d");
        g.select("tl tc tr ml mc");
        assert_eq!(g.selected_hand().unwrap(), Hand::FullHouse);
    }

    #[test]
    fn test_over_selection() {
        let g = &mut Game::empty();
        insert_card(g, "tl", "ah");
        insert_card(g, "tc", "2h");
        insert_card(g, "tr", "3h");
        insert_card(g, "ml", "4h");
        insert_card(g, "mc", "5h");
        insert_card(g, "mr", "ks");
        insert_card(g, "bl", "5d");
        g.select("tl tr tc ml mr mc");
        assert!(g.selected_hand().is_none());
    }
}
