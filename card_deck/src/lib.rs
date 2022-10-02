use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        let rng = rand::thread_rng().gen_range(1, 4);
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

    pub fn get_suit(&self) -> &Suit {
        return self;
    }
}

impl Rank {
    pub fn random() -> Rank {
        let which_rank = rand::thread_rng().gen_range(1, 4);
        match which_rank {
            1 => Rank::Ace,
            2 => Rank::King,
            3 => Rank::Queen,
            _ => Rank::Jack
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Jack
        }
    }

    pub fn get_rank(&self) -> &Rank {
        return self;
    }
}

pub fn winner_card(card: &Card) -> bool {
    if card.rank.get_rank() == &Rank::Ace && card.suit.get_suit() == &Suit::Spade {
        return true;
    }
    return false;
}