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

fn draw_stack(x: i32, y: i32, stack: &CardStack, selected: bool) {
    if selected {
        terminal::set_background(Color::from_rgb(100, 100, 100));
    }
    match stack.last() {
        None => terminal::print_xy(x, y, &format!("[color=gray]{}|{}{}", 0, " ", " ")),
        Some(Card(rank, suit)) => terminal::print_xy(
            x,
            y,
            &format!("[color=gray]{}|[color=white]{}{}", stack.len(), rank, suit,),
        ),
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

fn draw_bonus(Card(r, s): Card) {
    let msg = format!("Bonus suit: {}{}", r, s);
    terminal::print_xy(3, 18, &msg);
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
    const TOP: i32 = 6;
    const LEFT: i32 = 33;
    terminal::print_xy(LEFT, TOP, "   ♣♦♥♠");
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
        terminal::print_xy(LEFT, TOP + idx as i32, &row);
        idx += 1;
    }
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
}
