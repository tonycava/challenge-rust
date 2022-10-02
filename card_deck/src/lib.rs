use std::ops::Range;
use rand::Rng;

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Ace(u8),
    King(u8),
    Queen(u8),
    Jack(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng().gen_range(1, 4);
        println!("{} rng", rng);
        match rng {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club
        }
    }

    pub fn get_suit(self) -> Suit {
        return self;
    }
}

impl Rank {
    pub fn random() -> Rank {
        // let which_rank: u8 = Range::new(1, 4);
        // let which_number: u8 = Range::new(0, 10);
        let mut which_rank = rand::thread_rng().gen_range(1, 4);
        let mut which_number = rand::thread_rng().gen_range(2, 10);

        match which_rank {
            1 => Rank::Ace(which_number),
            2 => Rank::King(which_number),
            3 => Rank::Queen(which_number),
            _ => Rank::Jack(which_number)
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace(1),
            12 => Rank::Queen(1),
            13 => Rank::King(1),
            _ => Rank::Jack(1)
        }
    }

    pub fn get_rank(self) -> Rank {
        return self;
    }
}

pub fn winner_card(card: Card) -> bool {
    if (card.rank.get_rank() == Rank::Ace(2)
        || card.rank.get_rank() == Rank::Ace(3)
        || card.rank.get_rank() == Rank::Ace(4)
        || card.rank.get_rank() == Rank::Ace(5)
        || card.rank.get_rank() == Rank::Ace(6)
        || card.rank.get_rank() == Rank::Ace(7)
        || card.rank.get_rank() == Rank::Ace(8)
        || card.rank.get_rank() == Rank::Ace(9)
        || card.rank.get_rank() == Rank::Ace(10))

        && card.suit.get_suit() == Suit::Spade {
        return true;
    }
    return false;
}