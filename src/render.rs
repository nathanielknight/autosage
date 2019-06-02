use std::collections::HashSet;
use std::fmt;

use bear_lib_terminal::{terminal, Color};

use crate::model::*;

impl Into<String> for Rank {
    fn into(self) -> String {
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
        c.to_owned()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.clone();
        let s: String = r.into();
        write!(f, "{: >2}", s)
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Card(r, s) = self;
        let colo = match s {
            Suit::Club => "70,224,53",
            Suit::Diamond => "237,237,113",
            Suit::Heart => "226,99,99",
            Suit::Spade => "75,134,239",
        };
        write!(f, "[color={}]{: >2}{}[/color]", colo, r, s)
    }
}

fn draw_stack(x: i32, y: i32, stack: &CardStack, selected: bool) {
    if selected {
        terminal::set_background(Color::from_rgb(50, 50, 50));
    }
    match stack.last() {
        None => terminal::print_xy(x, y, &format!("[color=gray]{}|   ", 0)),
        Some(card) => terminal::print_xy(x, y, &format!("[color=gray]{}|{}", stack.len(), card)),
    }
    //reset default bg color
    terminal::set_background(Color::from_rgb(0, 0, 0));
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

fn draw_move(mv_opt: Option<Move>) {
    if let Some(mv) = mv_opt {
        let msg = match mv {
            Move::Trash(_) => "Trash".to_owned(),
            Move::PlayHand(h) => format!("{}", h),
        };
        terminal::print_xy(3, 15, &msg);
    }
}

fn draw_bonus(c: Card) {
    let msg = format!("Bonus suit: {}", c);
    terminal::print_xy(33, 6, &msg);
}

fn draw_trashes(t: &Trashes) {
    let msg = match &t {
        Trashes::None => "",
        Trashes::One => "•",
        Trashes::Two => "••",
    };

    terminal::print_xy(33, 3, &format!("Trashes:{}", msg));
}

fn draw_remaining(rem_cards: &HashSet<Card>) {
    const TOP: i32 = 9;
    const LEFT: i32 = 33;
    terminal::print_xy(LEFT, TOP, "Remaining Cards:");
    terminal::print_xy(LEFT, TOP + 2, "   ♣♦♥♠");
    let mut idx: usize = 1;
    for &rank in &RANKS {
        let cdhs = [
            rem_cards.contains(&Card(rank, Suit::Club)),
            rem_cards.contains(&Card(rank, Suit::Diamond)),
            rem_cards.contains(&Card(rank, Suit::Heart)),
            rem_cards.contains(&Card(rank, Suit::Spade)),
        ];
        let cdhs_s: String = cdhs.iter().map(|i| if *i { '•' } else { ' ' }).collect();
        let rank_s: String = rank.into();
        let row: String = format!("{: >2} {}", rank_s, cdhs_s);
        terminal::print_xy(LEFT, TOP + 2 + idx as i32, &row);
        idx += 1;
    }
}

fn draw_help_msg() {
    terminal::print_xy(3, 24, "Press 'H' for help");
}

pub fn draw_game(g: &Game) {
    draw_stack(
        3,
        3,
        &g.spread.tl,
        g.selected.contains(&Position(RowId::Top, ColumnId::Left)),
    );
    draw_stack(
        10,
        3,
        &g.spread.tc,
        g.selected.contains(&Position(RowId::Top, ColumnId::Center)),
    );
    draw_stack(
        17,
        3,
        &g.spread.tr,
        g.selected.contains(&Position(RowId::Top, ColumnId::Right)),
    );
    draw_stack(
        3,
        6,
        &g.spread.ml,
        g.selected
            .contains(&Position(RowId::Middle, ColumnId::Left)),
    );
    draw_stack(
        10,
        6,
        &g.spread.mc,
        g.selected
            .contains(&Position(RowId::Middle, ColumnId::Center)),
    );
    draw_stack(
        17,
        6,
        &g.spread.mr,
        g.selected
            .contains(&Position(RowId::Middle, ColumnId::Right)),
    );
    draw_stack(
        3,
        9,
        &g.spread.bl,
        g.selected
            .contains(&Position(RowId::Bottom, ColumnId::Left)),
    );
    draw_stack(
        10,
        9,
        &g.spread.bc,
        g.selected
            .contains(&Position(RowId::Bottom, ColumnId::Center)),
    );
    draw_stack(
        17,
        9,
        &g.spread.br,
        g.selected
            .contains(&Position(RowId::Bottom, ColumnId::Right)),
    );

    draw_move(g.selected_move());
    draw_bonus(g.bonus_card);
    draw_trashes(&g.trashes);
    draw_remaining(&g.remaining_cards());
    draw_help_msg();
}
