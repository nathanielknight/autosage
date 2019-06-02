/// Representation of the 52-card deck.
use crate::model::{Card, RANKS, SUITS};

pub fn new() -> Vec<Card> {
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
    let deck = new();
    assert!(deck.len() == 52);
}

pub fn shuffle(deck: &mut Vec<Card>) {
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
        let mut deck = new();
        let orig_len = deck.len();
        let hand = draw(&mut deck, handsize);
        assert!(hand.len() == handsize);
        assert!(hand.len() + deck.len() == orig_len);
    }
}
