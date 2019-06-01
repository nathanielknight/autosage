use std::fmt;

use bear_lib_terminal::terminal;

use crate::model::*;

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{: >2}", c)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
        };
        write!(f, "{}", c)
    }
}

fn draw_stack(x: i32, y: i32, stack: &CardStack) {
    match stack.last() {
        None => terminal::print_xy(x, y, &format!("{}|{}{}", 0, " ", " ")),
        Some(Card(rank, suit)) => {
            terminal::print_xy(x, y, &format!("{}|{}{}", stack.len(), rank, suit,))
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Hand::Pair => "Pair",
            Hand::ThreeOfAKind => "Three of a Kind",
            Hand::StraightThree => "Three-Card Straight",
            Hand::FourOfAKind => "Four of a Kind",
            Hand::FullHouse => "Full House",
            Hand::StraightFive => "Five-Card Straight",
            Hand::Flush => "Flush",
            Hand::StraightFlush => "Straight Flush!",
        };
        write!(f, "{}", msg)
    }
}

fn draw_move(mv: Move) {
    let msg = match mv {
        Move::Trash(_) => "Trash".to_owned(),
        Move::PlayHand(h) => format!("{}", h),
    };
    terminal::print_xy(3, 15, &msg);
}

fn draw_bonus(Card(r, s): Card) {
    let msg = format!("Bonus suit: {}{}", r, s);
    terminal::print_xy(3, 18, &msg);
}

pub fn draw_game(g: &Game) {
    draw_stack(3, 3, &g.spread.tl);
    draw_stack(12, 3, &g.spread.tc);
    draw_stack(21, 3, &g.spread.tr);
    draw_stack(3, 6, &g.spread.ml);
    draw_stack(12, 6, &g.spread.mc);
    draw_stack(21, 6, &g.spread.mr);
    draw_stack(3, 9, &g.spread.bl);
    draw_stack(12, 9, &g.spread.bc);
    draw_stack(21, 9, &g.spread.br);

    if let Some(mv) = g.selected_move() {
        draw_move(mv);
    }

    draw_bonus(g.bonus_card);
}
