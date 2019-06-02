use std::collections::HashSet;
use std::fmt;

use bear_lib_terminal::{terminal, Color};

use crate::model::*;


const DATA_LEFT: i32 = 33;

// ---------------------------------------------
// Format data

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

// ---------------------------------------------
// Draw feedback column

fn draw_score(score: u32) {
    let scr_msg = format!("Score:{: >10}", score);
    terminal::print_xy(DATA_LEFT, 3, &scr_msg);
}

fn draw_trashes(t: &Trashes) {
    let msg = match &t {
        Trashes::None => "",
        Trashes::One => "•",
        Trashes::Two => "••",
    };

    terminal::print_xy(DATA_LEFT, 5, &format!("Trashes:      {: >2}", msg));
}

fn draw_bonus(c: Card) {
    let msg = format!("Bonus suit:  {}", c);
    terminal::print_xy(DATA_LEFT, 7, &msg);
}


fn draw_remaining(rem_cards: &HashSet<Card>) {
    const TOP: i32 = 9;
    terminal::print_xy(DATA_LEFT, TOP, "Remaining Cards:");
    terminal::print_xy(DATA_LEFT, TOP + 2, "   ♣♦♥♠");
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
        terminal::print_xy(DATA_LEFT, TOP + 2 + idx as i32, &row);
        idx += 1;
    }
}

fn draw_stack(Position(rowid, colid): Position, stack: &CardStack, selected: bool) {
    let y = match rowid {
        RowId::Top => 3,
        RowId::Middle => 6,
        RowId::Bottom => 9,
    };
    let x = match colid {
        ColumnId::Left => 3,
        ColumnId::Center => 10,
        ColumnId::Right => 17,
    };
    match stack.last() {
        None => {
            let bonus = rowid.bonus();
            let msg = format!("[color=45,45,45]0|{: >+3}", bonus);
            terminal::print_xy(x, y, &msg);
        }
        Some(card) => {
            if selected {
                terminal::set_background(Color::from_rgb(70, 70, 70));
            } else {
                terminal::set_background(Color::from_rgb(40, 40, 40));
            }
            terminal::print_xy(x, y, &format!("[color=0,0,0]{}|{}", stack.len(), card));
        }
    }
    //reset default bg color
    terminal::set_background(Color::from_rgb(0, 0, 0));
}

fn draw_move(mv_opt: Option<Move>) {
    if let Some(mv) = mv_opt {
        let msg = match mv {
            Move::Trash(_) => "Trash".to_owned(),
            Move::PlayHand(h) => format!("{} ({} pts)", h, h.points()),
        };
        terminal::print_xy(3, 15, &msg);
    }
}

fn draw_help_msg() {
    terminal::print_xy(3, 24, "Press 'H' for help");
}

pub fn draw_game(g: &Game) {
    for rowid in &[RowId::Top, RowId::Middle, RowId::Bottom] {
        for colid in &[ColumnId::Left, ColumnId::Center, ColumnId::Right] {
            let pos = Position(*rowid, *colid);
            let stack = g.spread.get_stack(pos);
            let selected = g.selected.contains(&pos);
            draw_stack(pos, stack, selected);
        }
    }
    draw_move(g.selected_move());
    draw_help_msg();
    draw_score(g.score());
    draw_trashes(&g.trashes);
    draw_bonus(g.bonus_card);
    draw_remaining(&g.remaining_cards());
}
