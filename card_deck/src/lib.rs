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
    Ace,
    King,
    Queen,
    Jack,
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

    pub fn get_suit(&self) -> &Suit {
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

pub fn winner_card(card: Card) -> bool {
    if card.rank.get_rank() == &Rank::Ace && card.suit.get_suit() == &Suit::Spade {
        return true;
    }
    return false;
}