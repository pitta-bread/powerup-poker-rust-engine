use rand::{rng, seq::SliceRandom};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
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
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUp {
    IncreaseRank,
    DecreaseRank,
    AutoAce,
    PocketPair,
    ReDeal,
}

impl PowerUp {
    pub fn help_text(&self) -> &'static str {
        match self {
            PowerUp::IncreaseRank => "Increase the value of a card by +1. King goes to Ace.",
            PowerUp::DecreaseRank => "Decrease the value of a card by -1. Ace goes to King.",
            PowerUp::AutoAce => {
                "Auto-ace. Swap one hole card of your choice with a random Ace \
                remaining in the deck."
            }
            PowerUp::PocketPair => {
                "Pocket pair. Choose a hole card. \
                Swap the other card with a card from the \
                deck with the same value as the chosen card (but by necessity, \
                different suit, randomly found)."
            }
            PowerUp::ReDeal => {
                "Re-deal. Discard your current hole cards and draw \
                2 new ones from the top of the deck."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayingCard {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck<T> {
    pub cards: Vec<T>,
}

impl<T> Deck<T> {
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<T> {
        self.cards.pop()
    }
}

impl Deck<PlayingCard> {
    pub fn new_52_card_deck() -> Self {
        // shuffles too
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(PlayingCard { rank, suit });
            }
        }
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }
}

impl Deck<PowerUp> {
    pub fn new_deck(
        increase_rank_count: usize,
        decrease_rank_count: usize,
        auto_ace_count: usize,
        pocket_pair_count: usize,
        re_deal_count: usize,
    ) -> Self {
        // shuffles too
        let mut cards = Vec::new();
        cards.extend((0..increase_rank_count).map(|_| PowerUp::IncreaseRank));
        cards.extend((0..decrease_rank_count).map(|_| PowerUp::DecreaseRank));
        cards.extend((0..auto_ace_count).map(|_| PowerUp::AutoAce));
        cards.extend((0..pocket_pair_count).map(|_| PowerUp::PocketPair));
        cards.extend((0..re_deal_count).map(|_| PowerUp::ReDeal));
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }
}

pub struct Hand {
    pub playing_cards: [PlayingCard; 2],
    pub power_up: Option<PowerUp>,
}

pub struct Player {
    pub name: String,
    pub chips: usize,
    pub hand: Option<Hand>,
}

pub struct GameState {
    pub players: [Player; 2],
    pub card_deck: Deck<PlayingCard>,
    pub power_up_deck: Deck<PowerUp>,
}
